use anyhow::Result;
use std::ops::RangeInclusive;

fn main() -> Result<()> {
    let input = include_str!("../../../data/day02.txt");

    part1(input)?;

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

#[derive(thiserror::Error, Debug)]
enum ParseError {
    #[error("expected {0}")]
    Expected(&'static str),
}

fn parse_line(s: &str) -> Result<(PasswordPolicy, &str)> {
    // 1-4 j: jjjqzmgbjwpj
    let (policy, password) = {
        let mut tokens = s.split(':');
        (
            tokens
                .next()
                .ok_or(ParseError::Expected("password policy"))?,
            tokens
                .next()
                .ok_or(ParseError::Expected("password"))?
                .trim(),
        )
    };

    let (range, byte) = {
        let mut tokens = policy.split(' ');
        (
            tokens.next().ok_or(ParseError::Expected("policy range"))?,
            tokens.next().ok_or(ParseError::Expected("policy byte"))?,
        )
    };

    let byte = if byte.as_bytes().len() == 1 {
        byte.as_bytes()[0]
    } else {
        return Err(ParseError::Expected("password policy byte to be exactly 1 byte").into());
    };

    let (min, max) = {
        let mut tokens = range.split('-');
        (
            tokens
                .next()
                .ok_or(ParseError::Expected("policy range (lower bound)"))?,
            tokens
                .next()
                .ok_or(ParseError::Expected("policy range (upperbound)"))?,
        )
    };

    let range = (min.parse()?)..=(max.parse()?);
    Ok((PasswordPolicy { range, byte }, password))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        let pp = PasswordPolicy {
            range: 1..=3,
            byte: b'a',
        };

        assert_eq!(pp.is_valid("zeus"), false, "no 'a's");
        assert_eq!(pp.is_valid("hades"), true, "single 'a'");
        assert_eq!(pp.is_valid("banana"), true, "three 'a's");
        assert_eq!(pp.is_valid("aaaah"), false, "too many 'a's");
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

        assert_eq!(
            parse_line("1-3 a").unwrap_err().to_string(),
            "expected password"
        );
        assert_eq!(
            parse_line("1-3 : banana").unwrap_err().to_string(),
            "expected password policy byte to be exactly 1 byte"
        );
    }
}
