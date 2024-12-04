advent_of_code::solution!(4);

fn produce_table(input: &str) -> Vec<String> {
    let lines = input.split("\n");
    let mut result = Vec::new();
    for line in lines {
        if !line.is_empty() {
            result.push(String::from(line));
        }
    }
    result
}

fn dims(table: &Vec<String>) -> (usize, usize) {
    (table[0].len(), table.len())
}

fn horizontal_strings(table: &Vec<String>) -> Vec<String> {
    table.clone()
}

fn vertical_strings(table: &Vec<String>) -> Vec<String> {
    let (width, height) = dims(&table);
    let mut result = Vec::new();
    for j in 0..width {
        let mut vert_str = String::new();
        for i in 0..height {
            let ch = table[i].as_bytes()[j] as char;
            vert_str.push(ch);
        }
        result.push(vert_str);
    }
    result
}

fn diagonal_strings(table: &Vec<String>) -> Vec<String> {
    let (width, height) = dims(&table);
    let mut result = Vec::new();

    // top left to bottom right
    for x_offset in 0..width {
        let mut diag_str = String::new();
        let js = 0..(width - x_offset);
        let is = 0..height;
        for (j, i) in js.zip(is) {
            let ch = table[i].as_bytes()[j + x_offset] as char;
            diag_str.push(ch);
        }
        result.push(diag_str);
    }

    for y_offset in 1..height {
        let mut diag_str = String::new();
        let js = 0..width;
        let is = 0..(height - y_offset);
        for (j, i) in js.zip(is) {
            let ch = table[i + y_offset].as_bytes()[j] as char;
            diag_str.push(ch);
        }
        result.push(diag_str);
    }

    // top right to left bottom
    for x_offset in 0..width {
        let mut diag_str = String::new();
        let js = ((0 + x_offset)..width).rev();
        let is = 0..height;
        for (j, i) in js.zip(is) {
            let ch = table[i].as_bytes()[j - x_offset] as char;
            diag_str.push(ch);
        }
        result.push(diag_str);
    }

    for y_offset in 1..height {
        let mut diag_str = String::new();
        let js = (0..width).rev();
        let is = (0 + y_offset)..height;
        for (j, i) in js.zip(is) {
            let ch = table[i].as_bytes()[j] as char;
            diag_str.push(ch);
        }
        result.push(diag_str);
    }

    result
}

fn num_of_xmas(strings: &Vec<String>) -> usize {
    fn xmas_in_str(string: &String) -> usize {
        let mut result = 0;
        if string.len() >= 4 {
            for i in 0..string.len() {
                match string.get(i..(i + 4)) {
                    Some("XMAS") => result += 1,
                    Some("SAMX") => result += 1,
                    _ => {}
                }
            }
        }
        result
    }
    strings.iter().map(xmas_in_str).sum()
}

pub fn part_one(input: &str) -> Option<usize> {
    let table = produce_table(input);
    let all_strings = [
        horizontal_strings(&table),
        vertical_strings(&table),
        diagonal_strings(&table),
    ]
    .concat();
    Some(num_of_xmas(&all_strings))
}

pub fn part_two(input: &str) -> Option<u32> {
    let table = produce_table(input);

    let ch = |i: usize, j: usize| -> char { table[i].as_bytes()[j] as char };

    let mut result = 0;
    for i in 1..(table.len() - 1) {
        for j in 1..(table[0].len() - 1) {
            let str1 = [ch(i - 1, j - 1), ch(i, j), ch(i + 1, j + 1)];
            let str2 = [ch(i - 1, j + 1), ch(i, j), ch(i + 1, j - 1)];
            let targets = [['M', 'A', 'S'], ['S', 'A', 'M']];
            if targets.contains(&str1) && targets.contains(&str2) {
                result += 1;
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
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(9));
    }
}
