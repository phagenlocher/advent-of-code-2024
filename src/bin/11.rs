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

use ThisOrThat::*;

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

fn blinks_row(n: u32, row: &Vec<u64>) -> usize {
    fn blink_row(row: Vec<u64>) -> Vec<u64> {
        let mut result = Vec::new();
        for stone in row.iter() {
            match blink(*stone) {
                This(new_stone) => result.push(new_stone),
                That((fst, snd)) => {
                    result.push(fst);
                    result.push(snd);
                }
            }
        }
        result
    }
    let mut result = row.to_vec();
    for _ in 0..n {
        result = blink_row(result)
    }
    result.len()
}

pub fn part_one(input: &str) -> Option<usize> {
    let row = parse_input(input);
    Some(blinks_row(25, &row))
}

pub fn part_two(_input: &str) -> Option<usize> {
    /*
        I wanted to do some caching of computations here, but I can't figure out how to
        pass a hashmap that I need to borrow mutably and immutably...
    */
    //let row = parse_input(input);
    // Some(blinks_row(75, &row))
    None
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
