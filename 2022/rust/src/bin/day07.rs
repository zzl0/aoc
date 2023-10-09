use advtools::input;
use advtools::prelude::HashMap;

const RX: &str = r"\$ cd (.*)|(\d+) .*|\$ ls|dir .*";
const LIMIT: u64 = 100_000;
const REMAIN: u64 = 40_000_000;

fn main() {
    let mut curdir = Vec::new();
    let mut sizes = HashMap::<_, u64>::new();

    for line in input::rx_lines::<(&str, Option<u64>)>(RX) {
        match line {
            (_, Some(size)) => {
                for sub in 0..=curdir.len() {
                    *sizes.entry(curdir[..sub].to_vec()).or_default() += size;
                }
            }
            ("/" | "", _) => {}
            ("..", _) => {
                curdir.pop();
            }
            (subdir, _) => {
                curdir.push(subdir);
            }
        }
    }

    // Part 1: just sum the sizes up to the given limit.
    let sum_of_smaller = sizes.values().filter(|&&v| v <= LIMIT).sum::<u64>();
    advtools::verify("Sum of dirs < 100000", sum_of_smaller, 1444896);

    let to_delete = sizes[&vec![]] - REMAIN;
    let best_size = sizes.values().filter(|&&v| v >= to_delete).min().unwrap();
    advtools::verify("Size of dir to delete", best_size, 404395);
}
