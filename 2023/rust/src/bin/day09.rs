use itertools::Itertools;

fn solve(xs: Vec<isize>) -> (isize, isize) {
    let mut v = vec![xs];
    while v[v.len() - 1].iter().any(|&x| x != 0) {
        let xs = v[v.len() - 1]
            .iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .collect();
        v.push(xs);
    }
    v.iter()
        .rev()
        .fold((0, 0), |(a, b), xs| (xs[xs.len() - 1] + a, xs[0] - b))
}

fn main() {
    let input = include_str!("../../input/day09.txt");
    let (p1, p2) = input.split('\n').fold((0, 0), |(p1, p2), line| {
        let xs = line
            .split_whitespace()
            .map(|x| x.parse::<isize>().unwrap())
            .collect_vec();
        let (a, b) = solve(xs);
        (p1 + a, p2 + b)
    });
    println!("part1: {}", p1);
    println!("part2: {}", p2);
}
