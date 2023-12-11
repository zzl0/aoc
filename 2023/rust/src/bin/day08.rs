use std::collections::HashMap;

fn gcd(n: usize, m: usize) -> usize {
    assert!(n != 0 && m != 0);
    let (mut n, mut m) = (n, m);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

fn steps(path: &[u8], graph: &HashMap<&[u8], (&[u8], &[u8])>, start: &[u8], goal: &[u8]) -> usize {
    let mut node = start;
    let cnt = path
        .iter()
        .cycle()
        .position(|&d| {
            node = if d == b'L' {
                graph[node].0
            } else {
                graph[node].1
            };
            node.ends_with(goal)
        })
        .unwrap();
    cnt + 1
}

fn main() {
    let input = include_str!("../../input/day08.txt");
    let (path, rest) = input.split_once("\n\n").unwrap();
    let graph = rest
        .split('\n')
        .map(|l| {
            let l = l.as_bytes();
            (&l[0..3], (&l[7..10], &l[12..15]))
        })
        .collect::<HashMap<_, _>>();

    let p1 = steps(path.as_bytes(), &graph, b"AAA", b"ZZZ");
    println!("part1: {}", p1);

    let p2 = graph
        .keys()
        .filter(|k| k.ends_with(b"A"))
        .map(|node| steps(path.as_bytes(), &graph, node, b"Z"))
        .fold(1, |ans, x| (x * ans) / gcd(x, ans));
    println!("part2: {}", p2);
}
