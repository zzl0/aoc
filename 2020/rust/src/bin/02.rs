use anyhow::Result;
use std::ops::RangeInclusive;

fn main() -> Result<()> {
    let input = include_str!("../../../data/day02.txt");

    part1(input)?;
    part2(input)?;

    Ok(())
}

#[derive(Debug, PartialEq)]
struct PasswordPolicy {
    byte: u8,
    range: RangeInclusive<usize>,
}

impl PasswordPolicy {
    fn is_valid(&self, password: &str) -> bool {
        self.range.contains(
            &password
                .as_bytes()
                .iter()
                .filter(|&&b| b == self.byte)
                .count(),
        )
    }
}

fn parse_line(s: &str) -> Result<(PasswordPolicy, &str)> {
    // 1-4 j: jjjqzmgbjwpj
    peg::parser! {
        grammar parser() for str {
            rule number() -> usize
                = n:$(['0'..='9']+) { n.parse().unwrap() }

            rule range() -> RangeInclusive<usize>
                = min:number() "-" max:number() { min..=max }

            rule byte() -> u8
                = letter:$(['a'..='z']) { letter.as_bytes()[0] }

            rule password() -> &'input str
                = letters:$([_]*) { letters }

            pub(crate) rule line() -> (PasswordPolicy, &'input str)
                = range:range() " " byte:byte() ": " password:password() {
                    (PasswordPolicy{range, byte}, password)
                }

        }
    }
    Ok(parser::line(s)?)
}

fn part1(input: &str) -> Result<()> {
    let count = input
        .lines()
        .map(|line| parse_line(line).unwrap())
        .filter(|(policy, password)| policy.is_valid(password))
        .count();

    println!("part1: {}", count);
    Ok(())
}

#[derive(Debug, PartialEq)]
struct PasswordPolicy2 {
    byte: u8,
    positions: [usize; 2],
}

impl PasswordPolicy2 {
    fn is_valid(&self, password: &str) -> bool {
        self.positions
            .iter()
            .filter(|&&i| password.as_bytes()[i] == self.byte)
            .count()
            == 1
    }
}

fn parse_line2(s: &str) -> Result<(PasswordPolicy2, &str)> {
    // 1-4 j: jjjqzmgbjwpj
    peg::parser! {
        grammar parser() for str {
            rule number() -> usize
                = n:$(['0'..='9']+) { n.parse().unwrap() }

            rule position() -> usize
                = n:number() { n - 1 }

            rule positions() -> [usize; 2]
                = first:position() "-" second:position() { [first, second] }

            rule byte() -> u8
                = letter:$(['a'..='z']) { letter.as_bytes()[0] }

            rule password() -> &'input str
                = letters:$([_]*) { letters }

            pub(crate) rule line() -> (PasswordPolicy2, &'input str)
                = positions:positions() " " byte:byte() ": " password:password() {
                    (PasswordPolicy2 { byte, positions }, password)
                }

        }
    }
    Ok(parser::line(s)?)
}

fn part2(input: &str) -> Result<()> {
    let count = input
        .lines()
        .map(|line| parse_line2(line).unwrap())
        .filter(|(policy, password)| policy.is_valid(password))
        .count();

    println!("part2: {}", count);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        let pp = PasswordPolicy {
            range: 1..=3,
            byte: b'a',
        };

        assert!(!pp.is_valid("zeus"), "no 'a's");
        assert!(pp.is_valid("hades"), "single 'a'");
        assert!(pp.is_valid("banana"), "three 'a's");
        assert!(!pp.is_valid("aaaah"), "too many 'a's");
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            parse_line("1-3 a: banana").unwrap(),
            (
                PasswordPolicy {
                    range: 1..=3,
                    byte: b'a'
                },
                "banana"
            )
        );
    }
}
