use advtools::{input, prelude::Itertools};

fn main() {
    let input = input::string().as_bytes();

    let find = |n| {
        n + input
            .windows(n)
            .position(|win| win.iter().all_unique())
            .unwrap()
    };

    advtools::verify("part1", find(4), 1034);
    advtools::verify("part1", find(14), 2472);
}
