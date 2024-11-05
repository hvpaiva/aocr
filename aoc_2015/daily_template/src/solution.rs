use anyhow::Result;
use macros::aoc;

#[aoc(Part::One)]
pub fn solve_one(input: &str) -> Result<i64> {
    Ok(input.len() as i64)
}

#[aoc(Part::Two)]
pub fn solve_two(input: &str) -> Result<i64> {
    Ok(input.lines().count() as i64)
}
