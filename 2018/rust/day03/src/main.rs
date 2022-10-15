#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};
use regex::Regex;

macro_rules! err {
    // $( ... )* Match 0 or more times with no separator
    // $tt is a fragment specifier
    // tt is a token tree
    ($($tt:tt)*) => { anyhow!(format!($($tt)*)) }
}

// Maps a point to the count of overlapping claims corresponding to that point.
type Grid = HashMap<(u32, u32), u32>;

#[derive(Debug)]
struct Claim {
    id: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl Claim {
    fn iter_points(&self) -> IterPoints {
        IterPoints {
            claim: self,
            px: self.x,
            py: self.y,
        }
    }
}

struct IterPoints<'c> {
    claim: &'c Claim,
    px: u32,
    py: u32,
}

impl<'c> Iterator for IterPoints<'c> {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<(u32, u32)> {
        if self.py >= self.claim.y + self.claim.height {
            self.py = self.claim.y;
            self.px += 1;
        }
        if self.px >= self.claim.x + self.claim.width {
            return None;
        }
        let (px, py) = (self.px, self.py);
        self.py += 1;
        Some((px, py))
    }
}

impl FromStr for Claim {
    type Err = Error;

    fn from_str(s: &str) -> Result<Claim> {
        // Regex::new constructor can be expensive, the lazy_static provides a nice
        // way to construct static values lazily the first time they are used.
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?x)
                \#
                (?P<id>[0-9]+)
                \s+@\s+
                (?P<x>[0-9]+),(?P<y>[0-9]+):
                \s+
                (?P<width>[0-9]+)x(?P<height>[0-9]+)
                "
            )
            .unwrap();
        }

        let caps = RE
            .captures(s)
            .ok_or_else(|| anyhow!("unrecognized claim"))?;
        Ok(Claim {
            id: caps["id"].parse()?,
            x: caps["x"].parse()?,
            y: caps["y"].parse()?,
            width: caps["width"].parse()?,
            height: caps["height"].parse()?,
        })
    }
}

fn main() -> Result<()> {
    let input = include_str!("../input/input.txt");

    let mut claims: Vec<Claim> = vec![];
    for line in input.lines() {
        let claim = line
            .parse() // FromStr
            .map_err(|err| err!("failed to parse '{:?}': {}", line, err))?;
        claims.push(claim);
    }

    let mut grid = Grid::new();
    for claim in &claims {
        for (x, y) in claim.iter_points() {
            *grid.entry((x, y)).or_default() += 1
        }
    }
    part1(&grid)?;
    part2(&claims, &grid)?;

    Ok(())
}

fn part1(grid: &Grid) -> Result<()> {
    let count = grid.values().filter(|&&count| count > 1).count();
    println!("contested points: {}", count);
    Ok(())
}

fn part2(claims: &[Claim], grid: &Grid) -> Result<()> {
    for claim in claims {
        if claim.iter_points().all(|p| grid[&p] == 1) {
            println!("uncontested claim: {}", claim.id);
            return Ok(());
        }
    }
    Err(err!("no uncontested claims"))
}
