use aoc_runner_derive::aoc;

#[aoc(day4, part1)]
fn solve_one(input: &str) -> usize {
    for i in 0.. {
        let hash = format!("{:x}", md5::compute(format!("{}{}", input, i)));
        if hash.starts_with("00000") {
            return i;
        }
    }
    panic!("No solution found");
}

#[aoc(day4, part2)]
fn solve_two(input: &str) -> usize {
    for i in 0.. {
        let hash = format!("{:x}", md5::compute(format!("{}{}", input, i)));
        if hash.starts_with("000000") {
            return i;
        }
    }
    panic!("No solution found");
}

// I implemented this challenge before I actually started writing in Rust,
// so I only tested the real solution.
#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::input;

    use super::*;

    const INPUT: &str = input!("day4");

    #[test]
    fn integration_part_one() {
        assert_eq!(solve_one(INPUT.trim()), 254575);
    }

    #[test]
    fn integration_part_two() {
        assert_eq!(solve_two(INPUT.trim()), 1038736);
    }
}
