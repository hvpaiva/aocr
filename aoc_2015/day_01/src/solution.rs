use anyhow::Result;
use macros::aoc;

#[aoc(Part::One)]
pub fn solve_one(input: &str) -> Result<u64> {
    Ok(input.len() as u64)
}

#[aoc(Part::Two)]
pub fn solve_two(input: &str) -> Result<u64> {
    Ok(input.lines().count() as u64)
}
