use regex::Regex;
use std::collections::HashMap;

advent_of_code::solution!(24);

#[derive(Debug)]
enum Operation {
    And,
    Or,
    Xor,
}

#[derive(Debug)]
struct Thunk {
    inputs: (String, String),
    op: Operation,
}

#[derive(Debug)]
enum Comp {
    Thunk(Thunk),
    Value(bool),
}

impl Comp {
    fn eval(&self, vals: &HashMap<String, Comp>) -> bool {
        match self {
            Comp::Value(val) => *val,
            Comp::Thunk(Thunk { inputs, op }) => {
                let (name1, name2) = inputs;
                let input1 = vals.get(name1).unwrap().eval(vals);
                let input2 = vals.get(name2).unwrap().eval(vals);
                match op {
                    Operation::And => input1 && input2,
                    Operation::Or => input1 || input2,
                    Operation::Xor => input1 ^ input2,
                }
            }
        }
    }
}

fn parse_input_for_part_one(input: &str) -> HashMap<String, Comp> {
    let re = Regex::new(r"(?<constant>(?<cn>[0-9a-zA-Z]+): (?<cv>[1,0]))|((?<a>[0-9a-zA-Z]+) (?<op>AND|OR|XOR) (?<b>[0-9a-zA-Z]+) -> (?<t>[0-9a-zA-Z]+))").unwrap();
    re.captures_iter(input)
        .map(|caps| {
            if caps.name("constant").is_some() {
                let name = caps.name("cn").unwrap().as_str().to_string();
                let value = caps.name("cv").unwrap().as_str();
                let boolval = if value == "1" { true } else { false };
                (name, Comp::Value(boolval))
            } else {
                let name1 = caps.name("a").unwrap().as_str().to_string();
                let name2 = caps.name("b").unwrap().as_str().to_string();
                let name3 = caps.name("t").unwrap().as_str().to_string();
                let op = match caps.name("op").unwrap().as_str() {
                    "AND" => Operation::And,
                    "OR" => Operation::Or,
                    "XOR" => Operation::Xor,
                    x => panic!("unknown op: {x}"),
                };
                (
                    name3,
                    Comp::Thunk(Thunk {
                        inputs: (name1, name2),
                        op,
                    }),
                )
            }
        })
        .collect::<HashMap<_, _>>()
}

pub fn part_one(input: &str) -> Option<u64> {
    let vals = parse_input_for_part_one(input);
    let zkeys = vals.keys().cloned().filter(|n| n.starts_with("z"));
    let mut zvals = Vec::new();
    for key in zkeys {
        let thunk = vals.get(&key).unwrap();
        let val = thunk.eval(&vals);
        zvals.push((key, val));
    }
    zvals.sort_by(|(name1, _), (name2, _)| name1.cmp(&name2));
    let result = zvals
        .into_iter()
        .enumerate()
        .map(
            |(i, (_, boolval))| {
                if boolval {
                    u64::pow(2, i as u32)
                } else {
                    0
                }
            },
        )
        .sum();

    Some(result)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
