use std::{collections::HashSet, ops::AddAssign};

use anyhow::Result;
use macros::aoc;

#[aoc(Part::One)]
pub fn solve_one(input: &str) -> Result<i64> {
    let mut visited_houses = HashSet::with_capacity(input.len());
    let mut house = House::new();
    visited_houses.insert(house);

    input.chars().filter_map(Direction::parse).for_each(|dir| {
        house += dir.into();
        visited_houses.insert(house);
    });

    Ok(visited_houses.len().try_into()?)
}

#[aoc(Part::Two)]
pub fn solve_two(input: &str) -> Result<i64> {
    let mut visited_houses = HashSet::with_capacity(input.len());
    let mut santa_house = House::new();
    let mut robot_house = House::new();
    visited_houses.insert(santa_house);

    input
        .chars()
        .filter_map(Direction::parse)
        .enumerate()
        .for_each(|(i, dir)| {
            let current_house = if i % 2 == 0 {
                &mut santa_house
            } else {
                &mut robot_house
            };
            *current_house += dir.into();
            visited_houses.insert(*current_house);
        });

    Ok(visited_houses.len().try_into()?)
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct House(i16, i16);

impl House {
    fn new() -> House {
        House(0, 0)
    }
}

impl AddAssign for House {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
    }
}

impl From<Direction> for House {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::North => House(0, 1),
            Direction::South => House(0, -1),
            Direction::East => House(1, 0),
            Direction::West => House(-1, 0),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn parse(direction: char) -> Option<Direction> {
        match direction {
            '^' => Some(Self::North),
            'v' => Some(Self::South),
            '<' => Some(Self::West),
            '>' => Some(Self::East),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("^v", 2)]
    #[case("^>v<", 4)]
    #[case("^v^v^v^v^v", 2)]
    fn solve_one_test(#[case] input: &str, #[case] expected: i64) {
        assert_eq!(solve_one(input).unwrap(), expected);
    }

    #[rstest]
    #[case("^v", 3)]
    #[case("^>v<", 3)]
    #[case("^v^v^v^v^v", 11)]
    fn solve_two_test(#[case] input: &str, #[case] expected: i64) {
        assert_eq!(solve_two(input).unwrap(), expected);
    }

    #[rstest]
    #[case('^', Some(Direction::North))]
    #[case('v', Some(Direction::South))]
    #[case('<', Some(Direction::West))]
    #[case('>', Some(Direction::East))]
    #[case('t', None)]
    fn create_directions_test(#[case] input: char, #[case] expected: Option<Direction>) {
        assert_eq!(Direction::parse(input), expected);
    }
}
