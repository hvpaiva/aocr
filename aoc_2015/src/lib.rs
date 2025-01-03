use aoc_runner_derive::aoc_lib;

mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub mod input;
pub mod set;

// Only for doc testing, 'cause why not?
pub use day8::Encode;

aoc_lib! { year = 2015; }
