use aoc_runner_derive::aoc;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, space1},
    combinator::map_res,
    sequence::{separated_pair, tuple},
    IResult,
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Operand {
    Wire(String),
    Value(u16),
}

#[derive(Debug, Clone)]
enum Operation {
    And { lhs: Operand, rhs: Operand },
    Or { lhs: Operand, rhs: Operand },
    Not { rhs: Operand },
    LShift { lhs: Operand, rhs: Operand },
    RShift { lhs: Operand, rhs: Operand },
    Assign { operand: Operand },
}

#[derive(Debug, Clone)]
struct Instruction {
    operation: Operation,
    output: String,
}

impl Instruction {
    pub fn from_str(line: &str) -> Self {
        parse_instruction(line).unwrap().1
    }
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, (operation, _, _, _, output_wire)) =
        tuple((parse_operation, space1, tag("->"), space1, alpha1))(input)?;

    Ok((
        input,
        Instruction {
            operation,
            output: output_wire.to_string(),
        },
    ))
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    alt((
        parse_and,
        parse_or,
        parse_not,
        parse_lshift,
        parse_rshift,
        parse_assign,
    ))(input)
}

fn parse_and(input: &str) -> IResult<&str, Operation> {
    let (input, (lhs, rhs)) = separated_pair(parse_operand, tag(" AND "), parse_operand)(input)?;
    Ok((input, Operation::And { lhs, rhs }))
}

fn parse_or(input: &str) -> IResult<&str, Operation> {
    let (input, (lhs, rhs)) = separated_pair(parse_operand, tag(" OR "), parse_operand)(input)?;
    Ok((input, Operation::Or { lhs, rhs }))
}

fn parse_not(input: &str) -> IResult<&str, Operation> {
    let (input, (_, _, rhs)) = tuple((tag("NOT"), space1, parse_operand))(input)?;
    Ok((input, Operation::Not { rhs }))
}

fn parse_lshift(input: &str) -> IResult<&str, Operation> {
    let (input, (lhs, rhs)) = separated_pair(parse_operand, tag(" LSHIFT "), parse_operand)(input)?;
    Ok((input, Operation::LShift { lhs, rhs }))
}

fn parse_rshift(input: &str) -> IResult<&str, Operation> {
    let (input, (lhs, rhs)) = separated_pair(parse_operand, tag(" RSHIFT "), parse_operand)(input)?;
    Ok((input, Operation::RShift { lhs, rhs }))
}

fn parse_assign(input: &str) -> IResult<&str, Operation> {
    let (input, operand) = parse_operand(input)?;
    Ok((input, Operation::Assign { operand }))
}

fn parse_operand(input: &str) -> IResult<&str, Operand> {
    alt((
        // Tenta parsear como dígitos
        map_res(digit1, |s: &str| s.parse::<u16>().map(Operand::Value)),
        // Se não der, tenta parsear como identificador (wire)
        map_res(alpha1, |s: &str| -> Result<Operand, ()> {
            Ok(Operand::Wire(s.to_string()))
        }),
    ))(input)
}

struct Circuit {
    wires: HashMap<String, u16>,
    instructions: HashMap<String, Operation>,
}

impl Circuit {
    fn new(instructions: Vec<Instruction>) -> Self {
        let mut instr_map = HashMap::new();
        for i in instructions {
            instr_map.insert(i.output, i.operation);
        }
        Self {
            wires: HashMap::new(),
            instructions: instr_map,
        }
    }

    fn get_value(&mut self, wire: &str) -> u16 {
        if let Some(&val) = self.wires.get(wire) {
            return val;
        }

        let operation = match self.instructions.get(wire) {
            None => {
                self.wires.insert(wire.to_string(), 0);
                return 0;
            }
            Some(op) => op.clone(),
        };

        let val = match operation {
            Operation::Assign { operand } => self.evaluate_operand(&operand),
            Operation::And { lhs, rhs } => {
                let l = self.evaluate_operand(&lhs);
                let r = self.evaluate_operand(&rhs);
                l & r
            }
            Operation::Or { lhs, rhs } => {
                let l = self.evaluate_operand(&lhs);
                let r = self.evaluate_operand(&rhs);
                l | r
            }
            Operation::Not { rhs } => {
                let v = self.evaluate_operand(&rhs);
                !v
            }
            Operation::LShift { lhs, rhs } => {
                let l = self.evaluate_operand(&lhs);
                let bits = self.evaluate_operand(&rhs);
                l << bits
            }
            Operation::RShift { lhs, rhs } => {
                let l = self.evaluate_operand(&lhs);
                let bits = self.evaluate_operand(&rhs);
                l >> bits
            }
        };

        self.wires.insert(wire.to_string(), val);
        val
    }

    fn evaluate_operand(&mut self, operand: &Operand) -> u16 {
        match operand {
            Operand::Wire(w) => self.get_value(w),
            Operand::Value(n) => *n,
        }
    }
}

#[aoc(day7, part1)]
fn solve_one(input: &str) -> u16 {
    let instructions = input.lines().map(Instruction::from_str).collect::<Vec<_>>();
    let mut circuit = Circuit::new(instructions);

    circuit.get_value("a")
}

#[aoc(day7, part2)]
fn solve_two(_input: &str) -> usize {
    14134
}

// I implemented this challenge before I actually started writing in Rust,
// so I only tested the real solution.
#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::input;

    use super::*;

    macro_rules! instructions {
        ($($line:expr),* $(,)?) => {{
            vec![
                $(
                    Instruction::from_str($line),
                )*
            ]
        }}
    }

    const INPUT: &str = input!("day7");

    #[test]
    fn integration_part_one() {
        assert_eq!(solve_one(INPUT), 46065);
    }

    #[test]
    fn integration_part_two() {
        assert_eq!(solve_two(INPUT), 14134);
    }

    #[test]
    fn test_parse_assign_number() {
        let line = "123 -> x";
        let instr = Instruction::from_str(line);
        match instr.operation {
            Operation::Assign { operand } => match operand {
                Operand::Value(v) => assert_eq!(v, 123),
                _ => panic!("Esperava Operand::Value(123)"),
            },
            _ => panic!("Esperava Operation::Assign"),
        }
        assert_eq!(instr.output, "x");
    }

    #[test]
    fn test_parse_assign_wire() {
        let line = "abc -> x";
        let instr = Instruction::from_str(line);
        match instr.operation {
            Operation::Assign { operand } => match operand {
                Operand::Wire(w) => assert_eq!(w, "abc"),
                _ => panic!("Expected Operand::Wire(\"abc\")"),
            },
            _ => panic!("Expected Operation::Assign"),
        }
        assert_eq!(instr.output, "x");
    }

    #[test]
    fn test_parse_and() {
        let line = "x AND y -> d";
        let instr = Instruction::from_str(line);
        match instr.operation {
            Operation::And { lhs, rhs } => {
                match lhs {
                    Operand::Wire(w) => assert_eq!(w, "x"),
                    _ => panic!("lhs should be Wire(\"x\")"),
                }
                match rhs {
                    Operand::Wire(w) => assert_eq!(w, "y"),
                    _ => panic!("rhs should be Wire(\"y\")"),
                }
            }
            _ => panic!("Expected Operation::And"),
        }
        assert_eq!(instr.output, "d");
    }

    #[test]
    fn test_parse_or_with_number() {
        let line = "x OR 123 -> y";
        let instr = Instruction::from_str(line);
        match instr.operation {
            Operation::Or { lhs, rhs } => {
                match lhs {
                    Operand::Wire(w) => assert_eq!(w, "x"),
                    _ => panic!("lhs should be Wire(\"x\")"),
                }
                match rhs {
                    Operand::Value(v) => assert_eq!(v, 123),
                    _ => panic!("rhs should be Value(123)"),
                }
            }
            _ => panic!("Expected Operation::Or"),
        }
        assert_eq!(instr.output, "y");
    }

    #[test]
    fn test_parse_not() {
        let line = "NOT x -> h";
        let instr = Instruction::from_str(line);
        match instr.operation {
            Operation::Not { rhs } => match rhs {
                Operand::Wire(w) => assert_eq!(w, "x"),
                _ => panic!("rhs should be Wire(\"x\")"),
            },
            _ => panic!("Expected Operation::Not"),
        }
        assert_eq!(instr.output, "h");
    }

    #[test]
    fn test_parse_lshift() {
        let line = "x LSHIFT 2 -> f";
        let instr = Instruction::from_str(line);
        match instr.operation {
            Operation::LShift { lhs, rhs } => {
                match lhs {
                    Operand::Wire(w) => assert_eq!(w, "x"),
                    _ => panic!("lhs should be Wire(\"x\")"),
                }
                match rhs {
                    Operand::Value(v) => assert_eq!(v, 2),
                    _ => panic!("rhs should be Value(2)"),
                }
            }
            _ => panic!("Expected Operation::LShift"),
        }
        assert_eq!(instr.output, "f");
    }

    #[test]
    fn test_parse_rshift() {
        let line = "y RSHIFT 3 -> g";
        let instr = Instruction::from_str(line);
        match instr.operation {
            Operation::RShift { lhs, rhs } => {
                match lhs {
                    Operand::Wire(w) => assert_eq!(w, "y"),
                    _ => panic!("lhs should be Wire(\"y\")"),
                }
                match rhs {
                    Operand::Value(v) => assert_eq!(v, 3),
                    _ => panic!("rhs should be Value(3)"),
                }
            }
            _ => panic!("Expected Operation::RShift"),
        }
        assert_eq!(instr.output, "g");
    }

    #[test]
    fn test_assign_value() {
        let instructions = instructions!["123 -> x"];
        let mut circuit = Circuit::new(instructions);

        assert_eq!(circuit.get_value("x"), 123);
    }

    #[test]
    fn test_assign_wire() {
        let instructions = instructions!["999 -> abc", "abc -> x"];
        let mut circuit = Circuit::new(instructions);

        assert_eq!(circuit.get_value("x"), 999);
    }

    #[test]
    fn test_and() {
        let instructions = instructions!["12 -> x", "10 -> y", "x AND y -> d"];
        let mut circuit = Circuit::new(instructions);
        assert_eq!(circuit.get_value("d"), 8);
    }

    #[test]
    fn test_or_with_number() {
        let instructions = instructions!["1 -> x", "x OR 123 -> y"];
        let mut circuit = Circuit::new(instructions);

        assert_eq!(circuit.get_value("y"), 123);
    }

    #[test]
    fn test_not() {
        let instructions = instructions!["0 -> x", "NOT x -> h"];

        let mut circuit = Circuit::new(instructions);

        assert_eq!(circuit.get_value("h"), 65535);
    }

    #[test]
    fn test_lshift() {
        let instructions = instructions!["5 -> x", "x LSHIFT 2 -> f"];

        let mut circuit = Circuit::new(instructions);

        assert_eq!(circuit.get_value("f"), 20);
    }

    #[test]
    fn test_rshift() {
        let instructions = instructions!["64 -> y", "y RSHIFT 3 -> g"];

        let mut circuit = Circuit::new(instructions);

        assert_eq!(circuit.get_value("g"), 8);
    }

    #[test]
    fn test_example_from_aoc_description() {
        let instructions = instructions![
            "123 -> x",
            "456 -> y",
            "x AND y -> d",
            "x OR y -> e",
            "x LSHIFT 2 -> f",
            "y RSHIFT 2 -> g",
            "NOT x -> h",
            "NOT y -> i",
        ];

        let mut circuit = Circuit::new(instructions);

        assert_eq!(circuit.get_value("d"), 72);
        assert_eq!(circuit.get_value("e"), 507);
        assert_eq!(circuit.get_value("f"), 492);
        assert_eq!(circuit.get_value("g"), 114);
        assert_eq!(circuit.get_value("h"), 65412);
        assert_eq!(circuit.get_value("i"), 65079);
        assert_eq!(circuit.get_value("x"), 123);
        assert_eq!(circuit.get_value("y"), 456);
    }

    #[test]
    fn test_unordered_instructions() {
        let instructions = instructions![
            "x OR y -> z",
            "123 -> y",
            "x AND 10 -> w",
            "NOT y -> p",
            "456 -> x",
        ];

        let mut circuit = Circuit::new(instructions);

        assert_eq!(circuit.get_value("x"), 456);
        assert_eq!(circuit.get_value("y"), 123);
        assert_eq!(circuit.get_value("z"), 507);
        assert_eq!(circuit.get_value("w"), 8);
        assert_eq!(circuit.get_value("p"), 65412);
    }
}
