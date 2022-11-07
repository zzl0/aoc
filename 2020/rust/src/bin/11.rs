use std::fmt;
use std::iter::Extend;

use im::Vector;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2 {
    x: i64,
    y: i64,
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

impl Default for Tile {
    fn default() -> Self {
        Self::Floor
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Tile::Floor => '.',
            Tile::EmptySeat => 'L',
            Tile::OccupiedSeat => '#',
        };
        write!(f, "{}", c)
    }
}

impl Tile {
    fn next<I>(self, neighbors: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        match self {
            Self::Floor => Self::Floor,
            Self::EmptySeat => match neighbors
                .filter(|t| matches!(t, Self::OccupiedSeat))
                .count()
            {
                0 => Self::OccupiedSeat,
                _ => Self::EmptySeat,
            },
            Self::OccupiedSeat => {
                match neighbors
                    .filter(|t| matches!(t, Self::OccupiedSeat))
                    .count()
                {
                    0..=3 => Self::OccupiedSeat,
                    _ => Self::EmptySeat,
                }
            }
        }
    }
}

#[derive(PartialEq, Clone)]
struct Map<T>
where
    T: Clone,
{
    size: Vec2,
    tiles: Vector<T>,
}

impl<T> Map<T>
where
    T: Default + Clone,
{
    fn new(size: Vec2) -> Self {
        let num_tiles = size.x * size.y;
        Self {
            size,
            tiles: (0..num_tiles)
                .into_iter()
                .map(|_| Default::default())
                .collect(),
        }
    }
}

impl<T> Map<T>
where
    T: Clone,
{
    fn index(&self, pos: Vec2) -> Option<usize> {
        if (0..self.size.x).contains(&pos.x) && (0..self.size.y).contains(&pos.y) {
            Some((pos.x + pos.y * self.size.x) as _)
        } else {
            None
        }
    }

    fn set(&mut self, pos: Vec2, tile: T) {
        if let Some(index) = self.index(pos) {
            self.tiles[index] = tile;
        }
    }

    fn neighbor_positions(&self, pos: Vec2) -> impl Iterator<Item = Vec2> {
        (-1..=1)
            .flat_map(|dx| (-1..=1).map(move |dy| (dx, dy)))
            .filter(|&(dx, dy)| !(dx == 0 && dy == 0))
            .map(move |(dx, dy)| Vec2 {
                x: pos.x + dx,
                y: pos.y + dy,
            })
    }
}

#[derive(Debug)]
struct Positioned<T>(Vec2, T);

impl<T> Map<T>
where
    T: Copy,
{
    fn get(&self, pos: Vec2) -> Option<T> {
        self.index(pos).map(|index| self.tiles[index])
    }

    fn neighbor_tiles(&self, pos: Vec2) -> impl Iterator<Item = T> + '_ {
        self.neighbor_positions(pos)
            .filter_map(move |pos| self.get(pos))
    }

    fn iter(&self) -> impl Iterator<Item = Positioned<T>> + '_ {
        (0..self.size.y).flat_map(move |y| {
            (0..self.size.x).map(move |x| {
                let pos = Vec2 { x, y };
                Positioned(pos, self.get(pos).unwrap())
            })
        })
    }
}

impl<T> fmt::Debug for Map<T>
where
    T: fmt::Debug + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                write!(f, "{:?}", self.get(Vec2 { x, y }).unwrap())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<A> Extend<Positioned<A>> for Map<A>
where
    A: Clone,
{
    fn extend<T: IntoIterator<Item = Positioned<A>>>(&mut self, iter: T) {
        for Positioned(pos, tile) in iter {
            self.set(pos, tile)
        }
    }
}

impl Map<Tile> {
    fn parse(input: &[u8]) -> Self {
        let input = input.strip_suffix(&[b'\n']).unwrap();
        let mut columns = 0;
        let mut rows = 1;
        for &c in input.iter() {
            if c == b'\n' {
                rows += 1;
                columns = 0;
            } else {
                columns += 1;
            }
        }

        let mut iter = input.iter().copied();
        let mut map = Self::new(Vec2 {
            x: columns,
            y: rows,
        });
        for row in 0..map.size.y {
            for col in 0..map.size.x {
                let tile = match iter.next() {
                    Some(b'.') => Tile::Floor,
                    Some(b'L') => Tile::EmptySeat,
                    Some(b'#') => Tile::OccupiedSeat,
                    c => panic!("Expected '.', 'L' or '#', but got: {:?}", c),
                };
                map.set(Vec2 { x: col, y: row }, tile);
            }
            iter.next();
        }
        map
    }

    fn next(&self) -> Self {
        let mut res = Self::new(self.size);
        res.extend(
            self.iter()
                .map(|Positioned(pos, tile)| Positioned(pos, tile.next(self.neighbor_tiles(pos)))),
        );
        res
    }

    fn last(self) -> Self {
        itertools::iterate(self, Map::next)
            .tuple_windows()
            .find_map(|(prev, next)| if prev == next { Some(next) } else { None })
            .unwrap()
    }
}

fn part1(map: Map<Tile>) {
    let last = map.last();
    let answer = last
        .iter()
        .filter(|p| matches!(p.1, Tile::OccupiedSeat))
        .count();
    println!("part1: {}", answer);
}

fn main() {
    let input = include_bytes!("../../../data/day11.txt");
    let map = Map::<Tile>::parse(input);

    part1(map);
}

#[test]
fn test_neighbor_positions() {
    use std::collections::HashSet;

    let map = Map::<()>::new(Vec2 { x: 3, y: 3 });
    let positions: HashSet<_> = map
        .neighbor_positions(Vec2 { x: 1, y: 1 })
        .map(|v| (v.x, v.y))
        .collect();
    for p in &[
        (0, 0),
        (0, 1),
        (0, 2),
        (1, 0),
        (2, 0),
        (1, 2),
        (2, 2),
        (2, 1),
    ] {
        assert!(positions.contains(p));
    }
}
