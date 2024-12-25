use aoc_runner_derive::{aoc, aoc_generator};
use derive_more::derive::Display;

use crate::set;

#[derive(Debug, PartialEq, Clone, Copy, Display)]
enum Direction {
    #[display("^")]
    Up,
    #[display("v")]
    Down,
    #[display(">")]
    Left,
    #[display("<")]
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '>' => Direction::Right,
            '<' => Direction::Left,
            ch => panic!("This is not a valid direction: {}", ch),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy, Display, Hash, Eq)]
#[display("({x}, {y})")]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn inplace_move(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
}

#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<Direction> {
    input.chars().map(Direction::from).collect()
}

#[aoc(day3, part1)]
fn solve_one(directions: &[Direction]) -> usize {
    let mut position = Position::default();
    let mut visited_houses = set![position];

    for direction in directions {
        position.inplace_move(direction);
        visited_houses.insert(position);
    }

    visited_houses.len()
}

#[aoc(day3, part2)]
fn solve_two(directions: &[Direction]) -> usize {
    let mut santa_position = Position::default();
    let mut robo_santa_position = Position::default();

    let mut visited_houses = set![santa_position];

    for (i, direction) in directions.iter().enumerate() {
        if i % 2 == 0 {
            santa_position.inplace_move(direction);
            visited_houses.insert(santa_position);
        } else {
            robo_santa_position.inplace_move(direction);
            visited_houses.insert(robo_santa_position);
        }
    }

    visited_houses.len()
}

// I implemented this challenge before I actually started writing in Rust,
// so I only tested the real solution.
#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::input;

    use super::*;

    const INPUT: &str = input!("day3");

    #[test]
    fn integration_part_one() {
        assert_eq!(solve_one(&parse(INPUT)), 2572);
    }

    #[test]
    fn integration_part_two() {
        assert_eq!(solve_two(&parse(INPUT)), 2631);
    }
}
