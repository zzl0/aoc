use itertools::Itertools;

fn main() {
    let input = include_str!("../../../data/day09.txt");
    let numbers = input
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let n = 25;
    let answer = numbers.windows(n + 1).find_map(|s| {
        if (&s[..n])
            .iter()
            .tuple_combinations()
            .any(|(a, b)| a + b == s[n])
        {
            None
        } else {
            Some(s[n])
        }
    });
    dbg!(answer);

    let answer = answer.unwrap();
    let answers2 = (2..numbers.len())
        .into_iter()
        .flat_map(|n| {
            numbers
                .windows(n)
                .enumerate()
                .map(move |(i, s)| (n, i, s.iter().sum::<usize>()))
        })
        .find(|&(_, _, sum)| sum == answer);

    let (n, i, _) = answers2.unwrap();
    let set = &numbers[i..(i + n)];
    let answer3 = set.iter().max().unwrap() + set.iter().min().unwrap();
    dbg!(answer3);
}
