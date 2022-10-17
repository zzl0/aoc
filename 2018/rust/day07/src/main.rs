#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};
use itertools::Itertools;
use petgraph::prelude::*;
use regex::Regex;

macro_rules! err {
    ($($tt:tt)*) => {
        Err(anyhow!(format!($($tt)*)))
    };
}

fn main() -> Result<()> {
    let input = include_str!("../input/input.txt");

    solution1(input)?;
    solution2(input)?;

    Ok(())
}

// solution 1

fn solution1(input: &str) -> Result<()> {
    let deps: Vec<Dependency> = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Dependency>>>()?;

    let mut required_for: RequiredFor = HashMap::new();
    for dep in deps {
        required_for
            .entry(dep.step)
            .or_default()
            .insert(dep.required);
        required_for.entry(dep.required).or_default();
    }

    part1_1(&required_for)?;
    part2_1(&required_for)?;

    Ok(())
}

fn part1_1(required_for: &RequiredFor) -> Result<()> {
    let mut taken: HashSet<Step> = HashSet::new();
    let mut order: String = String::new();
    let mut next: Vec<Step> = vec![];

    loop {
        find_next_steps(required_for, &taken, &taken, &mut next);
        let next_step = match next.pop() {
            None => break,
            Some(next_step) => next_step,
        };
        taken.insert(next_step);
        order.push(next_step);
    }

    println!("part1: {}", order);

    Ok(())
}

fn part2_1(required_for: &RequiredFor) -> Result<()> {
    let mut workers = Workers::new(5);
    let mut assigned: HashSet<Step> = HashSet::new();
    let mut done: HashSet<Step> = HashSet::new();
    let mut order: String = String::new();
    let mut next: Vec<Step> = vec![];

    let mut seconds = 0;
    loop {
        workers.run_one_step(&mut order, &mut done);
        find_next_steps(required_for, &assigned, &done, &mut next);
        if next.is_empty() && workers.all_idle() {
            break;
        }
        for worker in workers.available() {
            if let Some(next_step) = next.pop() {
                assigned.insert(next_step);
                workers.work_on(worker, next_step);
            } else {
                break;
            }
        }
        seconds += 1;
    }

    println!("part2: {}", seconds);

    Ok(())
}

#[derive(Debug)]
struct Workers {
    status: Vec<Status>,
}

type WorkerID = usize;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Status {
    Idle,
    Working { step: Step, remaining: u8 },
}

impl Workers {
    fn new(count: usize) -> Workers {
        Workers {
            status: vec![Status::Idle; count],
        }
    }

    fn available(&self) -> Vec<WorkerID> {
        self.status
            .iter()
            .enumerate()
            .filter(|(_, &status)| status == Status::Idle)
            .map(|(i, _)| i)
            .collect()
    }

    fn all_idle(&self) -> bool {
        self.status.iter().all(|s| *s == Status::Idle)
    }

    fn work_on(&mut self, worker: WorkerID, step: Step) {
        let status = &mut self.status[worker];
        assert!(
            *status == Status::Idle,
            "worker {} is not available",
            worker
        );

        let remaining = (step as u8) - b'A' + 61;
        *status = Status::Working { step, remaining }
    }

    fn run_one_step(&mut self, order: &mut String, done: &mut HashSet<Step>) {
        for worker in 0..self.status.len() {
            let mut is_done = false;
            match self.status[worker] {
                Status::Working {
                    step,
                    ref mut remaining,
                } => {
                    *remaining -= 1;
                    if *remaining == 0 {
                        is_done = true;
                        order.push(step);
                        done.insert(step);
                    }
                }
                Status::Idle => {}
            }
            if is_done {
                self.status[worker] = Status::Idle;
            }
        }
    }
}

fn find_next_steps(
    required_for: &RequiredFor,
    taken: &HashSet<Step>,
    done: &HashSet<Step>,
    next_stack: &mut Vec<Step>,
) {
    for (&step, dependencies) in required_for {
        if taken.contains(&step) {
            continue;
        }
        if dependencies.iter().all(|s| done.contains(s)) {
            next_stack.push(step);
        }
    }
    next_stack.sort();
    next_stack.dedup();
    next_stack.reverse();
}

type RequiredFor = HashMap<Step, HashSet<Step>>;

// solution 2

fn solution2(input: &str) -> Result<()> {
    let mut graph = StableGraph::new();
    let mut nodes = HashMap::new();

    for line in input.lines() {
        let dep: Dependency = line
            .parse()
            .or_else(|err| err!("failed to parse '{:?}': {}", line, err))?;
        let (b, a) = (dep.step, dep.required);
        let na = *nodes.entry(a).or_insert_with(|| graph.add_node(a));
        let nb = *nodes.entry(b).or_insert_with(|| graph.add_node(b));
        graph.add_edge(na, nb, ());
    }

    let mut graph2 = graph.map(
        |_, n| Task {
            id: *n,
            worker: None,
            time_left: *n as u8 - b'A' + 61,
        },
        |_, _| (),
    );

    part1_2(&mut graph)?;
    part2_2(&mut graph2)?;
    Ok(())
}

fn part1_2(graph: &mut StableGraph<char, ()>) -> Result<()> {
    let mut order = String::new();
    while graph.node_count() > 0 {
        let root = graph
            .externals(Incoming)
            .sorted_by_key(|&n| graph[n])
            .next()
            .unwrap();
        order.push(graph.remove_node(root).unwrap());
    }
    println!("part1: {}", order);
    Ok(())
}

fn part2_2(graph: &mut StableGraph<Task, ()>) -> Result<()> {
    let mut clock = -1;
    let mut free_workers = vec![1, 2, 3, 4, 5];
    while graph.node_count() > 0 {
        clock += 1;

        // first, check for tasks that are done
        for node in graph.externals(Incoming).collect_vec() {
            if let Some(worker) = graph[node].worker {
                graph[node].time_left -= 1;
                if graph[node].time_left == 0 {
                    free_workers.push(worker);
                    graph.remove_node(node);
                }
            }
        }

        // second, assign workers to any available tasks (some may
        // have become available due to pruning).
        for node in graph.externals(Incoming).sorted_by_key(|&n| graph[n].id) {
            if graph[node].worker.is_none() {
                graph[node].worker = free_workers.pop();
            }
        }
    }

    println!("part2: {}", clock);
    Ok(())
}

struct Task {
    id: char,
    worker: Option<u8>,
    time_left: u8,
}

type Step = char;

#[derive(Debug, Clone, Copy)]
struct Dependency {
    step: Step,
    required: Step,
}

impl FromStr for Dependency {
    type Err = Error;

    fn from_str(s: &str) -> Result<Dependency> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin.")
                    .unwrap();
        }
        let caps = match RE.captures(s) {
            None => return err!("unrecognized dependency"),
            Some(caps) => caps,
        };

        Ok(Dependency {
            step: caps[2].as_bytes()[0] as Step,
            required: caps[1].as_bytes()[0] as Step,
        })
    }
}
