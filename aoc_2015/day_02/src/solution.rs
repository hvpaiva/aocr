use anyhow::Result;
use macros::aoc;

#[aoc(Part::One)]
pub fn solve_one(input: &str) -> Result<i64> {
    let total: i64 = input
        .lines()
        .flat_map(parse_box)
        .map(required_wrapping_paper)
        .sum();
    Ok(total)
}

#[aoc(Part::Two)]
pub fn solve_two(input: &str) -> Result<i64> {
    let total: i64 = input.lines().flat_map(parse_box).map(required_ribbon).sum();
    Ok(total)
}

#[derive(Debug, PartialEq)]
struct Box {
    length: i64,
    width: i64,
    height: i64,
}

fn parse_box(line: &str) -> Option<Box> {
    let mut dimensions = line
        .split('x')
        .map(str::parse::<i64>)
        .filter_map(Result::ok);
    Some(Box {
        length: dimensions.next()?,
        width: dimensions.next()?,
        height: dimensions.next()?,
    })
}

fn required_wrapping_paper(boxs: Box) -> i64 {
    let sides = &[
        boxs.length * boxs.width,
        boxs.width * boxs.height,
        boxs.height * boxs.length,
    ];
    let min_side = sides.iter().min().unwrap();
    let paper: i64 = sides.iter().map(|side| side * 2).sum();

    paper + min_side
}

fn required_ribbon(boxs: Box) -> i64 {
    let smallerst_perimeter: i64 = *[
        2 * (boxs.length + boxs.width),
        2 * (boxs.width + boxs.height),
        2 * (boxs.height + boxs.length),
    ]
    .iter()
    .min()
    .unwrap();

    let volume = boxs.length * boxs.width * boxs.height;

    smallerst_perimeter + volume
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[test]
    fn solve_one_test() {
        assert_eq!(solve_one("2x3x4\n1x1x10\n").unwrap(), 101);
    }

    #[test]
    fn solve_two_test() {
        assert_eq!(solve_two("2x3x4\n1x1x10\n").unwrap(), 48);
    }

    #[rstest]
    #[case(Box { length: 2, width: 3, height: 4 }, 58)]
    #[case(Box { length: 1, width: 1, height: 10 }, 43)]
    fn required_wrapping_paper_test(#[case] input: Box, #[case] expected: i64) {
        assert_eq!(required_wrapping_paper(input), expected);
    }

    #[rstest]
    #[case("2x3x4", Some(Box { length: 2, width: 3, height: 4 }))]
    #[case("1x1x10", Some(Box { length: 1, width: 1, height: 10 }))]
    #[case("1xax10", None)]
    #[case("txaxh", None)]
    #[case("xx", None)]
    #[case("", None)]
    fn parse_box_test(#[case] input: &str, #[case] expected: Option<Box>) {
        assert_eq!(parse_box(input), expected);
    }

    #[rstest]
    #[case(Box { length: 2, width: 3, height: 4 }, 34)]
    #[case(Box { length: 1, width: 1, height: 10 }, 14)]
    fn required_ribbon_test(#[case] input: Box, #[case] expected: i64) {
        assert_eq!(required_ribbon(input), expected);
    }
}
