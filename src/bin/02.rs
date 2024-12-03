advent_of_code::solution!(2);

fn parse_report_from_line(line: &str) -> Vec<u32> {
    let nums = line.split(" ");
    let mut result = Vec::new();
    for num in nums {
        let parsed_num = num.parse::<u32>().unwrap();
        result.push(parsed_num);
    }
    result
}

fn parse_reports(input: &str) -> Vec<Vec<u32>> {
    let lines = input.split("\n");

    let mut result = vec![];

    for line in lines {
        if line.is_empty() {
            continue;
        }
        let report = parse_report_from_line(line);
        result.push(report);
    }

    result
}

fn is_safe(report: &[u32]) -> bool {
    enum Order {
        Increasing,
        Decreasing,
        Unknown,
    }

    let mut order = Order::Unknown;
    for i in 1..report.len() {
        let a = report[i - 1];
        let b = report[i];
        let diff = if a > b { a - b } else { b - a };
        if !(1..=3).contains(&diff) {
            return false;
        }
        match order {
            Order::Unknown => {
                if a > b {
                    order = Order::Decreasing
                } else {
                    order = Order::Increasing
                }
            }
            Order::Increasing => {
                if a > b {
                    return false;
                }
            }
            Order::Decreasing => {
                if a < b {
                    return false;
                }
            }
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let reports = parse_reports(input);
    let mut ok_count = 0;

    for report in reports {
        if is_safe(&report) {
            ok_count += 1;
        }
    }
    Some(ok_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = parse_reports(input);
    let mut ok_count = 0;

    for report in reports {
        if is_safe(&report) {
            ok_count += 1;
        } else {
            for i in 0..report.len() {
                let mod_report = [&report[0..i], &report[i + 1..]].concat();
                if is_safe(&mod_report) {
                    ok_count += 1;
                    break;
                }
            }
        }
    }
    Some(ok_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
