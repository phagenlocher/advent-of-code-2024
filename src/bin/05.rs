use regex::Regex;
use std::cmp::Ordering;
use std::collections::BTreeMap;

advent_of_code::solution!(5);

fn parse_input(input: &str) -> (BTreeMap<u8, Vec<u8>>, Vec<Vec<u8>>) {
    let mut rules: BTreeMap<u8, Vec<u8>> = BTreeMap::new();
    let mut updates = Vec::new();

    let re = Regex::new(r"((?<a>\d{2})\|(?<b>\d{2}))|(?<u>(\d{2},?)+)").unwrap();

    let captures = re.captures_iter(input);
    for caps in captures {
        match (caps.name("a"), caps.name("b"), caps.name("u")) {
            (Some(a), Some(b), _) => {
                let a_val = a.as_str().parse::<u8>().unwrap();
                let b_val = b.as_str().parse::<u8>().unwrap();
                rules
                    .entry(b_val)
                    .and_modify(|vals| vals.push(a_val))
                    .or_insert(vec![a_val]);
            }
            (_, _, Some(u)) => {
                let update = u
                    .as_str()
                    .split(",")
                    .map(|x| x.parse::<u8>().unwrap())
                    .collect();
                updates.push(update);
            }
            _ => {}
        }
    }

    (rules, updates)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, updates) = parse_input(input);
    let result = updates
        .iter()
        .filter_map(|update| {
            for (i, val) in update.iter().enumerate() {
                match rules.get(val) {
                    None => {}
                    Some(excludes) => {
                        for excluded_val in excludes {
                            if update[i + 1..].contains(excluded_val) {
                                return None;
                            }
                        }
                    }
                }
            }
            Some(update[update.len() / 2] as u32)
        })
        .sum();
    Some(result)
}

fn fix_update(rules: &BTreeMap<u8, Vec<u8>>, update: &Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = update.clone();

    result.sort_by(|x, y| match rules.get(x) {
        None => Ordering::Equal,
        Some(befores) => {
            if befores.contains(y) {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }
    });

    result
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, updates) = parse_input(input);
    let result = updates
        .iter()
        .filter_map(|update| {
            for (i, val) in update.iter().enumerate() {
                match rules.get(val) {
                    None => {}
                    Some(excludes) => {
                        for excluded_val in excludes {
                            if update[i + 1..].contains(excluded_val) {
                                let fixed = fix_update(&rules, &update);
                                return Some(fixed[fixed.len() / 2] as u32);
                            }
                        }
                    }
                }
            }
            None
        })
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
