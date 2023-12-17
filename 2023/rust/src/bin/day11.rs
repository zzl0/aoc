use itertools::Itertools;

fn solve(universe: &Vec<&[u8]>, mut galaxies: Vec<(usize, usize)>, size: usize) -> usize {
    let (rows, cols) = (universe.len(), universe[0].len());
    let empty_rows = (0..rows).filter(|&r| universe[r].iter().all(|&c| c == b'.'));
    let empty_cols = (0..cols).filter(|&c| (0..rows).all(|r| universe[r][c] == b'.'));

    for r in empty_rows.rev() {
        for g in &mut galaxies {
            if g.0 > r {
                g.0 += size - 1
            }
        }
    }

    for c in empty_cols.rev() {
        for g in &mut galaxies {
            if g.1 > c {
                g.1 += size - 1
            }
        }
    }

    galaxies
        .iter()
        .tuple_combinations()
        .map(|(&(r1, c1), &(r2, c2))| r1.abs_diff(r2) + c1.abs_diff(c2))
        .sum()
}

fn main() {
    let input = include_str!("../../input/day11.txt");
    let universe = input.split('\n').map(|l| l.as_bytes()).collect_vec();
    let galaxies = (0..universe.len())
        .cartesian_product(0..universe[0].len())
        .filter(|&(r, c)| universe[r][c] == b'#')
        .collect_vec();

    let p1 = solve(&universe, galaxies.clone(), 2);
    println!("part1: {}", p1);

    let p2 = solve(&universe, galaxies, 1000000);
    println!("part2: {}", p2);
}
