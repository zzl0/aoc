use std::collections::HashMap;

fn find_symbol(lines: &[&[u8]], row: usize, col: usize) -> Option<(usize, usize, u8)> {
    for (dr, dc) in [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ] {
        let (rr, cc) = ((row as i32 + dr) as usize, (col as i32 + dc) as usize);
        let Some(&s) = lines.get(rr).and_then(|line| line.get(cc)) else {
            continue;
        };
        if s != b'.' && !s.is_ascii_digit() {
            return Some((rr, cc, s));
        }
    }
    None
}

fn main() {
    let input = include_str!("../../input/day03.txt");
    let lines = input.split('\n').map(str::as_bytes).collect::<Vec<_>>();
    let mut symbols = HashMap::new();

    for (row, line) in lines.iter().enumerate() {
        let mut col = 0;
        while col < line.len() {
            let (start, mut symbol) = (col, None);
            while col < line.len() && line[col].is_ascii_digit() {
                symbol = symbol.or_else(|| find_symbol(&lines, row, col));
                col += 1;
            }

            if let Some(symbol) = symbol {
                let num = line[start..col]
                    .iter()
                    .fold(0, |n, c| n * 10 + (c - b'0') as usize);
                symbols.entry(symbol).or_insert(Vec::new()).push(num);
            }
            col += 1;
        }
    }

    let p1: usize = symbols.values().flatten().sum();
    let p2: usize = symbols
        .iter()
        .filter(|(&(_, _, s), v)| s == b'*' && v.len() == 2)
        .map(|(_, v)| v[0] * v[1])
        .sum();

    println!("part1: {}", p1);
    println!("part2: {}", p2);
}
