use cartesian::cartesian;
use std::collections::HashMap;
use std::collections::HashSet;

advent_of_code::solution!(8);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn get_antinode_for(&self, other: Self) -> Self {
        let diff_x = i64::abs(self.x - other.x);
        let diff_y = i64::abs(self.y - other.y);
        let new_x = if other.x > self.x {
            other.x + diff_x
        } else {
            other.x - diff_x
        };
        let new_y = if other.y > self.y {
            other.y + diff_y
        } else {
            other.y - diff_y
        };
        Position { x: new_x, y: new_y }
    }

    fn get_extended_antinodes_for(&self, other: Self, min: &Position, max: &Position) -> Vec<Self> {
        let mut result = Vec::new();
        let mut curr = self.clone();
        let mut other = other.clone();

        loop {
            let new = curr.get_antinode_for(other);
            if new.x > max.x || new.y > max.y || new.x < min.x || new.y < min.y {
                break;
            }
            result.push(new);
            curr = other;
            other = new;
        }

        result
    }
}

#[derive(Debug, Copy, Clone)]
struct Antenna {
    frequency: char,
    position: Position,
}

enum Field {
    AntennaField(Antenna),
    EmptyField,
}

fn get_fields(input: &str) -> HashMap<Position, Field> {
    let mut result = HashMap::new();

    for (y, line) in input.split("\n").enumerate() {
        for (x, field) in line.chars().enumerate() {
            let pos = Position {
                x: x as i64,
                y: y as i64,
            };
            if field.is_alphanumeric() {
                let antenna = Antenna {
                    frequency: field,
                    position: pos,
                };
                result.insert(pos, Field::AntennaField(antenna));
            } else {
                result.insert(pos, Field::EmptyField);
            }
        }
    }

    result
}

fn group_antennas_by_frequency(antennas: &Vec<Antenna>) -> HashMap<char, Vec<Antenna>> {
    let mut result = HashMap::new();

    for antenna in antennas {
        result
            .entry(antenna.frequency)
            .and_modify(|antens: &mut Vec<Antenna>| antens.push(*antenna))
            .or_insert(vec![*antenna]);
    }

    result
}

fn calc_antinodes(
    grouped_antennas: HashMap<char, Vec<Antenna>>,
    max_pos: &Position,
    extended: bool,
) -> HashSet<Position> {
    fn calc_for_freq(
        antennas_same_freq: &Vec<Antenna>,
        max_pos: &Position,
        extended: bool,
    ) -> HashSet<Position> {
        let mut result = HashSet::new();
        for (antenna1, antenna2) in cartesian!(antennas_same_freq.iter(), antennas_same_freq.iter())
        {
            if antenna1.position == antenna2.position {
                continue;
            }
            if extended {
                result.insert(antenna1.position);
                result.insert(antenna2.position);
                let antinodes1 = antenna1.position.get_extended_antinodes_for(
                    antenna2.position,
                    &Position { x: 0, y: 0 },
                    max_pos,
                );
                let antinodes2 = antenna2.position.get_extended_antinodes_for(
                    antenna1.position,
                    &Position { x: 0, y: 0 },
                    max_pos,
                );
                for antinode in antinodes1 {
                    result.insert(antinode);
                }
                for antinode in antinodes2 {
                    result.insert(antinode);
                }
            } else {
                let antinode1 = antenna1.position.get_antinode_for(antenna2.position);
                let antinode2 = antenna2.position.get_antinode_for(antenna1.position);
                result.insert(antinode1);
                result.insert(antinode2);
            }
        }
        result
    }

    grouped_antennas
        .iter()
        .map(|(_, v)| calc_for_freq(v, max_pos, extended))
        .flatten()
        .collect()
}

pub fn solve_with(extended: bool, input: &str) -> Option<usize> {
    let fields = get_fields(input);
    let antennas = fields
        .iter()
        .filter_map(|(_, v)| match v {
            Field::EmptyField => None,
            Field::AntennaField(antenna) => Some(*antenna),
        })
        .collect();
    let grouped_antennas = group_antennas_by_frequency(&antennas);
    let max_pos = fields
        .keys()
        .max_by(|a, b| {
            let ad = f64::sqrt(f64::powi(a.x as f64, 2) + f64::powi(a.y as f64, 2));
            let bd = f64::sqrt(f64::powi(b.x as f64, 2) + f64::powi(b.y as f64, 2));
            ad.total_cmp(&bd)
        })
        .unwrap();
    let antinodes = calc_antinodes(grouped_antennas, max_pos, extended);

    Some(
        antinodes
            .iter()
            .filter(|pos| match fields.get(pos) {
                Some(_) => true,
                _ => false,
            })
            .count(),
    )
}

pub fn part_one(input: &str) -> Option<usize> {
    solve_with(false, input)
}

pub fn part_two(input: &str) -> Option<usize> {
    solve_with(true, input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_antinode_pos() {
        let pos1 = Position { x: 0, y: 0 };
        let pos2 = Position { x: 2, y: 0 };
        assert_eq!(pos1.get_antinode_for(pos2), Position { x: 4, y: 0 });

        let pos1 = Position { x: 2, y: 0 };
        let pos2 = Position { x: 0, y: 0 };
        assert_eq!(pos1.get_antinode_for(pos2), Position { x: -2, y: 0 });

        let pos1 = Position { x: 0, y: 0 };
        let pos2 = Position { x: 2, y: 2 };
        assert_eq!(pos1.get_antinode_for(pos2), Position { x: 4, y: 4 });

        let pos1 = Position { x: 1, y: 1 };
        let pos2 = Position { x: 2, y: 2 };
        assert_eq!(pos1.get_antinode_for(pos2), Position { x: 3, y: 3 });
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two_sample() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(9));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
