use std::collections::HashSet;

advent_of_code::solution!(12);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
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

    fn should_contain(&self, other: &Self) -> bool {
        if self.plant != other.plant {
            false
        } else {
            self.positions.iter().any(|self_pos| {
                other
                    .positions
                    .iter()
                    .any(|other_pos| self_pos.is_neighbor(other_pos))
            })
        }
    }

    fn add_plot(&self, other: &Self) -> Plot {
        Plot {
            plant: self.plant,
            positions: self
                .positions
                .union(&other.positions)
                .map(|pos| *pos)
                .collect(),
        }
    }
}

fn parse_input(input: &str) -> Vec<Plot> {
    let mut result = Vec::new();

    for (y, line) in input.split("\n").enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = Position {
                x: x as i32,
                y: y as i32,
            };
            result.push(Plot {
                plant: c,
                positions: HashSet::from([pos]),
            });
        }
    }

    result
}

fn parse_plots(mut plots: Vec<Plot>) -> Vec<Plot> {
    // All this index stuff is way to error prone, but I don't know how to do it
    // without, as I need to immutably borrow to find fitting elements and I need to
    // mutably borrow to actualy change elements, so I guess indices it is...
    fn find_next_merge(plots: &Vec<Plot>) -> Option<(usize, usize)> {
        for (i, plot_a) in plots.iter().enumerate() {
            for (j, plot_b) in plots[i + 1..].iter().enumerate() {
                if plot_a.should_contain(plot_b) {
                    return Some((i, j + i + 1));
                }
            }
        }
        None
    }

    // This is unbelievably slow, probably because of all the removal and reallocations
    loop {
        match find_next_merge(&plots) {
            None => return plots,
            Some((i, j)) => {
                let plot_b = plots.remove(j);
                let plot_a = plots.remove(i);
                plots.push(plot_a.add_plot(&plot_b));
            }
        }
    }
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
    None
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
        assert_eq!(
            plots.iter().map(|plot| plot.plant).collect::<Vec<_>>(),
            vec!['D', 'E', 'A', 'B', 'C']
        );
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
