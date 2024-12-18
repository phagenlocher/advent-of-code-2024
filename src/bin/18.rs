use petgraph::algo::dijkstra;
use petgraph::graphmap::*;

advent_of_code::solution!(18);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn neighbors(&self) -> [Vec2; 4] {
        [
            Vec2 {
                x: self.x,
                y: self.y + 1,
            },
            Vec2 {
                x: self.x,
                y: self.y - 1,
            },
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

fn build_graph_edges(size: i32) -> impl Iterator<Item = (Vec2, Vec2)> {
    (0..size)
        .map(move |y| (0..size).map(|x| Vec2 { x, y }).collect::<Vec<_>>())
        .flatten()
        .map(move |pos| {
            pos.neighbors()
                .into_iter()
                .filter(|nbrpos| {
                    nbrpos.x >= 0 && nbrpos.x <= size && nbrpos.y >= 0 && nbrpos.y <= size
                })
                .map(|nbrpos| (pos, nbrpos))
                .collect::<Vec<_>>()
        })
        .flatten()
}

fn parse_input(input: &str) -> impl Iterator<Item = Vec2> + '_ {
    input.split("\n").filter_map(|line| {
        if let [x, y] = line.split(",").collect::<Vec<_>>()[..] {
            let x = x.parse::<i32>().ok()?;
            let y = y.parse::<i32>().ok()?;
            Some(Vec2 { x, y })
        } else {
            None
        }
    })
}

fn part_one_sized(input: &str, size: i32, fallen_bytes: usize) -> Option<i32> {
    let corrupted = parse_input(input);
    let edges = build_graph_edges(size);
    let mut graph: UnGraphMap<Vec2, u8> = UnGraphMap::from_edges(edges);
    for corrupted_pos in corrupted.take(fallen_bytes) {
        graph.remove_node(corrupted_pos);
    }
    let start = Vec2 { x: 0, y: 0 };
    let end = Vec2 {
        x: size - 1,
        y: size - 1,
    };

    let result = dijkstra(&graph, start, Some(end), |_| 1);
    let result = result.get(&end);
    result.map(|x| *x)
}

pub fn part_one(input: &str) -> Option<i32> {
    part_one_sized(input, 71, 1024)
}

pub fn part_two_sized(input: &str, size: i32) -> Option<String> {
    let corrupted = parse_input(input);
    let edges = build_graph_edges(size);

    // Should use `Graph` here instead, to make the removal of nodes faster!
    let mut graph: UnGraphMap<Vec2, u8> = UnGraphMap::from_edges(edges);
    let start = Vec2 { x: 0, y: 0 };
    let end = Vec2 {
        x: size - 1,
        y: size - 1,
    };
    for corrupted_pos in corrupted {
        graph.remove_node(corrupted_pos);

        let result = dijkstra(&graph, start, Some(end), |_| 1);
        let result = result.get(&end);
        if result.is_none() {
            return Some(
                [
                    corrupted_pos.x.to_string(),
                    String::from(","),
                    corrupted_pos.y.to_string(),
                ]
                .concat(),
            );
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<String> {
    part_two_sized(input, 71)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_sized(&advent_of_code::template::read_file("examples", DAY), 7, 12);
        assert_eq!(result, Some(22));
    }
}
