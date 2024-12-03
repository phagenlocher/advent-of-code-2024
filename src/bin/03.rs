use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((?<a>\d{1,3}),(?<b>\d{1,3})\)").unwrap();
    let result = re
        .captures_iter(input)
        .map(|caps| {
            let a = caps.name("a").unwrap().as_str().parse::<u32>().unwrap();
            let b = caps.name("b").unwrap().as_str().parse::<u32>().unwrap();
            a * b
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(mul\((?<a>\d{1,3}),(?<b>\d{1,3})\))|(?<do>do\(\))|(?<dont>don't\(\))")
        .unwrap();
    let mut mul_enabled = true;
    let result = re
        .captures_iter(input)
        .map(|caps| {
            let do_stmt_opt = caps.name("do");
            let dont_stmt_opt = caps.name("dont");

            if do_stmt_opt.is_some() {
                mul_enabled = true;
                return 0;
            } else if dont_stmt_opt.is_some() {
                mul_enabled = false;
                return 0;
            } else {
                let a_opt = caps.name("a");
                let b_opt = caps.name("b");
                match (a_opt, b_opt) {
                    (Some(a_match), Some(b_match)) => {
                        let a = a_match.as_str().parse::<u32>().unwrap();
                        let b = b_match.as_str().parse::<u32>().unwrap();
                        if mul_enabled {
                            return a * b;
                        } else {
                            return 0;
                        }
                    }
                    _ => panic!("Whoops!"),
                }
            }
        })
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
