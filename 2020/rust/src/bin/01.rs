use anyhow::{anyhow, Result};
use itertools::Itertools;

macro_rules! err {
    ($($tt:tt)*) => {
        anyhow!(format!($($tt)*))
    };
}

fn main() -> anyhow::Result<()> {
    let nums = include_str!("../../../data/day01.txt")
        .split('\n')
        .map(|line| {
            line.parse::<i64>()
                .map_err(|e| err!("failed to parse '{:?}': {}", line, e))
        })
        .collect::<Result<Vec<_>, _>>()?;

    // part 1
    let (a, b) = nums
        .iter()
        .tuple_combinations()
        .find(|&(a, b)| a + b == 2020)
        .expect("no pair had a sum of 2020");
    dbg!(a + b);
    dbg!(a * b);

    // part 2
    let (a, b, c) = nums
        .iter()
        .tuple_combinations()
        .find(|&(a, b, c)| a + b + c == 2020)
        .expect("no tuple of length 3 had a sum of 2020");
    dbg!(a + b + c);
    dbg!(a * b * c);

    Ok(())
}
