// The macro_use attribute can also appear on extern crate.
// In this context it controls which macros are loaded from
// the external crate
#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::slice;
use std::str::FromStr;
use std::{collections::HashMap, ops::Range};

use anyhow::{anyhow, Error, Result};
use regex::Regex;

macro_rules! err {
    ($($tt:tt)*) => {
        anyhow!(format!($($tt)*))
    };
}

type GuardID = u32;
type EventsByGuard = HashMap<GuardID, Vec<Event>>;
type GuardSleepFrequency = HashMap<GuardID, [u32; 60]>;

#[derive(Debug)]
struct Event {
    datetime: DateTime,
    kind: EventKind,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct DateTime {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

#[derive(Debug)]
enum EventKind {
    StartShift { guard_id: GuardID },
    Asleep,
    WakeUp,
}

fn main() -> Result<()> {
    let input = include_str!("../input/input.txt");

    // collect events
    let mut events: Vec<Event> = vec![];
    for line in input.lines() {
        let event = line
            .parse()
            .map_err(|err| err!("failed to parse '{:?}': {}", line, err))?;
        events.push(event);
    }
    if events.is_empty() {
        return Err(err!("found no events"));
    }

    // sort them by time and group them by guard
    events.sort_by(|ev1, ev2| ev1.datetime.cmp(&ev2.datetime));
    let mut events_by_guard = EventsByGuard::new();
    let mut curr_guard_id = None;
    for ev in events {
        if let EventKind::StartShift { guard_id } = ev.kind {
            curr_guard_id = Some(guard_id);
        }
        match curr_guard_id {
            None => return Err(err!("no guard id set for event")),
            Some(id) => {
                events_by_guard.entry(id).or_default().push(ev);
            }
        }
    }

    // create a by-minute frequency map for each guard
    let mut minutes_asleep: GuardSleepFrequency = HashMap::new();
    for (&guard_id, events) in events_by_guard.iter() {
        let mut freq: [u32; 60] = [0; 60];
        for result in MinutesAsleepIter::new(events) {
            for minute in result? {
                freq[minute as usize] += 1;
            }
        }
        minutes_asleep.insert(guard_id, freq);
    }

    part1(&minutes_asleep)?;
    part2(&minutes_asleep)?;

    Ok(())
}

fn part1(minutes_asleep: &GuardSleepFrequency) -> Result<()> {
    let (&sleepiest, _) = minutes_asleep
        .iter()
        .max_by_key(|&(_, freqs)| -> u32 { freqs.iter().sum() })
        // unwrap is OK since we're guaranteed to have at least one event
        .unwrap();
    let minute = match sleepiest_minute(minutes_asleep, sleepiest) {
        None => return Err(err!("guard {} was never asleep", sleepiest)),
        Some(minute) => minute,
    };

    println!("part1, product: {}", sleepiest * minute);
    Ok(())
}

fn part2(minutes_asleep: &GuardSleepFrequency) -> Result<()> {
    let mut sleepiest_minutes: HashMap<GuardID, (u32, u32)> = HashMap::new();
    for (&guard_id, freqs) in minutes_asleep.iter() {
        let minute = match sleepiest_minute(minutes_asleep, guard_id) {
            None => continue,
            Some(minute) => minute,
        };
        let count = freqs[minute as usize];
        sleepiest_minutes.insert(guard_id, (minute, count));
    }
    if sleepiest_minutes.is_empty() {
        return Err(err!("no guards slept"));
    }
    let (&longest_asleep, &(minute, _)) = sleepiest_minutes
        .iter()
        .max_by_key(|&(_, (_, count))| count)
        .unwrap();

    println!("part 2, product: {}", longest_asleep * minute);
    Ok(())
}

fn sleepiest_minute(minutes_asleep: &GuardSleepFrequency, guard_id: GuardID) -> Option<u32> {
    let (sleepiest_minute, ..) = minutes_asleep[&guard_id]
        .iter()
        .enumerate()
        .max_by_key(|(_, freq)| -> u32 { **freq })
        .expect("Iterator of sleepy minutes should not be empty");
    Some(sleepiest_minute as u32)
}

impl FromStr for Event {
    type Err = Error;

    fn from_str(s: &str) -> Result<Event> {
        lazy_static! {
            // [1518-11-16 00:02] Guard #1433 begins shift
            static ref RE: Regex = Regex::new(
                r"(?x)
                \[
                    (?P<year>[0-9]{4})-(?P<month>[0-9]{2})-(?P<day>[0-9]{2})
                    \s+
                    (?P<hour>[0-9]{2}):(?P<minute>[0-9]{2})
                \]
                \s+
                (?:Guard\ \#(?P<id>[0-9]+)\ begins\ shift|(?P<sleep>.+))
                "
            ).unwrap();
        }

        let caps = match RE.captures(s) {
            None => return Err(err!("unrecognized event")),
            Some(caps) => caps,
        };
        let datetime = DateTime {
            year: caps["year"].parse()?,
            month: caps["month"].parse()?,
            day: caps["day"].parse()?,
            hour: caps["hour"].parse()?,
            minute: caps["minute"].parse()?,
        };
        let kind = if let Some(m) = caps.name("id") {
            EventKind::StartShift {
                guard_id: m.as_str().parse()?,
            }
        } else if &caps["sleep"] == "falls asleep" {
            EventKind::Asleep
        } else if &caps["sleep"] == "wakes up" {
            EventKind::WakeUp
        } else {
            return Err(err!("could not determine event kind"));
        };

        Ok(Event { datetime, kind })
    }
}

#[derive(Debug)]
struct MinutesAsleepIter<'a> {
    events: slice::Iter<'a, Event>,
    fell_asleep: Option<u32>,
}

impl<'a> MinutesAsleepIter<'a> {
    fn new(events: &'a [Event]) -> MinutesAsleepIter<'a> {
        MinutesAsleepIter {
            events: events.iter(),
            fell_asleep: None,
        }
    }
}

impl<'a> Iterator for MinutesAsleepIter<'a> {
    type Item = Result<Range<u32>>;

    fn next(&mut self) -> Option<Result<Range<u32>>> {
        loop {
            let ev = match self.events.next() {
                Some(ev) => ev,
                None => {
                    if self.fell_asleep.is_some() {
                        return Some(Err(err!("found sleep event without wake up")));
                    }
                    return None;
                }
            };
            match ev.kind {
                EventKind::StartShift { .. } => {}
                EventKind::Asleep => self.fell_asleep = Some(ev.datetime.minute),
                EventKind::WakeUp => {
                    let fell_asleep = match self.fell_asleep.take() {
                        Some(minute) => minute,
                        None => return Some(Err(err!("found wakeup without asleep"))),
                    };
                    if ev.datetime.minute < fell_asleep {
                        return Some(Err(err!("found wakeup before sleep")));
                    }
                    return Some(Ok(fell_asleep..ev.datetime.minute));
                }
            }
        }
    }
}
