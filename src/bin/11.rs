use std::collections::HashMap;
use ThisOrThat::*;

advent_of_code::solution!(11);

fn parse_input(input: &str) -> Vec<u64> {
    input
        .split("\n")
        .next()
        .unwrap()
        .split(" ")
        .map(|n| n.parse::<u64>().unwrap())
        .collect()
}

#[derive(Copy, Clone)]
enum ThisOrThat<A, B> {
    This(A),
    That(B),
}

type BlinkResult = ThisOrThat<u64, (u64, u64)>;

fn blink(stone: u64) -> BlinkResult {
    match stone {
        0 => This(1),
        n => {
            let num_digits = 1 + (f64::floor(f64::log10(n as f64)) as u32);
            if num_digits % 2 == 0 {
                let factor = u64::pow(10, num_digits / 2);
                let first = n / factor;
                let second = n % factor;
                That((first, second))
            } else {
                This(n * 2024)
            }
        }
    }
}

fn blinks(
    n: u32,
    stone: u64,
    cache: HashMap<(u32, u64), usize>,
) -> (usize, HashMap<(u32, u64), usize>) {
    if n == 0 {
        return (1, cache);
    }

    match blink(stone) {
        This(x) => {
            if let Some(cached_result) = cache.get(&(n - 1, x)) {
                (*cached_result, cache)
            } else {
                let (count, mut new_cache) = blinks(n - 1, x, cache);
                new_cache.insert((n - 1, x), count);
                (count, new_cache)
            }
        }

        That((fst, snd)) => {
            let (count1, fst_cache) = if let Some(cached_result) = cache.get(&(n - 1, fst)) {
                (*cached_result, cache)
            } else {
                let (count, mut new_cache) = blinks(n - 1, fst, cache);
                new_cache.insert((n - 1, fst), count);
                (count, new_cache)
            };
            let (count2, snd_cache) = if let Some(cached_result) = fst_cache.get(&(n - 1, snd)) {
                (*cached_result, fst_cache)
            } else {
                let (count, mut new_cache) = blinks(n - 1, snd, fst_cache);
                new_cache.insert((n - 1, snd), count);
                (count, new_cache)
            };
            (count1 + count2, snd_cache)
        }
    }
}

fn blinks_row(n: u32, row: &Vec<u64>) -> usize {
    let mut cache = HashMap::new();
    let mut result = 0;

    for stone in row {
        let (count, new_cache) = blinks(n, *stone, cache);
        cache = new_cache;
        result += count
    }

    result
}

pub fn part_one(input: &str) -> Option<usize> {
    let row = parse_input(input);
    Some(blinks_row(25, &row))
}

pub fn part_two(input: &str) -> Option<usize> {
    let row = parse_input(input);
    Some(blinks_row(75, &row))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_ex() {
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        let row = parse_input(input);
        let result = blinks_row(6, &row);
        assert_eq!(result, 22);
        let result = blinks_row(25, &row);
        assert_eq!(result, 55312);
    }
}
