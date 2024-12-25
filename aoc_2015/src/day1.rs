use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
fn solve_one(input: &str) -> i32 {
    input.chars().map(parse).sum()
}

#[aoc(day1, part2)]
fn solve_two(input: &str) -> i32 {
    input
        .chars()
        .map(parse)
        .enumerate()
        .fold_until(0, |acc, (i, c)| {
            *acc += c;
            if *acc < 0 {
                return Until::Break(i as i32 + 1);
            }
            Until::Continue
        })
        .expect_break("Santa didn't reach the basement")
}

fn parse(ch: char) -> i32 {
    match ch {
        '(' => 1,
        ')' => -1,
        _ => panic!("Invalid character: {}", ch),
    }
}

#[derive(Debug, PartialEq)]
enum Until<T> {
    Continue,
    Break(T),
}

impl<T> Until<T> {
    fn expect_break(self, message: &str) -> T {
        match self {
            Until::Break(res) => res,
            _ => panic!("{}", message),
        }
    }
}

trait FoldUntil: Iterator + Sized {
    fn fold_until<F, T>(self, mut acc: T, mut fold: F) -> Until<T>
    where
        F: FnMut(&mut T, Self::Item) -> Until<T>,
    {
        for item in self {
            match fold(&mut acc, item) {
                Until::Continue => continue,
                Until::Break(res) => return Until::Break(res),
            }
        }
        Until::Continue
    }
}

impl<I: Iterator> FoldUntil for I {}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::input;

    use super::*;

    #[rstest]
    #[case("(())", 0)]
    #[case("()()", 0)]
    #[case("(((", 3)]
    #[case("(()(()(", 3)]
    #[case("))(((((", 3)]
    #[case("())", -1)]
    #[case("))(", -1)]
    #[case(")))", -3)]
    #[case(")())())", -3)]
    fn test_part_one(#[case] input: &str, #[case] expected: i32) {
        assert_eq!(solve_one(input), expected);
    }

    #[rstest]
    #[case("))(((((", 1)]
    #[case("())", 3)]
    #[case("(())())", 7)]
    fn test_part_two(#[case] input: &str, #[case] expected: i32) {
        assert_eq!(solve_two(input), expected);
    }

    #[rstest]
    #[case("(())")]
    #[case("(((")]
    #[case("(()(()(")]
    #[should_panic(expected = "Santa didn't reach the basement")]
    fn test_part_two_panic(#[case] input: &str) {
        solve_two(input);
    }

    #[test]
    fn test_fold_until() {
        let input = [1, 2, 3, 4, 5];
        let result = input.iter().fold_until(0, |acc, &x| {
            *acc += x;
            if *acc > 5 {
                Until::Break(*acc)
            } else {
                Until::Continue
            }
        });

        assert_eq!(result, Until::Break(6));
    }

    #[test]
    #[should_panic(expected = "test")]
    fn test_fold_until_panic() {
        let input = [1, 2, 3, 4, 5];
        let _ = input
            .iter()
            .fold_until(0, |_, _| Until::Continue)
            .expect_break("test");
    }

    #[test]
    fn test_fold_until_continue() {
        let input = [1, 2, 3, 4, 5];
        let result = input.iter().fold_until(0, |_, _| Until::Continue);
        assert_eq!(result, Until::Continue);
    }

    const INPUT: &str = input!("day1");

    #[test]
    fn integration_part_one() {
        assert_eq!(solve_one(INPUT), 232);
    }

    #[test]
    fn integration_part_two() {
        assert_eq!(solve_two(INPUT), 1783);
    }
}
