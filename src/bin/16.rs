use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::u32;

advent_of_code::solution!(16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2 {
    x: u32,
    y: u32,
}

impl Vec2 {
    fn neighbors(&self, direction: Direction) -> [Vec2; 2] {
        match direction {
            Direction::Horizontal => self.horizontal_neighbors(),
            Direction::Vertical => self.vertical_neighbors(),
        }
    }

    fn vertical_neighbors(&self) -> [Vec2; 2] {
        [
            Vec2 {
                x: self.x,
                y: self.y + 1,
            },
            Vec2 {
                x: self.x,
                y: self.y - 1,
            },
        ]
    }

    fn horizontal_neighbors(&self) -> [Vec2; 2] {
        [
            Vec2 {
                x: self.x + 1,
                y: self.y,
            },
            Vec2 {
                x: self.x - 1,
                y: self.y,
            },
        ]
    }
}

#[derive(Debug, Clone)]
struct Maze {
    start: Vec2,
    end: Vec2,
    paths: HashSet<Vec2>,
}

fn parse_input(input: &str) -> Maze {
    let mut paths = HashSet::new();
    let mut start_opt = None;
    let mut end_opt = None;
    for (y, line) in input.split("\n").enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = Vec2 {
                x: x as u32,
                y: y as u32,
            };
            match c {
                'S' => {
                    start_opt = Some(pos);
                }
                'E' => {
                    end_opt = Some(pos);
                }
                '.' => {
                    paths.insert(pos);
                    ()
                }
                _ => (),
            }
        }
    }
    Maze {
        start: start_opt.unwrap(),
        end: end_opt.unwrap(),
        paths,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Field {
    position: Vec2,
    horizontal_cost: u32,
    vertical_cost: u32,
}

impl Field {
    fn new(position: Vec2) -> Field {
        Field {
            position,
            horizontal_cost: u32::MAX,
            vertical_cost: u32::MAX,
        }
    }

    fn cost(&self, direction: Direction) -> u32 {
        match direction {
            Direction::Horizontal => self.horizontal_cost,
            Direction::Vertical => self.vertical_cost,
        }
    }

    fn with_cost(&mut self, cost: u32, direction: Direction) -> Field {
        self.update(cost, direction);
        *self
    }

    fn update(&mut self, cost: u32, direction: Direction) -> bool {
        match direction {
            Direction::Horizontal => {
                if cost < self.horizontal_cost {
                    self.horizontal_cost = cost;
                    let new_vert_cost = cost + 1000;
                    if new_vert_cost < self.vertical_cost {
                        self.vertical_cost = new_vert_cost
                    }
                    true
                } else {
                    false
                }
            }
            Direction::Vertical => {
                if cost < self.vertical_cost {
                    self.vertical_cost = cost;
                    let new_hori_cost = cost + 1000;
                    if new_hori_cost < self.horizontal_cost {
                        self.horizontal_cost = new_hori_cost
                    }
                    true
                } else {
                    false
                }
            }
        }
    }
}

fn flood_maze(maze: Maze) -> HashMap<Vec2, Field> {
    let mut frontier = VecDeque::new();
    frontier.push_front(maze.start);

    let mut visits: HashMap<Vec2, Field> = maze
        .paths
        .into_iter()
        .map(|position| (position, Field::new(position)))
        .collect();
    visits.insert(
        maze.start,
        Field::new(maze.start).with_cost(0, Direction::Horizontal),
    );
    visits.insert(maze.end, Field::new(maze.end));

    while let Some(pos) = frontier.pop_front() {
        let cost = visits.get(&pos).unwrap().cost(Direction::Horizontal);
        for nbr in pos.neighbors(Direction::Horizontal) {
            visits.entry(nbr).and_modify(|nbr| {
                let updated = nbr.update(
                    cost.checked_add(1).unwrap_or(u32::MAX),
                    Direction::Horizontal,
                );
                if updated {
                    frontier.push_back(nbr.position);
                }
            });
        }

        let cost = visits.get(&pos).unwrap().cost(Direction::Vertical);
        for nbr in pos.neighbors(Direction::Vertical) {
            visits.entry(nbr).and_modify(|nbr| {
                let updated =
                    nbr.update(cost.checked_add(1).unwrap_or(u32::MAX), Direction::Vertical);
                if updated {
                    frontier.push_back(nbr.position);
                }
            });
        }
    }

    visits
}

pub fn part_one(input: &str) -> Option<u32> {
    let maze = parse_input(input);
    let end_pos = maze.end.clone();
    let visits = flood_maze(maze);
    visits.get(&end_pos).map(|field| {
        u32::min(
            field.cost(Direction::Horizontal),
            field.cost(Direction::Vertical),
        )
    })
}

pub fn part_two(input: &str) -> Option<u32> {
    let maze = parse_input(input);
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_field() {
        let mut field = Field::new(Vec2 { x: 0, y: 0 });
        field.update(100, Direction::Horizontal);
        assert_eq!(field.cost(Direction::Horizontal), 100);
        assert_eq!(field.cost(Direction::Vertical), 1100);
        field.update(110, Direction::Horizontal);
        assert_eq!(field.cost(Direction::Horizontal), 100);
        assert_eq!(field.cost(Direction::Vertical), 1100);
        field.update(110, Direction::Vertical);
        assert_eq!(field.cost(Direction::Horizontal), 100);
        assert_eq!(field.cost(Direction::Vertical), 110);
    }

    #[test]
    fn test_examples() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(1004));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(3008));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(7036));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, None);
    }
}
