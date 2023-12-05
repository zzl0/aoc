use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input/day04.txt");
    let (mut p1, mut p2) = (0, 0);
    let mut copies = vec![1; 256];

    for (i, l) in input.split('\n').enumerate() {
        let (_, rest) = l.split_once(": ").unwrap();
        let (wanted, got) = rest.split_once(" | ").unwrap();
        let wanted = wanted.split_whitespace().collect::<HashSet<_>>();
        let got = got.split_whitespace().collect::<HashSet<_>>();
        let won = wanted.intersection(&got).count();
        p1 += if won != 0 { 1 << (won - 1) } else { 0 };
        p2 += copies[i];
        for j in 0..won {
            copies[i + j + 1] += copies[i];
        }
    }

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
