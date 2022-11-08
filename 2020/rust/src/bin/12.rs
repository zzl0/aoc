use derive_more::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Add, Sub)]
struct Vec2 {
    x: isize,
    y: isize,
}

impl Vec2 {
    fn manhattan(self) -> usize {
        (self.x.abs() + self.y.abs()) as _
    }

    fn rotate(self, d: AngleDelta) -> Self {
        let Self { x, y } = self;
        match d.0.rem_euclid(4) {
            0 => Self { x, y },
            1 => Self { x: y, y: -x },
            2 => Self { x: -x, y: -y },
            3 => Self { x: -y, y: x },
            _ => unreachable!(),
        }
    }
}

impl std::ops::Mul<isize> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
enum Direction {
    East = 0,
    South = 1,
    West = 2,
    North = 3,
}

impl Into<isize> for Direction {
    fn into(self) -> isize {
        self as _
    }
}

impl std::convert::TryFrom<isize> for Direction {
    type Error = &'static str;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if (0..=3).contains(&value) {
            Ok(unsafe { std::mem::transmute(value as u8) })
        } else {
            Err("direction out of bounds!")
        }
    }
}

impl Direction {
    fn vec(self) -> Vec2 {
        match self {
            Direction::East => Vec2 { x: 1, y: 0 },
            Direction::South => Vec2 { x: 0, y: -1 },
            Direction::West => Vec2 { x: -1, y: 0 },
            Direction::North => Vec2 { x: 0, y: 1 },
        }
    }
}

/// Represents an angle, in multiples of 90°
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct AngleDelta(isize);

impl std::ops::Add<AngleDelta> for Direction {
    type Output = Self;

    fn add(self, rhs: AngleDelta) -> Self::Output {
        let angle: isize = self.into();
        (angle + rhs.0).rem_euclid(4).try_into().unwrap()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct ShipState {
    pos: Vec2,
    dir: Direction,
    waypoint: Vec2,
}

impl std::ops::Add<Instruction> for ShipState {
    type Output = Self;

    fn add(self, rhs: Instruction) -> Self::Output {
        match rhs {
            Instruction::Move(dir, units) => Self {
                pos: self.pos + dir.vec() * units,
                ..self
            },
            Instruction::Rotate(delta) => Self {
                dir: self.dir + delta,
                ..self
            },
            Instruction::Advance(units) => Self {
                pos: self.pos + self.dir.vec() * units,
                ..self
            },
        }
    }
}

impl ShipState {
    fn apply(self, ins: Instruction) -> Self {
        match ins {
            // moves waypoint
            Instruction::Move(dir, units) => Self {
                waypoint: self.waypoint + dir.vec() * units,
                ..self
            },
            // rotates waypoint (relative to ship)
            Instruction::Rotate(delta) => Self {
                waypoint: self.waypoint.rotate(delta),
                ..self
            },
            // advance towards waypoint
            Instruction::Advance(units) => Self {
                pos: self.pos + self.waypoint * units,
                ..self
            },
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Instruction {
    /// Moves in given direction
    Move(Direction, isize),
    /// Turns
    Rotate(AngleDelta),
    /// Moves forward
    Advance(isize),
}

fn parse_instructions(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    input.lines().map(|line| {
        let command = line.as_bytes()[0];
        // Safety: this will panic if `line` starts with multibyte character
        let number: isize = (&line[1..]).parse().unwrap();

        match command {
            b'N' => Instruction::Move(Direction::North, number),
            b'S' => Instruction::Move(Direction::South, number),
            b'E' => Instruction::Move(Direction::East, number),
            b'W' => Instruction::Move(Direction::West, number),
            b'L' => Instruction::Rotate(AngleDelta(-number / 90)),
            b'R' => Instruction::Rotate(AngleDelta(number / 90)),
            b'F' => Instruction::Advance(number),
            c => panic!("unknown instruction {}", c as char),
        }
    })
}

fn part1(input: &str) {
    let start = ShipState {
        dir: Direction::East,
        pos: Vec2 { x: 0, y: 0 },
        waypoint: Vec2 { x: 0, y: 0 },
    };
    let end = parse_instructions(input).fold(start, |state, ins| state + ins);
    let res = (end.pos - start.pos).manhattan();
    println!("part1: {}", res);
}

fn part2(input: &str) {
    let start = ShipState {
        dir: Direction::East,
        pos: Vec2 { x: 0, y: 0 },
        waypoint: Vec2 { x: 10, y: 1 },
    };
    let end = parse_instructions(input).fold(start, |state, ins| state.apply(ins));
    let res = (end.pos - start.pos).manhattan();
    println!("part2: {}", res);
}

fn main() {
    let input = include_str!("../../../data/day12.txt");
    part1(input);
    part2(input);
}

#[test]
fn vec2_add() {
    let a = Vec2 { x: 3, y: 8 };
    let b = Vec2 { x: 2, y: 10 };
    assert_eq!(a + b, Vec2 { x: 5, y: 18 });
}

#[test]
fn manhattan_example() {
    let start = Vec2 { x: 0, y: 0 };
    let end = Vec2 { x: 17, y: -8 };
    assert_eq!((end - start).manhattan(), 25);
}

#[test]
fn direction_try_from() {
    use std::convert::TryFrom;

    assert_eq!(
        <Direction as TryFrom<isize>>::try_from(0).unwrap(),
        Direction::East
    );
    assert_eq!(
        <Direction as TryFrom<isize>>::try_from(2).unwrap(),
        Direction::West
    );
    assert!(<Direction as TryFrom<isize>>::try_from(-1).is_err(),);
    assert!(<Direction as TryFrom<isize>>::try_from(4).is_err(),);
}

#[test]
fn test_direction_add() {
    // From example
    assert_eq!(Direction::East + AngleDelta(1), Direction::South);
    // Turning "left" (counter-clockwise)
    assert_eq!(Direction::East + AngleDelta(-1), Direction::North);
    // Doing a 360°
    assert_eq!(Direction::East + AngleDelta(4), Direction::East);
}

#[test]
fn test_rotate() {
    let v = Vec2 { x: 3, y: 1 };
    assert_eq!(v.rotate(AngleDelta(0)), v);
    assert_eq!(v.rotate(AngleDelta(4)), v);
    assert_eq!(v.rotate(AngleDelta(-4)), v);

    assert_eq!(v.rotate(AngleDelta(1)), Vec2 { x: 1, y: -3 });
    assert_eq!(v.rotate(AngleDelta(2)), Vec2 { x: -3, y: -1 });
    assert_eq!(v.rotate(AngleDelta(3)), Vec2 { x: -1, y: 3 });
}
