use regex::Regex;
#[allow(unused_imports)]
use std::{collections::HashSet, io::stdin};

advent_of_code::solution!(14);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2 {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Robot {
    pos: Vec2,
    vel: Vec2,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Arena {
    size: Vec2,
    robots: Vec<Robot>,
}

impl Arena {
    fn safety_factor(&self) -> i32 {
        let x_mid = self.size.x / 2;
        let y_mid = self.size.y / 2;

        let (first_quadrant, second_quadrant, third_quadrant, fourth_quadrant) = self
            .robots
            .iter()
            .fold((0, 0, 0, 0), |(fst, snd, thr, fou), robot| {
                if robot.pos.x < x_mid && robot.pos.y < y_mid {
                    (fst + 1, snd, thr, fou)
                } else if robot.pos.x > x_mid && robot.pos.y < y_mid {
                    (fst, snd + 1, thr, fou)
                } else if robot.pos.x < x_mid && robot.pos.y > y_mid {
                    (fst, snd, thr + 1, fou)
                } else if robot.pos.x > x_mid && robot.pos.y > y_mid {
                    (fst, snd, thr, fou + 1)
                } else {
                    (fst, snd, thr, fou)
                }
            });
        first_quadrant * second_quadrant * third_quadrant * fourth_quadrant
    }

    #[allow(dead_code)]
    fn print_arena(&self) {
        let positions = self
            .robots
            .iter()
            .map(|robot| (robot.pos.x, robot.pos.y))
            .collect::<HashSet<_>>();

        for y in 0..self.size.y {
            for x in 0..self.size.x {
                if positions.contains(&(x, y)) {
                    print!("X")
                } else {
                    print!(" ")
                }
            }
            print!("\n");
        }
    }
}

fn parse_input(input: &str) -> Vec<Robot> {
    let re = Regex::new(r"(p=(?<px>\d+),(?<py>\d+) v=(?<vx>-?\d+),(?<vy>-?\d+))").unwrap();
    re.captures_iter(input)
        .map(|caps| {
            let px = caps.name("px")?.as_str().parse::<i32>().ok()?;
            let py = caps.name("py")?.as_str().parse::<i32>().ok()?;
            let vx = caps.name("vx")?.as_str().parse::<i32>().ok()?;
            let vy = caps.name("vy")?.as_str().parse::<i32>().ok()?;
            Some(Robot {
                pos: Vec2 { x: px, y: py },
                vel: Vec2 { x: vx, y: vy },
            })
        })
        .flatten()
        .collect()
}

fn simulate_robots(arena: Arena, seconds: i32) -> Arena {
    let new_robots = arena
        .robots
        .iter()
        .map(|robot| Robot {
            vel: robot.vel,
            pos: Vec2 {
                x: (robot.pos.x + (robot.vel.x * seconds)).rem_euclid(arena.size.x),
                y: (robot.pos.y + (robot.vel.y * seconds)).rem_euclid(arena.size.y),
            },
        })
        .collect();
    Arena {
        size: arena.size,
        robots: new_robots,
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let robots = parse_input(input);
    let arena = Arena {
        size: Vec2 { x: 101, y: 103 },
        robots,
    };
    let result = simulate_robots(arena, 100);
    Some(result.safety_factor())
}

pub fn part_two(_input: &str) -> Option<u32> {
    /*
       This is a fun one! I used the code below to generate representations of the arena.
       First, I went through time steps [0..inf) and realized that sometimes, a clump of
       robots would form. Then I checked if these clumps did repeat. It turns out that
       the placement of robots repeats after 10403 steps. Looking back a the clumps,
       they occurred at step 84 and 187, which is how the final code below was deduced.
       Running it (on my input) produces all of these clumps in quick succession. At
       some point the clump forms a christmas tree!
    */

    // let robots = parse_input(input);
    // let mut arena = Arena {
    //     size: Vec2 { x: 101, y: 103 },
    //     robots,
    // };
    // arena = simulate_robots(arena, 84);
    // let mut counter: u64 = 84;
    // loop {
    //     println!("\n\nStep {counter} ===\n");
    //     arena.print_arena();
    //     let mut buf = String::new();
    //     stdin().read_line(&mut buf).unwrap();
    //     arena = simulate_robots(arena, 103);
    //     counter += 103;
    // }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_robot_move() {
        let arena = Arena {
            size: Vec2 { x: 11, y: 7 },
            robots: vec![Robot {
                pos: Vec2 { x: 2, y: 4 },
                vel: Vec2 { x: 2, y: -3 },
            }],
        };
        let result = simulate_robots(arena, 5);
        assert_eq!(
            result.robots,
            vec![Robot {
                pos: Vec2 { x: 1, y: 3 },
                vel: Vec2 { x: 2, y: -3 },
            }]
        );
    }

    #[test]
    fn test_part_one() {
        let robots = parse_input(&advent_of_code::template::read_file("examples", DAY));
        let arena = Arena {
            size: Vec2 { x: 11, y: 7 },
            robots,
        };
        let result = simulate_robots(arena, 100);
        assert_eq!(result.safety_factor(), 12);
    }
}
