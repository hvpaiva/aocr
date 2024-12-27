use aoc_runner_derive::aoc;

#[derive(Debug, PartialEq, Eq)]
struct LookAndSay {
    num: char,
    qt: usize,
}

fn look_and_say(input: &str) -> String {
    let mut number_by_quantity = Vec::new();

    for (i, num) in input.chars().enumerate() {
        if i == 0 {
            number_by_quantity.push(LookAndSay { num, qt: 1 });
        } else {
            let last = number_by_quantity.last_mut().unwrap();
            if last.num == num {
                last.qt += 1;
            } else {
                number_by_quantity.push(LookAndSay { num, qt: 1 });
            }
        }
    }

    let mut result = String::new();
    for LookAndSay { num, qt } in number_by_quantity {
        result.push_str(&qt.to_string());
        result.push(num);
    }

    result
}

fn play_game(input: &str, rounds: usize) -> String {
    let mut result = input.to_string();
    for _ in 0..rounds {
        result = look_and_say(&result);
    }
    result
}

#[aoc(day10, part1)]
fn solve_one(input: &str) -> usize {
    play_game(input, 40).len()
}

#[aoc(day10, part2)]
fn solve_two(input: &str) -> usize {
    play_game(input, 50).len()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::input;

    use super::*;

    #[rstest]
    #[case("1", "11")]
    #[case("11", "21")]
    #[case("21", "1211")]
    #[case("1211", "111221")]
    #[case("111221", "312211")]
    fn test_look_and_say(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(look_and_say(input), expected);
    }

    const INPUT: &str = input!("day10");

    #[test]
    fn integration_part_one() {
        assert_eq!(solve_one(INPUT.trim()), 252594);
    }

    #[test]
    fn integration_part_two() {
        assert_eq!(solve_two(INPUT.trim()), 3579328);
    }
}
