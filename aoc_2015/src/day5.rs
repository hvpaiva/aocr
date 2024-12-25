use std::collections::HashMap;

use aoc_runner_derive::aoc;
use itertools::Itertools;

struct Letter(String);

fn has_three_vowels(s: &str) -> bool {
    let is_vowel = |c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u');
    s.chars().filter(|&c| is_vowel(c)).count() >= 3
}

fn has_double(s: &str) -> bool {
    s.chars()
        .tuple_windows::<(char, char)>()
        .any(|(a, b)| a == b)
}

fn is_not_in_blacklist(s: &str) -> bool {
    !s.contains("ab") && !s.contains("cd") && !s.contains("pq") && !s.contains("xy")
}

fn has_repeated_pair_with_inner(s: &str) -> bool {
    s.chars()
        .tuple_windows::<(char, char, char)>()
        .any(|(a, _, c)| a == c)
}

fn has_pair_twice(s: &str) -> bool {
    let mut seen_pairs: HashMap<&str, usize> = HashMap::new();
    let chars = s.as_bytes();

    for i in 0..chars.len().saturating_sub(1) {
        let pair = &s[i..i + 2];
        if let Some(&prev_index) = seen_pairs.get(pair) {
            if prev_index + 1 < i {
                return true;
            }
        } else {
            seen_pairs.insert(pair, i);
        }
    }

    false
}

impl Letter {
    fn new(s: &str) -> Self {
        Self(s.to_string())
    }

    fn is_nice(&self, rules: Vec<fn(&str) -> bool>) -> bool {
        rules.iter().all(|rule| rule(&self.0))
    }
}

#[aoc(day5, part1, Clearer)]
fn solve_one(input: &str) -> usize {
    input
        .lines()
        .map(Letter::new)
        .filter(|l| l.is_nice(vec![has_three_vowels, has_double, is_not_in_blacklist]))
        .count()
}

#[aoc(day5, part2)]
fn solve_two(input: &str) -> usize {
    input
        .lines()
        .map(Letter::new)
        .filter(|l| l.is_nice(vec![has_repeated_pair_with_inner, has_pair_twice]))
        .count()
}

// I implemented this challenge before I actually started writing in Rust,
// so I only tested the real solution.
#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::input;

    use super::*;

    const INPUT: &str = input!("day5");

    #[test]
    fn integration_part_one() {
        assert_eq!(solve_one(INPUT), 258);
    }

    #[test]
    fn integration_part_two() {
        assert_eq!(solve_two(INPUT), 53);
    }

    #[test]
    fn test_has_three_vowels() {
        assert_eq!(has_three_vowels("aei"), true);
        assert_eq!(has_three_vowels("xazegov"), true);
        assert_eq!(has_three_vowels("aeiouaeiouaeiou"), true);
        assert_eq!(has_three_vowels("dvszwmarrgswjxmb"), false);
    }

    #[test]
    fn test_has_double() {
        assert_eq!(has_double("xx"), true);
        assert_eq!(has_double("abcdde"), true);
        assert_eq!(has_double("aabbccdd"), true);
        assert_eq!(has_double("jchzalrnumimnmhp"), false);
    }

    #[test]
    fn test_is_not_in_blacklist() {
        assert_eq!(is_not_in_blacklist("abcd"), false);
        assert_eq!(is_not_in_blacklist("pqrs"), false);
        assert_eq!(is_not_in_blacklist("xyzt"), false);
        assert_eq!(is_not_in_blacklist("ugknbfddgicrmopn"), true);
    }

    #[test]
    fn test_has_repeated_pair_with_inner() {
        assert_eq!(has_repeated_pair_with_inner("xyx"), true);
        assert_eq!(has_repeated_pair_with_inner("abcdefeghi"), true);
        assert_eq!(has_repeated_pair_with_inner("aaa"), true);
        assert_eq!(has_repeated_pair_with_inner("uurcxstgmygtbstg"), false);
    }

    #[test]
    fn test_has_pair_twice() {
        assert_eq!(has_pair_twice("xyxy"), true);
        assert_eq!(has_pair_twice("aabcdefgaa"), true);
        assert_eq!(has_pair_twice("aaa"), false);
        assert_eq!(has_pair_twice("ieodomkazucvgmuy"), false);
    }
}
