use std::collections::HashMap;

advent_of_code::solution!(19);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Color {
    White, // w
    Blue,  // u
    Black, // b
    Red,   // r
    Green, // g
}

#[derive(Debug)]
struct Towel {
    colors: Vec<Color>,
}

#[derive(Debug)]
struct Matching {
    towels: Vec<Towel>,
    schemes: Vec<Vec<Color>>,
}

fn parse_input(input: &str) -> Matching {
    let color_from_char = |ch| match ch {
        'w' => Color::White,
        'u' => Color::Blue,
        'b' => Color::Black,
        'r' => Color::Red,
        'g' => Color::Green,
        _ => panic!("unknown color: {ch}"),
    };

    let mut lines = input.split("\n").filter(|l| !l.is_empty());

    let towels = lines
        .next()
        .map(|line| {
            line.split(", ")
                .map(|towel| {
                    let colors = towel.chars().map(color_from_char).collect::<Vec<_>>();
                    Towel { colors }
                })
                .collect::<Vec<_>>()
        })
        .unwrap();

    let schemes = lines
        .map(|line| line.chars().map(color_from_char).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    Matching { towels, schemes }
}

fn check_scheme(towels: &[Towel], scheme: &[Color]) -> bool {
    for towel in towels {
        match scheme.strip_prefix(&towel.colors[..]) {
            None => continue,
            Some(rest) => {
                if rest.is_empty() {
                    return true;
                } else {
                    if check_scheme(towels, rest) {
                        return true;
                    } else {
                        continue;
                    }
                }
            }
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<usize> {
    let matching = parse_input(input);
    Some(
        matching
            .schemes
            .iter()
            .filter(|scheme| check_scheme(&matching.towels, scheme))
            .count(),
    )
}

fn count_schemes(
    towels: &[Towel],
    scheme: &[Color],
    mut cache: HashMap<Vec<Color>, u64>,
) -> (u64, HashMap<Vec<Color>, u64>) {
    if let Some(cached_value) = cache.get(scheme) {
        return (*cached_value, cache);
    }

    let mut count = 0;

    for towel in towels {
        match scheme.strip_prefix(&towel.colors[..]) {
            None => continue,
            Some(rest) => {
                if rest.is_empty() {
                    count += 1
                } else {
                    let (rec_result, rec_cache) = count_schemes(towels, rest, cache);
                    cache = rec_cache;
                    count += rec_result
                }
            }
        }
    }

    cache.insert(scheme.to_vec(), count);
    (count, cache)
}

pub fn part_two(input: &str) -> Option<u64> {
    let matching = parse_input(input);
    let mut cache = HashMap::new();
    let mut result = 0;
    for scheme in matching.schemes {
        let (count, new_cache) = count_schemes(&matching.towels, &scheme, cache);
        cache = new_cache;
        result += count;
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
