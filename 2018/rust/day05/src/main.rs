use std::mem;

use anyhow::{Ok, Result};

fn main() -> Result<()> {
    let input = include_str!("../input/input.txt");

    part1_1(input);
    part2_1(input);

    part1_2(input)?;
    part2_2(input)?;

    Ok(())
}

// solution 1: https://github.com/birkenfeld/advent18/blob/master/src/bin/day05.rs

fn reacts1(a: char, b: char) -> bool {
    a != b && a.eq_ignore_ascii_case(&b)
}

fn reduce(polymer: &str, skip: Option<char>) -> usize {
    // One pass over the input is enough, if we always keep track if
    // last pushed and the new unit react
    polymer
        .chars()
        .fold(vec![], |mut stack, ch| match stack.last() {
            _ if skip == Some(ch.to_ascii_lowercase()) => stack,
            Some(&pch) if reacts1(pch, ch) => {
                stack.pop();
                stack
            }
            _ => {
                stack.push(ch);
                stack
            }
        })
        .len()
}

fn part1_1(input: &str) {
    println!("part1: {}", reduce(input, None));
}

fn part2_1(input: &str) {
    let min_len = (b'a'..=b'z')
        .map(|c| reduce(input, Some(c as char)))
        .min()
        .unwrap();
    println!("part2: {}", min_len);
}

// solution 2

fn part1_2(input: &str) -> Result<()> {
    println!("part1: {}", react(input).len());
    Ok(())
}

fn part2_2(input: &str) -> Result<()> {
    let mut best = input.len();
    for b in b'A'..=b'Z' {
        let unit1 = b as char;
        let unit2 = (b + 32) as char;
        let cleansed = input.replace(unit1, "").replace(unit2, "");
        let reacted = react(&cleansed);
        if reacted.len() < best {
            best = reacted.len();
        }
    }
    println!("part2: {}", best);
    Ok(())
}

fn react(input: &str) -> String {
    let mut polymer = input.as_bytes().to_vec();
    let mut reacted_polymer = vec![];
    loop {
        let mut reacted = false;
        let mut i = 1;
        while i < polymer.len() {
            if reacts(polymer[i - 1], polymer[i]) {
                reacted = true;
                i += 2;
                continue;
            }
            reacted_polymer.push(polymer[i - 1]);
            i += 1;
        }

        if i == polymer.len() {
            reacted_polymer.push(polymer[i - 1]);
        }

        mem::swap(&mut polymer, &mut reacted_polymer);
        reacted_polymer.clear();
        if !reacted {
            break;
        }
    }
    // We only ever remove ASCII bytes, which is guaranteed to
    // preserve the UTF-8 validity for `polymer`.
    String::from_utf8(polymer).unwrap()
}

fn reacts(b1: u8, b2: u8) -> bool {
    if b1 < b2 {
        b2 - b1 == 32
    } else {
        b1 - b2 == 32
    }
}
