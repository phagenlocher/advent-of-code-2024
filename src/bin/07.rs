use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(7);

struct MultiSetGenerator<E> {
    elems: Vec<E>,
    current_val: Vec<E>,
    yielded_first: bool,
}

impl<E: Ord + Clone> MultiSetGenerator<E> {
    fn new_of_size(elems: &Vec<E>, n: usize) -> MultiSetGenerator<E> {
        let mut elems_copy = elems.clone();
        elems_copy.dedup();
        elems_copy.sort();

        let first_opt = elems_copy.first();
        MultiSetGenerator {
            elems: elems_copy.clone(),
            current_val: match first_opt {
                Some(first) => vec![first.clone(); n],
                None => vec![],
            },
            yielded_first: false,
        }
    }

    fn next_elem_for(&self, elem: E) -> Option<&E> {
        if let Ok(index) = self.elems.binary_search(&elem) {
            self.elems.get(index + 1)
        } else {
            None
        }
    }
}

impl<E: Ord + Copy> Iterator for MultiSetGenerator<E> {
    type Item = Vec<E>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.yielded_first {
            self.yielded_first = true;
            return Some(self.current_val.clone());
        }
        for i in 0..self.current_val.len() {
            let elem = self.current_val[i];
            if let Some(succ) = self.next_elem_for(elem) {
                // Incr new elem
                self.current_val[i] = *succ;

                // Reset all others
                let first_elem = self.elems[0];
                for j in 0..i {
                    self.current_val[j] = first_elem;
                }

                return Some(self.current_val.clone());
            }
        }
        None
    }
}

struct Equation {
    result: u64,
    factors: Vec<u64>,
}

fn parse_equations(input: &str) -> Vec<Equation> {
    input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            if let [result_str, factor_str] = line.split(":").collect::<Vec<_>>()[..] {
                Equation {
                    result: result_str.parse::<u64>().unwrap(),
                    factors: factor_str
                        .split(" ")
                        .filter(|factor| !factor.is_empty())
                        .map(|factor| factor.parse::<u64>().unwrap())
                        .collect(),
                }
            } else {
                panic!("whoops: {line}")
            }
        })
        .collect()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Operation {
    Add,
    Mul,
    Con,
}

fn validate_equation(equation: &Equation, operations: &Vec<Operation>) -> bool {
    if let Some((first, rest)) = equation.factors.split_first() {
        for operations in MultiSetGenerator::new_of_size(operations, equation.factors.len() - 1) {
            let calc_result: u64 =
                rest.iter()
                    .zip(operations)
                    .fold(*first, |acc, (x, op)| match op {
                        Operation::Add => acc + x,
                        Operation::Mul => acc * x,
                        Operation::Con => {
                            let mut acc_str = acc.to_string();
                            let x_str = x.to_string();
                            acc_str.push_str(&x_str);
                            acc_str.parse::<u64>().unwrap()
                        }
                    });
            if calc_result == equation.result {
                return true;
            }
        }
    } else {
        return equation.result == 0;
    }
    false
}

fn solve_with(operations: &Vec<Operation>, input: &str) -> Option<u64> {
    let equations = parse_equations(input);
    Some(
        equations
            .par_iter()
            .filter_map(|equation| {
                if validate_equation(equation, operations) {
                    Some(equation.result)
                } else {
                    None
                }
            })
            .sum(),
    )
}

pub fn part_one(input: &str) -> Option<u64> {
    use Operation::*;
    solve_with(&vec![Add, Mul], input)
}

pub fn part_two(input: &str) -> Option<u64> {
    use Operation::*;
    solve_with(&vec![Add, Mul, Con], input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
        use Operation::*;
        let result: Vec<Vec<Operation>> =
            MultiSetGenerator::new_of_size(&vec![Add, Mul], 2).collect();
        let expected: Vec<Vec<Operation>> = vec![
            vec![Add, Add],
            vec![Mul, Add],
            vec![Add, Mul],
            vec![Mul, Mul],
        ];
        assert_eq!(result, expected)
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
