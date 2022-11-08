use im::HashMap;

#[derive(Default, Clone, Copy, Debug)]
struct Results {
    ones: usize,
    threes: usize,
}

fn part1(numbers: &[usize]) {
    let results = numbers.windows(2).fold(Results::default(), |acc, s| {
        if let [x, y] = s {
            match y - x {
                1 => Results {
                    ones: acc.ones + 1,
                    ..acc
                },
                3 => Results {
                    threes: acc.threes + 1,
                    ..acc
                },
                gap => panic!("invalid input (found {} gap)", gap),
            }
        } else {
            unreachable!()
        }
    });

    dbg!(results, results.ones * results.threes);
}

fn part2(numbers: &[usize]) {
    let mut num_paths = HashMap::new();
    let n = numbers.len();
    num_paths.insert(numbers.last().copied().unwrap(), 1);
    for i in (0..(n - 1)).into_iter().rev() {
        let i_val = numbers[i];
        let range = (i + 1)..=std::cmp::min(i + 3, n - 1);

        let num_neighbors: usize = range
            .filter_map(|j| {
                let j_val = numbers[j];
                let gap = j_val - i_val;
                if (1..=3).contains(&gap) {
                    Some(num_paths.get(&j_val).unwrap())
                } else {
                    None
                }
            })
            .sum();
        num_paths.insert(i_val, num_neighbors);
    }
    dbg!(num_paths.get(&0));
}

fn main() {
    let input = include_str!("../../../data/day10.txt");
    let mut numbers: Vec<_> = std::iter::once(0)
        .chain(input.lines().map(|x| x.parse::<usize>().unwrap()))
        .collect();
    numbers.sort_unstable();

    if let Some(&max) = numbers.iter().max() {
        numbers.push(max + 3);
    }

    part1(&numbers);
    part2(&numbers);
}
