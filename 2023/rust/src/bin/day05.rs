use itertools::Itertools;
use std::collections::BTreeSet;

fn lowest_loc(maps: &[BTreeSet<(u64, u64, u64)>], mut ranges: BTreeSet<(u64, u64)>) -> u64 {
    for i in 0..7 {
        for (mut start, mut len) in std::mem::take(&mut ranges) {
            for &(rng_src, rng_len, rng_dst) in &maps[i] {
                if start + len <= rng_src {
                    break;
                } else if rng_src + rng_len <= start {
                    continue;
                }

                if start < rng_src {
                    let len_before = rng_src - start;
                    ranges.insert((start, len_before));
                    start += len_before;
                    len -= len_before;
                }

                let off_ovl = start - rng_src;
                let len_ovl = len.min(rng_len - off_ovl);
                ranges.insert((rng_dst + off_ovl, len_ovl));
                start += len_ovl;
                len -= len_ovl;
            }

            if len > 0 {
                ranges.insert((start, len));
            }
        }
    }
    ranges.first().unwrap().0
}

fn main() {
    let input = include_str!("../../input/day05.txt");
    let (seeds, maps) = input.split_once("\n\n").unwrap();

    let seeds = seeds
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let maps = maps
        .split("\n\n")
        .map(|part| {
            part.lines()
                .skip(1)
                .map(|line| {
                    let (dst, src, len) = line
                        .split(' ')
                        .map(|v| v.parse::<u64>().unwrap())
                        .collect_tuple()
                        .unwrap();
                    (src, len, dst)
                })
                .collect::<BTreeSet<_>>()
        })
        .collect_vec();

    let part1 = seeds.iter().map(|&s| (s, 1)).collect();
    let p1 = lowest_loc(&maps, part1);
    println!("part1: {}", p1);

    let part2 = seeds.into_iter().tuples().collect();
    let p2 = lowest_loc(&maps, part2);
    println!("part2: {}", p2);
}
