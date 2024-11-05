use anyhow::{bail, Result};
use macros::aoc;

#[aoc(Part::One)]
pub fn solve_one(input: &str) -> Result<i64> {
    let mut count: i64 = 0;
    for c in input.chars() {
        if c == '(' {
            count += 1;
        } else {
            count -= 1;
        }
    }

    Ok(count)
}

#[aoc(Part::Two)]
pub fn solve_two(input: &str) -> Result<i64> {
    let mut count: i64 = 0;
    for (i, c) in input.chars().enumerate() {
        if c == '(' {
            count += 1;
        } else {
            count -= 1;
        }

        if count < 0 {
            return Ok((i + 1) as i64);
        }
    }

    bail!("Santa didn't reach the basement")
}
