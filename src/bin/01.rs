use regex::Regex;

advent_of_code::solution!(1);

fn get_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    let lines = input.split("\n");

    let mut firsts = Vec::new();
    let mut seconds = Vec::new();

    let re = Regex::new(r"([0-9]+)\s+([0-9]+)").unwrap();
    for line in lines {
        if line.len() == 0 {
            continue;
        }
        let Some(groups) = re.captures(line) else {
            panic!("Whoops...");
        };
        let a = groups[1].parse::<u32>().unwrap();
        let b = groups[2].parse::<u32>().unwrap();
        firsts.push(a);
        seconds.push(b);
    }

    (firsts, seconds)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut result: u32 = 0;

    let (mut firsts, mut seconds) = get_lists(input);

    firsts.sort();
    seconds.sort();

    for (a, b) in firsts.iter().zip(seconds.iter()) {
        if a > b {
            result = result + (a - b);
        } else {
            result = result + (b - a);
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (firsts, seconds) = get_lists(input);
    let mut result: u32 = 0;
    for a in firsts.iter() {
        for b in seconds.iter() {
            if a == b {
                result = result + a;
            }
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
