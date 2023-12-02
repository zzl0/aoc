const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn calibration(input: &str, f: impl Fn(char, &str) -> Option<u32>) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut digits = line.char_indices().filter_map(|(i, c)| f(c, &line[i..]));
            let first = digits.next().unwrap();
            let last = digits.next_back().unwrap_or(first);
            10 * first + last
        })
        .sum()
}

fn main() {
    let input: &str = include_str!("../../input/day01.txt");

    let p1 = calibration(input, |c, _| c.to_digit(10));
    println!("part 1: {}", p1);

    let p2 = calibration(input, |c, rest| {
        c.to_digit(10).or_else(|| {
            DIGITS
                .iter()
                .position(|d| rest.starts_with(d))
                .map(|i| i as u32 + 1)
        })
    });
    println!("part 2: {}", p2);
}
