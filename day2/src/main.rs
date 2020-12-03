use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let password_file = "input.txt";

    let input = File::open(password_file).unwrap();
    let buffered = BufReader::new(input);

    let valid_count = buffered
        .lines()
        .filter(|x| {
            let (rule, password) = parse_sled_password(x.as_ref().unwrap());
            valid_sled_password(rule, password)
        })
        .count();

    println!("Valid sled passwords: {}", valid_count);

    let input = File::open(password_file).unwrap();
    let buffered = BufReader::new(input);

    let valid_count = buffered
        .lines()
        .filter(|x| {
            let (rule, password) = parse_toboggan_password(x.as_ref().unwrap());
            valid_toboggan_password(rule, password)
        })
        .count();

    println!("Valid toboggan passwords: {}", valid_count);
}

fn parse_sled_password(input: &str) -> (SledRule, &str) {
    let sep = input.find(':').unwrap();
    let (rule_str, password) = input.split_at(sep);
    let password = password.strip_prefix(": ").unwrap();
    let (min, remainder) = rule_str.split_at(rule_str.find('-').unwrap());
    let remainder = remainder.strip_prefix('-').unwrap();
    let (max, remainder) = remainder.split_at(remainder.find(' ').unwrap());
    let character = remainder.strip_prefix(' ').unwrap();
    (
        SledRule {
            character: character.chars().nth(0).unwrap(),
            minimum: min.parse().unwrap(),
            maximum: max.parse().unwrap(),
        },
        password,
    )
}

fn valid_sled_password(rule: SledRule, password: &str) -> bool {
    match password.find(rule.character) {
        None => return false,
        _ => {}
    }

    let mut important_chars = password.to_string();
    important_chars.retain(|c| c == rule.character);
    if important_chars.len() >= rule.minimum && important_chars.len() <= rule.maximum {
        true
    } else {
        false
    }
}

#[derive(Debug, PartialEq)]
struct SledRule {
    character: char,
    minimum: usize,
    maximum: usize,
}

fn parse_toboggan_password(input: &str) -> (TobogganRule, &str) {
    let sep = input.find(':').unwrap();
    let (rule_str, password) = input.split_at(sep);
    let password = password.strip_prefix(": ").unwrap();
    let (pos1, remainder) = rule_str.split_at(rule_str.find('-').unwrap());
    let remainder = remainder.strip_prefix('-').unwrap();
    let (pos2, remainder) = remainder.split_at(remainder.find(' ').unwrap());
    let character = remainder.strip_prefix(' ').unwrap();
    (
        TobogganRule {
            character: character.chars().nth(0).unwrap(),
            position1: pos1.parse().unwrap(),
            position2: pos2.parse().unwrap(),
        },
        password,
    )
}

fn valid_toboggan_password(rule: TobogganRule, password: &str) -> bool {
    match password.find(rule.character) {
        None => return false,
        _ => {}
    }

    let char_pos1 = password.chars().nth(rule.position1 - 1).unwrap();
    let char_pos2 = password.chars().nth(rule.position2 - 1).unwrap();

    if char_pos1 != char_pos2 && (char_pos1 == rule.character || char_pos2 == rule.character) {
        true
    } else {
        false
    }
}

#[derive(Debug, PartialEq)]
struct TobogganRule {
    character: char,
    position1: usize,
    position2: usize,
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example() {
        let input = vec![
            ("1-3 a: abcde", true),
            ("1-3 b: cdefg", false),
            ("2-9 c: ccccccccc", true),
        ];
        for (input_line, expectation) in input.into_iter() {
            let (rule, pass) = parse_sled_password(input_line);
            assert_eq!(valid_sled_password(rule, pass), expectation);
        }
    }

    #[test]
    fn example_part2() {
        let input = vec![
            ("1-3 a: abcde", true),
            ("1-3 b: cdefg", false),
            ("2-9 c: ccccccccc", false),
        ];
        for (input_line, expectation) in input.into_iter() {
            println!("{}", input_line);
            let (rule, pass) = parse_toboggan_password(input_line);
            assert_eq!(valid_toboggan_password(rule, pass), expectation);
        }
    }

    #[test]
    fn parse_password_test() {
        let input = "1-3 a: abcde";
        assert_eq!(
            parse_sled_password(input),
            (
                SledRule {
                    character: 'a',
                    minimum: 1,
                    maximum: 3,
                },
                "abcde"
            )
        );
    }
}
