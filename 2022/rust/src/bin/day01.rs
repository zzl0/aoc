use advtools::input;
use advtools::itertools::Itertools;

fn main() {
    let chunks = input::string().split("\n\n");

    let (c1, c2, c3) = chunks
        .map(|elf| elf.lines().flat_map(str::parse::<i32>).sum::<i32>())
        .sorted_by_key(|k| -k)
        .next_tuple()
        .unwrap();

    println!("part1: {}", c1);
    println!("part2: {}", c1 + c2 + c3);
    // part1: 65912
    // part2: 195625
}
