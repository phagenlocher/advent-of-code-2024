use std::collections::HashMap;
use std::collections::HashSet;

advent_of_code::solution!(10);

type Height = u8;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn move_x(&mut self, x: i32) {
        self.x += x
    }

    fn move_y(&mut self, y: i32) {
        self.y += y
    }

    fn moved(&self, x: i32, y: i32) -> Position {
        let mut clone = self.clone();
        clone.move_x(x);
        clone.move_y(y);
        clone
    }
}

#[derive(Debug, Clone)]
struct HeightMap(HashMap<Position, Height>);

impl HeightMap {
    fn get_neighbhors(&self, pos: Position) -> Vec<(Position, Height)> {
        [
            pos.moved(-1, 0),
            pos.moved(1, 0),
            pos.moved(0, -1),
            pos.moved(0, 1),
        ]
        .iter()
        .filter_map(|pos| self.0.get(pos).map(|height| (*pos, *height)))
        .collect()
    }

    fn get_height(&self, pos: Position) -> Option<Height> {
        self.0.get(&pos).map(|height| *height)
    }

    fn start_positions(&self) -> Vec<Position> {
        self.0
            .iter()
            .filter_map(|(pos, h)| if *h == 0 { Some(*pos) } else { None })
            .collect()
    }
}

fn parse_input(input: &str) -> HeightMap {
    let mut result = HashMap::new();

    for (y, line) in input.split("\n").enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = Position {
                x: x as i32,
                y: y as i32,
            };
            let height = match c {
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                '0' => 0,
                _ => panic!("Whoops: {c}"),
            };
            result.insert(pos, height);
        }
    }

    HeightMap(result)
}

fn get_trailhead_rating(
    height_map: &HeightMap,
    start: Position,
    seen: &mut HashSet<Position>,
    keep_seen: bool,
) -> u32 {
    let curr_height_opt = height_map.get_height(start);
    seen.insert(start);

    match curr_height_opt {
        None => panic!("Invalid position: {start:?}"),
        Some(9) => 1,
        Some(curr_height) => {
            let ngbrs = height_map.get_neighbhors(start);
            let new_pos = ngbrs
                .iter()
                .filter(|(pos, height)| !seen.contains(pos) && (curr_height + 1) == *height)
                .collect::<Vec<_>>();

            let mut result = 0;
            for (pos, _) in new_pos {
                let mut seen_new = seen.clone();
                result += get_trailhead_rating(
                    height_map,
                    *pos,
                    if keep_seen { seen } else { &mut seen_new },
                    keep_seen,
                )
            }
            result
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let height_map = parse_input(input);
    Some(
        height_map
            .start_positions()
            .iter()
            .map(|pos| get_trailhead_rating(&height_map, *pos, &mut HashSet::new(), true))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let height_map = parse_input(input);
    Some(
        height_map
            .start_positions()
            .iter()
            .map(|pos| get_trailhead_rating(&height_map, *pos, &mut HashSet::new(), false))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
