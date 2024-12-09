use std::collections::HashMap;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(6);

#[derive(Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone)]
enum FieldType {
    Empty,
    Obstacle,
    Guard(Direction),
}

#[derive(Copy, Clone)]
struct Guard {
    direction: Direction,
    x_pos: i32,
    y_pos: i32,
    last_x_pos: i32,
    last_y_pos: i32,
}

impl Guard {
    fn new(direction: Direction, x_pos: i32, y_pos: i32) -> Guard {
        return Guard {
            direction,
            x_pos,
            y_pos,
            last_x_pos: x_pos,
            last_y_pos: y_pos,
        };
    }

    fn walk_one_step(&mut self) {
        self.last_x_pos = self.x_pos;
        self.last_y_pos = self.y_pos;
        match self.direction {
            Direction::Up => self.y_pos -= 1,
            Direction::Down => self.y_pos += 1,
            Direction::Left => self.x_pos -= 1,
            Direction::Right => self.x_pos += 1,
        }
    }

    fn reset_step(&mut self) {
        self.x_pos = self.last_x_pos;
        self.y_pos = self.last_y_pos;
    }

    fn turn_90_degrees(&mut self) {
        match self.direction {
            Direction::Up => self.direction = Direction::Right,
            Direction::Down => self.direction = Direction::Left,
            Direction::Left => self.direction = Direction::Up,
            Direction::Right => self.direction = Direction::Down,
        }
    }
}

fn build_map(input: &str) -> (HashMap<(i32, i32), FieldType>, Guard) {
    let mut map = HashMap::new();
    let mut guard = None;

    for (y, line) in input.split("\n").enumerate() {
        let fields = line.chars().map(|field| match field {
            '.' => FieldType::Empty,
            '#' => FieldType::Obstacle,
            '^' => FieldType::Guard(Direction::Up),
            'v' => FieldType::Guard(Direction::Down),
            '<' => FieldType::Guard(Direction::Left),
            '>' => FieldType::Guard(Direction::Right),
            _ => panic!("Unknown char in puzzle map: {field}\n"),
        });
        for (x, field) in fields.enumerate() {
            let corrected_field = match field {
                FieldType::Guard(direction) => {
                    guard = Some(Guard::new(direction, x as i32, y as i32));
                    FieldType::Empty
                }
                field => field,
            };

            map.entry((x as i32, y as i32)).or_insert(corrected_field);
        }
    }

    (
        map,
        guard.unwrap_or_else(|| panic!("No guard in puzzle map!")),
    )
}

enum WalkResult {
    OutOfMap(HashMap<(i32, i32), Vec<Direction>>),
    Cycle(HashMap<(i32, i32), Vec<Direction>>),
}

impl WalkResult {
    fn step_count(self) -> usize {
        self.seen_steps().len()
    }

    fn seen_steps(self) -> HashMap<(i32, i32), Vec<Direction>> {
        match self {
            Self::Cycle(steps) => steps,
            Self::OutOfMap(steps) => steps,
        }
    }
}

fn simulate_walking_path(puzzle_map: &HashMap<(i32, i32), FieldType>, guard: &Guard) -> WalkResult {
    let mut guard = guard.clone();
    let mut seen_map = HashMap::new();

    seen_map.insert((guard.x_pos, guard.y_pos), vec![guard.direction]);

    loop {
        guard.walk_one_step();

        match seen_map.get(&(guard.x_pos, guard.y_pos)) {
            None => {}
            Some(directions) => {
                if directions.contains(&guard.direction) {
                    // Found a cycle!
                    return WalkResult::Cycle(seen_map);
                }
            }
        }

        match puzzle_map.get(&(guard.x_pos, guard.y_pos)) {
            None => break, // Outside of map!
            Some(field) => match &field {
                FieldType::Obstacle => {
                    guard.reset_step();
                    guard.turn_90_degrees();
                    continue;
                }
                _ => {}
            },
        }

        seen_map
            .entry((guard.x_pos, guard.y_pos))
            .and_modify(|directions| directions.push(guard.direction))
            .or_insert(vec![guard.direction]);
    }

    WalkResult::OutOfMap(seen_map)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (puzzle_map, guard) = build_map(input);
    Some(simulate_walking_path(&puzzle_map, &guard).step_count())
}

fn count_cycle_causing_obstacles(
    puzzle_map: &HashMap<(i32, i32), FieldType>,
    guard: &Guard,
) -> usize {
    let (orig_x, orig_y) = (guard.x_pos, guard.y_pos);
    let seen_steps = simulate_walking_path(puzzle_map, guard).seen_steps();
    seen_steps
        .par_iter()
        .filter(|((x, y), _)| *x != orig_x || *y != orig_y)
        .map(|((x, y), _)| {
            let mut new_puzzle_map = puzzle_map.clone();
            new_puzzle_map
                .entry((*x, *y))
                .and_modify(|v| *v = FieldType::Obstacle);
            simulate_walking_path(&new_puzzle_map, &guard)
        })
        .filter(|walk_result| match *walk_result {
            WalkResult::Cycle(_) => true,
            WalkResult::OutOfMap(_) => false,
        })
        .count()
}

pub fn part_two(input: &str) -> Option<usize> {
    let (puzzle_map, guard) = build_map(input);
    Some(count_cycle_causing_obstacles(&puzzle_map, &guard))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
