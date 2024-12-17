use itertools::{intersperse, Itertools};
use regex::Regex;
use std::ops::BitXor;

advent_of_code::solution!(17);

#[derive(Debug, Clone)]
struct Input {
    reg_a: i32,
    reg_b: i32,
    reg_c: i32,
    program: Vec<u8>,
}

fn parse_input(input: &str) -> Option<Input> {
    let re = Regex::new(
        r"(Register A: (?<a>\d+)\nRegister B: (?<b>\d+)\nRegister C: (?<c>\d+)\n\nProgram: (?<input>(\d+,?)+))",
    ).ok()?;

    let caps = re.captures(input)?;
    let reg_a = caps.name("a")?.as_str().parse::<i32>().ok()?;
    let reg_b = caps.name("b")?.as_str().parse::<i32>().ok()?;
    let reg_c = caps.name("c")?.as_str().parse::<i32>().ok()?;
    let program = caps
        .name("input")?
        .as_str()
        .split(",")
        .filter_map(|x| x.parse::<u8>().ok())
        .collect::<Vec<_>>();
    Some(Input {
        reg_a,
        reg_b,
        reg_c,
        program,
    })
}

#[derive(Debug, Clone)]
struct VirtualMachine {
    reg_a: i32,
    reg_b: i32,
    reg_c: i32,
    pc: usize,
    program: Vec<u8>,
}

impl VirtualMachine {
    fn with_input(input: Input) -> VirtualMachine {
        VirtualMachine::new(input.reg_a, input.reg_b, input.reg_c, input.program)
    }

    fn new(reg_a: i32, reg_b: i32, reg_c: i32, program: Vec<u8>) -> VirtualMachine {
        VirtualMachine {
            reg_a,
            reg_b,
            reg_c,
            pc: 0,
            program,
        }
    }
}

impl Iterator for VirtualMachine {
    type Item = Option<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        let combo_op = |x: u8| match x {
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => x as i32,
        };

        let opcode = self.program.get(self.pc)?;
        let value = self.program.get(self.pc + 1)?;
        let mut output = None;
        let mut jumped = false;

        match (opcode, value) {
            (0, n) => {
                let v = combo_op(*n);
                let divisor = i32::pow(2, if v < 0 { 0 } else { v as u32 });
                self.reg_a = self.reg_a / divisor;
            }
            (1, n) => {
                self.reg_b = self.reg_b.bitxor(*n as i32);
            }
            (2, n) => {
                let v = combo_op(*n);
                self.reg_b = v.rem_euclid(8);
            }
            (3, n) => {
                if self.reg_a != 0 {
                    self.pc = *n as usize;
                    jumped = true;
                }
            }
            (4, _n) => {
                self.reg_b = self.reg_b.bitxor(self.reg_c);
            }
            (5, n) => {
                output = Some(combo_op(*n).rem_euclid(8));
            }
            (6, n) => {
                let v = combo_op(*n);
                let divisor = i32::pow(2, if v < 0 { 0 } else { v as u32 });
                self.reg_b = self.reg_a / divisor;
            }
            (7, n) => {
                let v = combo_op(*n);
                let divisor = i32::pow(2, if v < 0 { 0 } else { v as u32 });
                self.reg_c = self.reg_a / divisor;
            }
            _ => panic!("unknown opcode: {opcode}"),
        }

        if !jumped {
            self.pc += 2;
        }

        Some(output)
    }
}

fn run_vm(vm: VirtualMachine) -> String {
    vm.filter_map(|x| x.map(|n| n.to_string()))
        .intersperse(String::from(","))
        .collect::<Vec<_>>()
        .concat()
}

pub fn part_one(input: &str) -> Option<String> {
    let input = parse_input(input).unwrap();
    let vm = VirtualMachine::with_input(input);
    let output = run_vm(vm);
    Some(output)
}

pub fn part_two(_input: &str) -> Option<i32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("4,6,3,5,6,3,5,2,1,0")));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
