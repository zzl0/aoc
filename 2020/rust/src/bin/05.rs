use bitvec::prelude::*;

#[derive(Default, Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord)]
struct Seat(u16);

impl Seat {
    fn parse(input: &str) -> Self {
        let mut res: Seat = Default::default();

        let bits = BitSlice::<_, Lsb0>::from_element_mut(&mut res.0);
        for (i, &b) in input.as_bytes().iter().rev().enumerate() {
            bits.set(
                i,
                match b {
                    b'F' | b'L' => false,
                    b'B' | b'R' => true,
                    _ => panic!("unexpected letter: {}", b as char),
                },
            )
        }
        res
    }
}

fn part1(input: &str) {
    let max_id = input
        .lines()
        .map(Seat::parse)
        .map(|seat| seat.0)
        .max()
        .unwrap();
    println!("part1: {}", max_id);
}

fn part2(input: &str) {
    let mut ids: Vec<_> = input.lines().map(Seat::parse).collect();
    ids.sort();

    let mut prev_id: Option<Seat> = None;
    for id in ids {
        if let Some(prev_id) = prev_id {
            if id.0 - prev_id.0 > 1 {
                println!("part2: {}", prev_id.0 + 1);
                return;
            }
        }
        prev_id = Some(id);
    }
}

fn main() {
    let input = include_str!("../../../data/day05.txt");
    part1(input);
    part2(input);
}

#[test]
fn test_05_seat_id() {
    assert_eq!(Seat::parse("BFFFBBFRRR"), Seat(567));
    assert_eq!(Seat::parse("FFFBBBFRRR"), Seat(119));
    assert_eq!(Seat::parse("BBFFBBFRLL"), Seat(820));
}
