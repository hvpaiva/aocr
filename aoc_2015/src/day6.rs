use aoc_runner_derive::aoc;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, space1, u64},
    combinator::map,
    sequence::{separated_pair, tuple},
    IResult,
};

struct Point {
    x: usize,
    y: usize,
}

struct Command {
    instruction: Instruction,
    coordinate: Coordinate,
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        map(tag("turn on"), |_| Instruction::TurnOn),
        map(tag("turn off"), |_| Instruction::TurnOff),
        map(tag("toggle"), |_| Instruction::Toggle),
    ))(input)
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    let (input, (x, y)) = separated_pair(u64, char(','), u64)(input)?;
    Ok((
        input,
        Point {
            x: x as usize,
            y: y as usize,
        },
    ))
}

impl Command {
    fn from<'a>(s: impl Into<&'a str>) -> Self {
        let input = s.into();
        let (_, (instruction, _, from, _, _, _, to)) = tuple((
            parse_instruction,
            space1,
            parse_point,
            space1,
            tag("through"),
            space1,
            parse_point,
        ))(input)
        .unwrap();

        Self {
            instruction,
            coordinate: Coordinate { from, to },
        }
    }
}

enum Instruction {
    TurnOn,
    TurnOff,
    Toggle,
}

struct Coordinate {
    from: Point,
    to: Point,
}

#[derive(Clone, Copy)]
enum LightKind {
    Dimmer,
    Default,
}

impl Default for LightKind {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(Clone, Copy, Default)]
struct Light {
    kind: LightKind,
    level: usize,
}

impl Light {
    fn new(kind: LightKind) -> Self {
        Self { kind, level: 0 }
    }

    fn apply(&mut self, instruction: &Instruction) {
        match self.kind {
            LightKind::Dimmer => match instruction {
                Instruction::TurnOn => self.level += 1,
                Instruction::TurnOff => self.level = self.level.saturating_sub(1),
                Instruction::Toggle => self.level += 2,
            },
            LightKind::Default => match instruction {
                Instruction::TurnOn => self.level = 1,
                Instruction::TurnOff => self.level = 0,
                Instruction::Toggle => self.level = 1 - self.level,
            },
        }
    }
}

struct Grid {
    flatten: Vec<Light>,
    size: usize,
}

impl Grid {
    fn new(light_kind: LightKind) -> Self {
        Self::new_with_size(light_kind, 1000)
    }

    fn new_with_size(light_kind: LightKind, size: usize) -> Self {
        Self {
            flatten: vec![Light::new(light_kind); size * size],
            size,
        }
    }

    fn find(&mut self, x: usize, y: usize) -> &mut Light {
        &mut self.flatten[x + y * self.size]
    }

    fn apply(&mut self, command: &Command) {
        for x in command.coordinate.from.x..=command.coordinate.to.x {
            for y in command.coordinate.from.y..=command.coordinate.to.y {
                let light = self.find(x, y);
                light.apply(&command.instruction);
            }
        }
    }

    fn sum(&self) -> usize {
        self.flatten.iter().map(|light| light.level).sum()
    }
}

#[aoc(day6, part1)]
fn solve_one(input: &str) -> usize {
    let mut grid = Grid::new(LightKind::Default);
    let commands = input.lines().map(Command::from);
    commands
        .into_iter()
        .for_each(|command| grid.apply(&command));

    grid.sum()
}

#[aoc(day6, part2)]
fn solve_two(input: &str) -> usize {
    let mut grid = Grid::new(LightKind::Dimmer);
    let commands = input.lines().map(Command::from);
    commands
        .into_iter()
        .for_each(|command| grid.apply(&command));

    grid.sum()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::input;

    use super::*;

    const INPUT: &str = input!("day6");

    #[test]
    fn integration_part_one() {
        assert_eq!(solve_one(INPUT), 543903);
    }

    #[test]
    fn integration_part_two() {
        assert_eq!(solve_two(INPUT), 14687245);
    }

    #[test]
    fn parse_turn_on_command() {
        let cmd = Command::from("turn on 0,0 through 999,999");
        assert!(matches!(cmd.instruction, Instruction::TurnOn));
        assert_eq!(cmd.coordinate.from.x, 0);
        assert_eq!(cmd.coordinate.from.y, 0);
        assert_eq!(cmd.coordinate.to.x, 999);
        assert_eq!(cmd.coordinate.to.y, 999);
    }

    #[test]
    fn parse_turn_off_command() {
        let cmd = Command::from("turn off 10,20 through 100,200");
        assert!(matches!(cmd.instruction, Instruction::TurnOff));
        assert_eq!(cmd.coordinate.from.x, 10);
        assert_eq!(cmd.coordinate.from.y, 20);
        assert_eq!(cmd.coordinate.to.x, 100);
        assert_eq!(cmd.coordinate.to.y, 200);
    }

    #[test]
    fn parse_toggle_command() {
        let cmd = Command::from("toggle 5,5 through 15,15");
        assert!(matches!(cmd.instruction, Instruction::Toggle));
        assert_eq!(cmd.coordinate.from.x, 5);
        assert_eq!(cmd.coordinate.from.y, 5);
        assert_eq!(cmd.coordinate.to.x, 15);
        assert_eq!(cmd.coordinate.to.y, 15);
    }

    #[test]
    fn light_default_turn_on() {
        let mut light = Light::new(LightKind::Default);
        assert_eq!(light.level, 0);

        light.apply(&Instruction::TurnOn);
        assert_eq!(light.level, 1);
    }

    #[test]
    fn light_default_turn_off() {
        let mut light = Light {
            kind: LightKind::Default,
            level: 1,
        };
        light.apply(&Instruction::TurnOff);
        assert_eq!(light.level, 0);
    }

    #[test]
    fn light_default_toggle() {
        let mut light = Light {
            kind: LightKind::Default,
            level: 0,
        };
        light.apply(&Instruction::Toggle);
        assert_eq!(light.level, 1);

        light.apply(&Instruction::Toggle);
        assert_eq!(light.level, 0);
    }

    #[test]
    fn light_dimmer_turn_on() {
        let mut light = Light::new(LightKind::Dimmer);
        assert_eq!(light.level, 0);

        light.apply(&Instruction::TurnOn);
        assert_eq!(light.level, 1);
    }

    #[test]
    fn light_dimmer_turn_off_saturate() {
        let mut light = Light::new(LightKind::Dimmer);

        light.apply(&Instruction::TurnOff);
        assert_eq!(light.level, 0);
    }

    #[test]
    fn light_dimmer_toggle() {
        let mut light = Light {
            kind: LightKind::Dimmer,
            level: 5,
        };
        light.apply(&Instruction::Toggle);
        assert_eq!(light.level, 7);
    }

    #[test]
    fn grid_turn_on_small_region() {
        let mut grid = Grid::new_with_size(LightKind::Default, 3);

        let cmd = Command::from("turn on 0,0 through 1,1");
        grid.apply(&cmd);

        for x in 0..=1 {
            for y in 0..=1 {
                assert_eq!(grid.find(x, y).level, 1, "({x},{y}) should be ON");
            }
        }

        assert_eq!(grid.find(2, 2).level, 0);
    }

    #[test]
    fn grid_turn_off_small_region() {
        let mut grid = Grid::new_with_size(LightKind::Default, 3);

        let cmd_on = Command::from("turn on 0,0 through 1,1");
        grid.apply(&cmd_on);

        let cmd_off = Command::from("turn off 1,1 through 2,2");
        grid.apply(&cmd_off);

        assert_eq!(grid.find(1, 1).level, 0);

        assert_eq!(grid.find(2, 2).level, 0);

        assert_eq!(grid.find(0, 0).level, 1);
    }

    #[test]
    fn grid_toggle_small_region() {
        let mut grid = Grid::new_with_size(LightKind::Default, 3);

        let cmd_toggle = Command::from("toggle 0,0 through 1,0");
        grid.apply(&cmd_toggle);

        assert_eq!(grid.find(0, 0).level, 1);
        assert_eq!(grid.find(1, 0).level, 1);

        grid.apply(&cmd_toggle);
        assert_eq!(grid.find(0, 0).level, 0);
        assert_eq!(grid.find(1, 0).level, 0);
    }

    #[test]
    fn grid_dimmer_turn_off_at_zero() {
        let mut grid = Grid::new_with_size(LightKind::Dimmer, 3);

        let cmd_off = Command::from("turn off 0,0 through 0,0");
        grid.apply(&cmd_off);
        assert_eq!(grid.find(0, 0).level, 0);

        let cmd_on = Command::from("turn on 0,0 through 0,0");
        grid.apply(&cmd_on);
        assert_eq!(grid.find(0, 0).level, 1);

        grid.apply(&cmd_off);
        assert_eq!(grid.find(0, 0).level, 0);
    }

    #[test]
    fn solve_one_small_input() {
        let input = r#"
turn on 0,0 through 1,1
toggle 0,0 through 0,0
turn off 1,1 through 1,1
"#;
        let result = solve_one(input.trim());
        // LightKind::Default:
        // 1) turn on 0..1,0..1 => 4 cells set to 1
        // 2) toggle 0..0,0..0 => (0,0) turns 1 to 0
        // 3) turn off 1..1,1..1 => (1,1) turns 1 to 0
        // Final: (0,0)=0, (0,1)=1, (1,0)=1, (1,1)=0 => total 2
        assert_eq!(result, 2);
    }

    #[test]
    fn solve_two_small_input() {
        let input = r#"
turn on 0,0 through 1,1
toggle 0,0 through 0,0
turn off 1,1 through 1,1
"#;
        let result = solve_two(input.trim());
        // LightKind::Dimmer:
        // 1) turn on => each cell level += 1 => 4 cells = 1
        // 2) toggle (0,0) => +2 => (0,0)=3, rest=1
        // 3) turn off (1,1) => saturating_sub(1) => (1,1)=0 (it was 1)
        // Final (0,0)=3, (0,1)=1, (1,0)=1, (1,1)=0 => total 5
        assert_eq!(result, 5);
    }
}
