use std::collections::HashMap;
use std::collections::HashSet;

advent_of_code::solution!(12);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn neighbors(&self) -> [Position; 4] {
        [
            Position {
                x: self.x + 1,
                y: self.y,
            },
            Position {
                x: self.x - 1,
                y: self.y,
            },
            Position {
                x: self.x,
                y: self.y + 1,
            },
            Position {
                x: self.x,
                y: self.y - 1,
            },
        ]
    }

    fn is_neighbor(&self, other: &Self) -> bool {
        if self.x == other.x {
            [other.y - 1, other.y + 1].contains(&self.y)
        } else if self.y == other.y {
            [other.x - 1, other.x + 1].contains(&self.x)
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Plot {
    plant: char,
    positions: HashSet<Position>,
}

impl Plot {
    fn area(&self) -> usize {
        self.positions.len()
    }

    fn perimeter(&self) -> usize {
        self.positions
            .iter()
            .map(|a_pos| {
                let num_nbrs = self
                    .positions
                    .iter()
                    .filter(|b_pos| a_pos.is_neighbor(b_pos))
                    .count();
                4 - num_nbrs
            })
            .sum()
    }

    fn sides(&self) -> usize {
        /*
           The idea here is to seperate the positions that have a border in a certain direction
           (top, bottom, left, right) and then seperate them into sets of neighbors. The amount
           of sets is amount of the borders in that direction (with the length of the sets)
           being the length of each individual border.
        */
        let create_border_positions = |offset_x, offset_y| {
            self.positions
                .clone()
                .iter()
                .filter_map(|pos| {
                    if !self.positions.contains(&Position {
                        x: pos.x + offset_x,
                        y: pos.y + offset_y,
                    }) {
                        Some(*pos)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        };

        let top_lines = seperate_by_fill(create_border_positions(-1, 0)).len();
        let bot_lines = seperate_by_fill(create_border_positions(1, 0)).len();
        let lef_lines = seperate_by_fill(create_border_positions(0, -1)).len();
        let rig_lines = seperate_by_fill(create_border_positions(0, 1)).len();
        top_lines + bot_lines + lef_lines + rig_lines
    }
}

fn parse_input(input: &str) -> HashMap<char, Vec<Position>> {
    let mut result = HashMap::new();

    for (y, line) in input.split("\n").enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = Position {
                x: x as i32,
                y: y as i32,
            };
            result
                .entry(c)
                .and_modify(|poss: &mut Vec<Position>| poss.push(pos))
                .or_insert(vec![pos]);
        }
    }

    result
}

fn seperate_nbrs(pos: Position, positions: Vec<Position>) -> (HashSet<Position>, Vec<Position>) {
    let mut frontier = vec![pos];
    let mut positions: HashSet<Position> = positions.into_iter().collect();
    let mut result = HashSet::from([pos]);

    while let Some(pos) = frontier.pop() {
        for nbr in pos.neighbors() {
            if positions.remove(&nbr) {
                frontier.push(nbr);
                result.insert(nbr);
            }
        }
    }

    (result, positions.into_iter().collect())
}

fn seperate_by_fill(mut positions: Vec<Position>) -> Vec<HashSet<Position>> {
    let mut result = Vec::new();

    while let Some(pos) = positions.pop() {
        let (nbrs, rest) = seperate_nbrs(pos, positions);
        result.push(nbrs);
        positions = rest;
    }

    result
}

fn parse_plots(plots: HashMap<char, Vec<Position>>) -> Vec<Plot> {
    let mut result = Vec::new();

    for (plant, positions) in plots {
        for positions in seperate_by_fill(positions) {
            result.push(Plot { plant, positions });
        }
    }

    result
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = parse_input(input);
    let plots = parse_plots(map);
    Some(
        plots
            .iter()
            .map(|plot| plot.area() * plot.perimeter())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = parse_input(input);
    let plots = parse_plots(map);
    Some(plots.iter().map(|plot| plot.area() * plot.sides()).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plot_parsing() {
        let single_plots = parse_input(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        let plots = parse_plots(single_plots);
        let mut plants = plots.iter().map(|plot| plot.plant).collect::<Vec<_>>();
        plants.sort();
        assert_eq!(plants, vec!['A', 'B', 'C', 'D', 'E']);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(1206));
    }
}
