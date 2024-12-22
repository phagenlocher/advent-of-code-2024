use petgraph::algo::all_simple_paths;
use petgraph::graphmap::*;
use std::sync::OnceLock;

advent_of_code::solution!(21);

struct Associator<I: Iterator> {
    iter: I,
    last: Option<I::Item>,
}

impl<I: Iterator> Associator<I> {
    fn assocs(iter: I) -> Associator<I> {
        Associator { iter, last: None }
    }
}

impl<I: Iterator> Iterator for Associator<I>
where
    I::Item: Copy,
{
    type Item = (I::Item, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let first = self.last.or_else(|| self.iter.next())?;
        let second = self.iter.next()?;
        self.last = Some(second);
        Some((first, second))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_direction_key(&self) -> DirectionKey {
        match self {
            Direction::Up => DirectionKey::Up,
            Direction::Down => DirectionKey::Down,
            Direction::Left => DirectionKey::Left,
            Direction::Right => DirectionKey::Right,
        }
    }
}

fn super_weird_shortest_path<N: Ord + PartialEq + Copy + std::hash::Hash>(
    graph: &DiGraphMap<N, Direction>,
    from: N,
    to: N,
) -> Option<Vec<Direction>> {
    if from == to {
        return Some(Vec::new());
    }

    /*
       In order to get the shortest path after multiple indirections using DirectionKeypads we have
       to minimize certain measures about the directions taken to press certain buttons:

           1.  The amount of changes in diretion have to be minimal. This ensures that indirections
               higher up can press the 'A' button multiple times, without having to leave it for
               another button.
           2.  Left and Down directions should happen earlier than Right and Up directions. This is
               somewhat counterintuitive since the Left button is the furthest from the 'A' button
               on the DirectionKeypad, but the idea is that you *first* press the left button and
               then press the other needed buttons on your way back to the 'A' button.
    */
    let direction_cost = |directions: &Vec<Direction>| -> usize {
        let change_penalty: usize = Associator::assocs(directions.iter())
            .map(|(a, b)| if a == b { 0 } else { 100 })
            .sum();

        let left_before_other_penalty: usize = directions
            .iter()
            .enumerate()
            .map(|(i, x)| match x {
                Direction::Left => i,
                Direction::Down => i * 2,
                Direction::Right => directions.len() - i,
                Direction::Up => directions.len() - i,
            })
            .sum();

        change_penalty + left_before_other_penalty
    };

    let mut result: Option<(Vec<Direction>, usize)> = None;
    for path in all_simple_paths::<Vec<_>, _>(graph, from, to, 0, None) {
        let directions = Associator::assocs(path.iter())
            .map(|(a, b)| *graph.edge_weight(*a, *b).unwrap())
            .collect();
        let path_cost = direction_cost(&directions);
        if let Some((ref best_path, best_cost)) = result {
            if best_cost > path_cost || (best_cost == path_cost && best_path.len() > path.len()) {
                result = Some((directions, path_cost))
            }
        } else {
            result = Some((directions, path_cost))
        }
    }
    result.map(|(p, _)| p)
}

trait Keypad {
    type Key: std::fmt::Debug + Ord + Eq + Copy + std::hash::Hash + 'static;

    const START_KEY: Self::Key;

    fn get_connection_graph() -> &'static DiGraphMap<Self::Key, Direction>;

    fn shortest_path_for_single_motion(from: Self::Key, to: Self::Key) -> Vec<Direction> {
        match super_weird_shortest_path(Self::get_connection_graph(), from, to) {
            None => panic!("Can't find a path from {from:?} to {to:?}"),
            Some(path) => path,
        }
    }

    fn shortest_path(presses: Vec<Self::Key>) -> Vec<Vec<Direction>> {
        let keys = vec![Self::START_KEY].into_iter().chain(presses.into_iter());
        Associator::assocs(keys)
            .map(|(from, to)| Self::shortest_path_for_single_motion(from, to))
            .collect()
    }
}

// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+

struct NumericKeypad;

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Ord, Eq, Hash)]
enum NumericKey {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Submit,
}

fn numeric_connection_graph() -> &'static DiGraphMap<NumericKey, Direction> {
    use Direction::*;
    use NumericKey::*;
    static GRAPHMAP: OnceLock<DiGraphMap<NumericKey, Direction>> = OnceLock::new();
    GRAPHMAP.get_or_init(|| {
        let mut g = DiGraphMap::new();
        // Horizontal
        for (a, b) in [
            (Zero, Submit),
            (One, Two),
            (Two, Three),
            (Four, Five),
            (Five, Six),
            (Seven, Eight),
            (Eight, Nine),
        ] {
            g.add_edge(a, b, Right);
            g.add_edge(b, a, Left);
        }

        // Vertical
        for (a, b) in [
            (Zero, Two),
            (Submit, Three),
            (One, Four),
            (Two, Five),
            (Three, Six),
            (Four, Seven),
            (Five, Eight),
            (Six, Nine),
        ] {
            g.add_edge(a, b, Up);
            g.add_edge(b, a, Down);
        }

        g
    })
}

impl Keypad for NumericKeypad {
    type Key = NumericKey;

    const START_KEY: Self::Key = NumericKey::Submit;

    fn get_connection_graph() -> &'static DiGraphMap<Self::Key, Direction> {
        numeric_connection_graph()
    }
}

//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+

struct DirectionKeypad;

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Ord, Eq, Hash)]
enum DirectionKey {
    Up,
    Down,
    Left,
    Right,
    Submit,
}

fn direction_connection_graph() -> &'static DiGraphMap<DirectionKey, Direction> {
    use DirectionKey::*;
    static GRAPHMAP: OnceLock<DiGraphMap<DirectionKey, Direction>> = OnceLock::new();
    GRAPHMAP.get_or_init(|| {
        let mut g = DiGraphMap::new();
        // Horizontal
        for (a, b) in [(Left, Down), (Down, Right), (Up, Submit)] {
            g.add_edge(a, b, Direction::Right);
            g.add_edge(b, a, Direction::Left);
        }

        // Vertical
        for (a, b) in [(Down, Up), (Right, Submit)] {
            g.add_edge(a, b, Direction::Up);
            g.add_edge(b, a, Direction::Down);
        }

        g
    })
}

impl Keypad for DirectionKeypad {
    type Key = DirectionKey;

    const START_KEY: Self::Key = DirectionKey::Submit;

    fn get_connection_graph() -> &'static DiGraphMap<Self::Key, Direction> {
        direction_connection_graph()
    }
}

fn direction_keys_for<K: Keypad>(presses: Vec<K::Key>) -> Vec<DirectionKey> {
    let mut result = Vec::new();
    for directions in K::shortest_path(presses) {
        for direction in directions {
            result.push(direction.to_direction_key());
        }
        result.push(DirectionKey::Submit);
    }
    result
}

fn parse_input(input: &str) -> Vec<Vec<NumericKey>> {
    input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '0' => NumericKey::Zero,
                    '1' => NumericKey::One,
                    '2' => NumericKey::Two,
                    '3' => NumericKey::Three,
                    '4' => NumericKey::Four,
                    '5' => NumericKey::Five,
                    '6' => NumericKey::Six,
                    '7' => NumericKey::Seven,
                    '8' => NumericKey::Eight,
                    '9' => NumericKey::Nine,
                    'A' => NumericKey::Submit,
                    _ => panic!("unknown key: {c}"),
                })
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    fn extract_numeric_part(code: &Vec<NumericKey>) -> usize {
        code.iter()
            .filter_map(|key| match key {
                NumericKey::Zero => Some(0.to_string()),
                NumericKey::One => Some(1.to_string()),
                NumericKey::Two => Some(2.to_string()),
                NumericKey::Three => Some(3.to_string()),
                NumericKey::Four => Some(4.to_string()),
                NumericKey::Five => Some(5.to_string()),
                NumericKey::Six => Some(6.to_string()),
                NumericKey::Seven => Some(7.to_string()),
                NumericKey::Eight => Some(8.to_string()),
                NumericKey::Nine => Some(9.to_string()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .concat()
            .parse::<usize>()
            .unwrap()
    }

    let codes = parse_input(input);
    let mut result = 0;

    for code in codes {
        let numeric = extract_numeric_part(&code);
        let lvl1 = direction_keys_for::<NumericKeypad>(code);
        let lvl2 = direction_keys_for::<DirectionKeypad>(lvl1);
        let lvl3 = direction_keys_for::<DirectionKeypad>(lvl2);
        let len = lvl3.len();
        result += len * numeric;
    }

    Some(result)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numeric_keypad_shortest_path_for_single_motion() {
        use Direction::*;
        use NumericKey::*;
        let result = NumericKeypad::shortest_path_for_single_motion(Submit, Submit);
        assert_eq!(result.len(), 0);
        let result = NumericKeypad::shortest_path_for_single_motion(Zero, Nine);
        assert_eq!(result.len(), 4);
        let result = NumericKeypad::shortest_path_for_single_motion(Submit, Seven);
        assert_eq!(result.len(), 5);
        assert_eq!(result, vec![Up, Up, Up, Left, Left]);
    }

    #[test]
    fn test_numeric_keypad_shortest_path() {
        use Direction::*;
        use NumericKey::*;
        let input = vec![Zero, Two, Nine, Submit, Submit];
        let result = NumericKeypad::shortest_path(input.clone());
        assert_eq!(result.len(), input.len());
        assert_eq!(
            result,
            vec![
                vec![Left],
                vec![Up],
                vec![Right, Up, Up],
                vec![Down, Down, Down],
                vec![]
            ]
        );
    }

    #[test]
    fn test_example() {
        let input = "029A";
        let result = part_one(input);
        assert_eq!(result, Some(68 * 29));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
