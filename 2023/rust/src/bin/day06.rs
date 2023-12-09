use itertools::Itertools;

fn ways((t, d): (f64, f64)) -> u64 {
    // d = (t - w) * w => w = (t +- sqrt(t^2 - 4d)) / 2
    let diff = (t.powf(2.) - 4. * d).sqrt();
    let mut l = (t - diff) / 2.0;
    let mut h = (t + diff) / 2.0;

    // remove equal cases
    if l.fract() == 0.0 {
        l += 1.0;
    }
    if h.fract() == 0.0 {
        h -= 1.0;
    }
    (h.floor() - l.ceil()) as u64 + 1
}

fn main() {
    let input = include_str!("../../input/day06.txt");

    let (times, dist) = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .map(|v| v.parse::<f64>().unwrap())
                .collect_vec()
        })
        .collect_tuple()
        .unwrap();
    let p1 = times.into_iter().zip(dist).map(ways).product::<u64>();
    println!("part1: {}", p1);

    let single = input
        .lines()
        .map(|line| {
            line.split_once(":")
                .unwrap()
                .1
                .replace(" ", "")
                .parse::<f64>()
                .unwrap()
        })
        .collect_tuple()
        .unwrap();
    let p2 = ways(single);
    println!("part2: {}", p2);
}
