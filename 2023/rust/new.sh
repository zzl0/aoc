#!/usr/bin/env bash

if [ $# -ne 1 ]; then
    echo "Require one argument, e.g.: ./new.sh day01"
    exit 1
fi

set -x
touch input/$1.txt
cat <<EOT > src/bin/$1.rs
fn main() {
    let input = include_str!("../../input/$1.txt");

    // let p1 =
    // println!("part1: {}", p1);

    // let p2 =
    // println!("part2: {}", p2);
}
EOT
