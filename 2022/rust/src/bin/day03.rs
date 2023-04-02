use advtools::input;
use advtools::itertools::Itertools;

fn common_prio(itemlists: &[&[u8]]) -> i32 {
    let common = itemlists[0]
        .iter()
        .find(|item| itemlists[1..].iter().all(|l| l.contains(item)));

    match common {
        Some(c @ b'a'..=b'z') => (c - b'a' + 1) as i32,
        Some(c @ b'A'..=b'Z') => (c - b'A' + 27) as i32,
        _ => unreachable!(),
    }
}

fn main() {
    // part 1
    let score: i32 = input::lines()
        .map(|line| {
            let line = line.as_bytes();
            let len = line.len() / 2;
            common_prio(&[&line[..len], &line[len..]])
        })
        .sum();
    advtools::verify("Part 1 score", score, 8240);

    // part 2
    let score: i32 = input::lines()
        .tuples()
        .map(|(line1, line2, line3)| {
            common_prio(&[line1.as_bytes(), line2.as_bytes(), line3.as_bytes()])
        })
        .sum();
    advtools::verify("Part 2 score", score, 2587);
    //
}
