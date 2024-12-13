use core::ops::Add;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;

advent_of_code::solution!(13);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: u32,
    y: u32,
}

impl Position {
    fn scale(&self, factor: u32) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Machine {
    vec_a: Position,
    vec_b: Position,
    prize: Position,
}

fn parse_input(input: &str) -> Vec<Machine> {
    let re = Regex::new(r"(Button A: X\+(?<ax>\d+), Y\+(?<ay>\d+)\nButton B: X\+(?<bx>\d+), Y\+(?<by>\d+)\nPrize: X=(?<px>\d+), Y=(?<py>\d+))").unwrap();
    re.captures_iter(input)
        .map(|caps| {
            let ax = caps.name("ax")?.as_str().parse::<u32>().ok()?;
            let ay = caps.name("ay")?.as_str().parse::<u32>().ok()?;
            let bx = caps.name("bx")?.as_str().parse::<u32>().ok()?;
            let by = caps.name("by")?.as_str().parse::<u32>().ok()?;
            let px = caps.name("px")?.as_str().parse::<u32>().ok()?;
            let py = caps.name("py")?.as_str().parse::<u32>().ok()?;
            Some(Machine {
                vec_a: Position { x: ax, y: ay },
                vec_b: Position { x: bx, y: by },
                prize: Position { x: px, y: py },
            })
        })
        .flatten()
        .collect()
}

/*
    While this is a graph problem looking at the nodes in the resulting graph looks like this:

                      Depth
           0        --- 0
         /   \
        A     B     --- 1
      /   \ /   \
    A+A   A+B   B+B --- 2
  ...  ...    ...  ...

    For each depth d there are exactly d+1 unique possibilities for sums with d summands.
    The sums are formed by producing all combinations of As and Bs as summands. Therefore,
    we can just iterate through these sums on each depth.

    Since B is cheaper than A, the first solution we find is optimal if we first look at
    the sums that contain more Bs than As.
*/
fn prize_cost(machine: &Machine) -> Option<u32> {
    /*
       This finds a minimum starting depth, to make a solution possible.
    */
    let min_depth = {
        if machine.prize.x < machine.prize.y {
            if machine.vec_a.x > machine.vec_b.x {
                machine.prize.x / machine.vec_a.x
            } else {
                machine.prize.x / machine.vec_b.x
            }
        } else {
            if machine.vec_a.y > machine.vec_b.y {
                machine.prize.y / machine.vec_a.y
            } else {
                machine.prize.y / machine.vec_b.y
            }
        }
    };
    for depth in min_depth..10000 {
        let mut all_greater = true;
        for i in (0..=depth).rev() {
            let times_a = depth - i;
            let times_b = i;
            let final_pos = machine.vec_a.scale(times_a) + machine.vec_b.scale(times_b);
            if final_pos == machine.prize {
                return Some(3 * times_a + 1 * times_b);
            } else if final_pos.x < machine.prize.x || final_pos.y < machine.prize.y {
                all_greater = false
            }
        }
        /*
            If all positions found were greater than the wanted position, there is no use in
            searching for more as the following positions will always increase in magnitude.
        */
        if all_greater {
            break;
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let machines = parse_input(input);
    Some(
        machines
            .par_iter()
            .map(|machine| prize_cost(machine).unwrap_or(0))
            .sum(),
    )
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
        assert_eq!(result, Some(480));
    }
}
