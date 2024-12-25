use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Gift> {
    input.lines().map(Gift::new).collect()
}

#[derive(Debug, PartialEq)]
struct Gift {
    l: u32,
    w: u32,
    h: u32,
}

impl Gift {
    fn new(gift_str: &str) -> Self {
        let dims: Vec<u32> = gift_str
            .split('x')
            .map(|s| s.parse::<u32>().expect("Invalid dimension"))
            .collect();
        if dims.len() != 3 {
            panic!("Expected exactly 3 dimensions in the format LxWxH");
        }
        Gift {
            l: dims[0],
            w: dims[1],
            h: dims[2],
        }
    }

    fn sides(&self) -> [u32; 3] {
        [self.l * self.w, self.w * self.h, self.h * self.l]
    }

    fn smallest_side(&self) -> u32 {
        // The sides are never empty, so we can safely call `unwrap()`
        *self.sides().iter().min().unwrap()
    }

    fn area(&self) -> u32 {
        self.sides().iter().map(|side| side * 2).sum::<u32>() + self.smallest_side()
    }

    fn smallest_perimeter(&self) -> u32 {
        let mut dim = [self.l, self.w, self.h];
        dim.sort();
        2 * (dim[0] + dim[1])
    }

    fn volume(&self) -> u32 {
        self.l * self.w * self.h
    }
}

#[aoc(day2, part1)]
fn solve_one(input: &[Gift]) -> u32 {
    input.iter().fold(0, |acc, gift| acc + gift.area())
}

#[aoc(day2, part2)]
fn solve_two(input: &[Gift]) -> u32 {
    input.iter().fold(0, |acc, gift| {
        acc + gift.smallest_perimeter() + gift.volume()
    })
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::input;

    use super::*;

    // Macro just to shorten the code
    macro_rules! gift {
        ($dims:expr) => {{
            Gift::new($dims)
        }};
    }

    #[test]
    fn test_parse() {
        let input = "2x3x4\n1x1x10\n";
        let expected = vec![Gift { l: 2, w: 3, h: 4 }, Gift { l: 1, w: 1, h: 10 }];
        assert_eq!(parse(input), expected);
    }

    #[test]
    fn test_solve_one() {
        let input = [gift!("2x3x4"), gift!("1x1x10")];
        assert_eq!(solve_one(&input), 101);
    }

    #[test]
    fn test_solve_one_empty() {
        let input = [];
        assert_eq!(solve_one(&input), 0);
    }

    #[test]
    fn test_solve_two() {
        let input = [gift!("2x3x4"), gift!("1x1x10")];
        assert_eq!(solve_two(&input), 48);
    }

    const INPUT: &str = input!("day2");

    #[test]
    fn integration_part_one() {
        assert_eq!(solve_one(&parse(INPUT)), 1588178);
    }

    #[test]
    fn integration_part_two() {
        assert_eq!(solve_two(&parse(INPUT)), 3783758);
    }

    #[test]
    fn test_area() {
        assert_eq!(gift!("2x3x4").area(), 58);
        assert_eq!(gift!("1x1x10").area(), 43);
    }

    #[test]
    fn test_smallest_perimeter() {
        assert_eq!(gift!("2x3x4").smallest_perimeter(), 10);
        assert_eq!(gift!("1x1x10").smallest_perimeter(), 4);
    }

    #[test]
    fn test_volume() {
        assert_eq!(gift!("2x3x4").volume(), 24);
        assert_eq!(gift!("1x1x10").volume(), 10);
    }

    #[test]
    fn test_sides() {
        assert_eq!(gift!("2x3x4").sides(), [6, 12, 8]);
        assert_eq!(gift!("1x1x10").sides(), [1, 10, 10]);
    }

    #[test]
    fn test_smallest_side() {
        assert_eq!(gift!("2x3x4").smallest_side(), 6);
        assert_eq!(gift!("1x1x10").smallest_side(), 1);
    }

    #[test]
    #[should_panic(expected = "Invalid dimension: ParseIntError { kind: Empty }")]
    fn test_parse_wrong_format() {
        let input = "2x3x";
        let _ = Gift::new(input);
        let _ = gift!(input);
    }

    #[test]
    #[should_panic(expected = "Invalid dimension: ParseIntError { kind: InvalidDigit }")]
    fn test_parse_wrong_parse() {
        let input = "2x3xA";
        let _ = Gift::new(input);
        let _ = gift!(input);
    }

    #[test]
    #[should_panic(expected = "Invalid dimension: ParseIntError { kind: Empty }")]
    fn test_parse_empty() {
        let input = "";
        let _ = Gift::new(input);
        let _ = gift!(input);
    }

    #[test]
    #[should_panic(expected = "Expected exactly 3 dimensions in the format LxWxH")]
    fn test_parse_missing_dimensions() {
        let input = "2x3";
        let _ = Gift::new(input);
        let _ = gift!(input);
    }
}
