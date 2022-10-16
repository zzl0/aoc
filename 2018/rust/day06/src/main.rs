use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use anyhow::{anyhow, Error, Ok, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let input = include_str!("../input/input.txt");

    let coordinates = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Coordinate>>>()?;
    if coordinates.is_empty() {
        return Err(anyhow!("no coordinates given"));
    }

    // solution 1
    part1_1(&coordinates)?;
    part2_1(&coordinates)?;

    // solution 2

    let mut grid = Grid::new(coordinates);
    grid.find_finite();
    part1_2(&grid)?;
    part2_2(&grid)?;

    Ok(())
}

// solution 1

fn part1_1(points: &[Coordinate]) -> Result<()> {
    let max_x = points.iter().map(|c| c.x).max().unwrap();
    let min_x = points.iter().map(|c| c.x).min().unwrap();
    let max_y = points.iter().map(|c| c.y).max().unwrap();
    let min_y = points.iter().map(|c| c.y).min().unwrap();

    let mut area_sizes = vec![0; points.len()];

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let dists: Vec<i32> = points
                .iter()
                .map(|p| p.distance(Coordinate { x, y }))
                .collect();
            let min_dist = dists.iter().min().unwrap();
            if let Some(((i, _),)) = dists
                .iter()
                .enumerate()
                .filter(|j| j.1 == min_dist)
                .collect_tuple()
            {
                if x == min_x || x == max_x || y == min_y || y == max_y {
                    area_sizes[i] = i32::min_value();
                } else {
                    area_sizes[i] += 1;
                }
            }
        }
    }

    let max_area_size = area_sizes.iter().max().unwrap();
    println!("part1: {}", max_area_size);

    Ok(())
}

fn part2_1(points: &[Coordinate]) -> Result<()> {
    let max_x = points.iter().map(|c| c.x).max().unwrap();
    let min_x = points.iter().map(|c| c.x).min().unwrap();
    let max_y = points.iter().map(|c| c.y).max().unwrap();
    let min_y = points.iter().map(|c| c.y).min().unwrap();

    let region_size = (min_x..=max_x)
        .cartesian_product(min_y..max_y)
        .map(|p| {
            points
                .iter()
                .map(|pc| pc.distance(Coordinate { x: p.0, y: p.1 }))
                .sum::<i32>()
        })
        .filter(|&s| s < 10000)
        .count();
    println!("part2: {}", region_size);
    Ok(())
}

// solution 2

fn part1_2(grid: &Grid) -> Result<()> {
    let mut biggest_area = 0;
    for &loc in &grid.finite {
        let mut candidate_area = 0;
        for &loc2 in grid.table.values() {
            if loc == loc2 {
                candidate_area += 1;
            }
        }
        if candidate_area > biggest_area {
            biggest_area = candidate_area;
        }
    }
    println!("part1: {}", biggest_area);
    Ok(())
}

fn part2_2(grid: &Grid) -> Result<()> {
    let bound = 500;
    let mut size = 0;
    for x in -bound..=bound {
        for y in -bound..=bound {
            if grid.distance_sum(Coordinate { x, y }) < 10000 {
                size += 1
            }
        }
    }
    println!("part2: {}", size);
    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn distance(self, other: Coordinate) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn border(self, step: i32) -> impl Iterator<Item = Coordinate> {
        (self.x - step..=self.x + step)
            .flat_map(move |x| (self.y - step..=self.y + step).map(move |y| Coordinate { x, y }))
            .filter(move |&c2| self.distance(c2) == step)
    }
}

impl FromStr for Coordinate {
    type Err = Error;

    fn from_str(s: &str) -> Result<Coordinate> {
        let comma = match s.find(',') {
            None => return Err(anyhow!("could not find comma")),
            Some(i) => i,
        };
        let (pos1, pos2) = (s[..comma].trim(), s[comma + 1..].trim());
        Ok(Coordinate {
            x: pos1.parse()?,
            y: pos2.parse()?,
        })
    }
}

#[derive(Debug)]
struct Grid {
    // all coordinates given in the input
    locations: Vec<Coordinate>,
    // all know finite coordinates
    finite: HashSet<Coordinate>,
    // a map from an arbitrary coordiate to its closet location
    table: HashMap<Coordinate, Coordinate>,
}

impl Grid {
    fn new(locations: Vec<Coordinate>) -> Grid {
        Grid {
            locations,
            finite: HashSet::new(),
            table: HashMap::new(),
        }
    }

    fn find_finite(&mut self) {
        for step in 0..100 {
            for loc in &self.locations {
                if self.finite.contains(&loc) {
                    continue;
                }
                for c in loc.border(step) {
                    let closest = match self.closest_location(c) {
                        None => continue,
                        Some(closest) => closest,
                    };
                    self.table.insert(c, closest);
                }
            }
            for &loc in &self.locations {
                if loc.border(step).all(|c| self.table.get(&c) != Some(&loc)) {
                    self.finite.insert(loc);
                }
            }
        }
    }

    fn distance_sum(&self, c: Coordinate) -> i32 {
        self.locations.iter().map(|&loc| loc.distance(c)).sum()
    }

    fn closest_location(&self, c: Coordinate) -> Option<Coordinate> {
        let (mut min, mut uniq) = (self.locations[0], true);
        for &loc in &self.locations[1..] {
            if loc.distance(c) == min.distance(c) {
                uniq = false;
            } else if loc.distance(c) < min.distance(c) {
                min = loc;
                uniq = true;
            }
        }
        if uniq {
            Some(min)
        } else {
            None
        }
    }
}
