fn parse_game(line: &str) -> Option<(usize, usize)> {
    let (game_id, game) = line.trim_start_matches("Game ").split_once(':')?;
    let (mut r, mut g, mut b, mut possible) = (0, 0, 0, true);
    for round in game.split([';', ',']) {
        let (n, color) = round.trim().split_once(' ')?;
        let n = n.parse().ok()?;
        match color.as_bytes()[0] {
            b'r' => {
                possible &= n <= 12;
                r = r.max(n);
            }
            b'g' => {
                possible &= n <= 13;
                g = g.max(n);
            }
            b'b' => {
                possible &= n <= 14;
                b = b.max(n);
            }
            _ => unreachable!(),
        }
    }
    let p1 = if possible { game_id.parse().ok()? } else { 0 };
    let p2 = r * g * b;
    Some((p1, p2))
}

fn main() {
    let input = include_str!("../../input/day02.txt");

    let (p1, p2) = input.split('\n').fold((0, 0), |(p1, p2), line| {
        let (a, b) = parse_game(line).unwrap();
        (p1 + a, p2 + b)
    });
    println!("part1: {}", p1);
    println!("part2: {}", p2);
}
