use bitvec::prelude::*;

#[derive(Default, Debug, PartialEq)]
struct Seat {
    row: u8,
    col: u8,
}

impl Seat {
    const ROW_BITS: usize = 7;
    const COL_BITS: usize = 3;

    fn parse(input: &str) -> Self {
        let bytes = input.as_bytes();
        let mut res: Seat = Default::default();

        {
            let row = BitSlice::<_, Msb0>::from_element_mut(&mut res.row);
            for (i, &b) in bytes[0..Self::ROW_BITS].iter().enumerate() {
                row.set(
                    (8 - Self::ROW_BITS) + i,
                    match b {
                        b'F' => false,
                        b'B' => true,
                        _ => panic!("unexpected row letter: {}", b as char),
                    },
                )
            }
        }

        {
            let col = BitSlice::<_, Msb0>::from_element_mut(&mut res.col);
            for (i, &b) in bytes[Self::ROW_BITS..][..Self::COL_BITS].iter().enumerate() {
                col.set(
                    (8 - Self::COL_BITS) + i,
                    match b {
                        b'L' => false,
                        b'R' => true,
                        _ => panic!("unexpected col letter: {}", b as char),
                    },
                );
            }
        }

        res
    }

    fn id(&self) -> u64 {
        ((self.row as u64) << Self::COL_BITS) + (self.col as u64)
    }
}

fn part1(input: &str) {
    let max_id = input
        .lines()
        .map(Seat::parse)
        .map(|seat| seat.id())
        .max()
        .unwrap();
    println!("part1: {}", max_id);
}

fn main() {
    let input = include_str!("../../../data/day05.txt");
    part1(input);
}

#[test]
fn test_04_parse() {
    let input = "FBFBBFFRLR";
    let seat = Seat::parse(input);
    assert_eq!(seat, Seat { row: 44, col: 5 });
}

#[test]
fn test_04_seat_id() {
    macro_rules! validate {
        ($input:expr, $row:expr, $col:expr, $id:expr) => {
            let seat = Seat::parse($input);
            assert_eq!(
                seat,
                Seat {
                    row: $row,
                    col: $col
                }
            );
            assert_eq!(seat.id(), $id);
        };
    }
    validate!("BFFFBBFRRR", 70, 7, 567);
    validate!("FFFBBBFRRR", 14, 7, 119);
    validate!("BBFFBBFRLL", 102, 4, 820);
}
