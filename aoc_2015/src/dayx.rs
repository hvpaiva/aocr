use aoc_runner_derive::aoc;

#[aoc(dayx, part1)]
fn solve_one(input: &str) -> usize {
    input.len()
}

#[aoc(dayx, part2)]
fn solve_two(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::input;

    use super::*;

    const INPUT: &str = input!("dayx");

    #[test]
    #[ignore = "Not implemented"]
    fn integration_part_one() {
        assert_eq!(solve_one(INPUT), 0);
    }

    #[test]
    #[ignore = "Not implemented"]
    fn integration_part_two() {
        assert_eq!(solve_two(INPUT), 0);
    }
}
