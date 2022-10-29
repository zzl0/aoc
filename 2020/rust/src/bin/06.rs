use im::HashSet;
use std::fmt;

pub struct Answers(HashSet<u8>);

impl fmt::Debug for Answers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for &answer in &self.0 {
            write!(f, "{}", answer as char)?;
        }
        Ok(())
    }
}

fn part1(input: &str) {
    let answer: usize = input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .flat_map(|line| line.as_bytes().iter().copied())
                .collect::<HashSet<u8>>()
                .len()
        })
        .sum();

    println!("part1: {}", answer);
}

fn main() {
    let input = include_str!("../../../data/day06.txt");
    part1(input);
}
