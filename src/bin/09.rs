advent_of_code::solution!(9);

type Id = u64;
type Size = u64;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Chunk {
    File(Id, Size),
    Empty(Size),
}

impl Chunk {
    fn size(&self) -> Size {
        match self {
            Chunk::File(_, size) => *size,
            Chunk::Empty(size) => *size,
        }
    }

    fn is_file(&self) -> bool {
        match self {
            Chunk::File(_, _) => true,
            _ => false,
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            Chunk::Empty(_) => true,
            _ => false,
        }
    }
}

fn parse_input(input: &str) -> Vec<Chunk> {
    let mut is_file = true;
    let mut id_counter = 0;
    input
        .split("\n")
        .next()
        .unwrap()
        .chars()
        .filter_map(|c| {
            let n = match c {
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                '0' => 0,
                _ => panic!("Whoops: {c}"),
            };

            let mut file_created = false;

            let result = if n == 0 {
                None
            } else if is_file {
                file_created = true;
                Some(Chunk::File(id_counter, n))
            } else {
                Some(Chunk::Empty(n))
            };

            is_file = !is_file;
            if file_created {
                id_counter += 1;
            }

            result
        })
        .collect()
}

fn compact_chunks_fragmented(memory: &mut Vec<Chunk>) {
    loop {
        let empty_index = match memory.iter().position(|x| x.is_empty()) {
            None => break,
            Some(i) => i,
        };

        let last_chunk = match memory.pop() {
            None => break,
            Some(chunk) => chunk,
        };

        match last_chunk {
            Chunk::Empty(_) => continue,
            Chunk::File(id, size) => {
                let empty_space = memory[empty_index];

                if empty_space.size() == size {
                    memory[empty_index] = Chunk::File(id, empty_space.size());
                } else if empty_space.size() < size {
                    memory[empty_index] = Chunk::File(id, empty_space.size());
                    memory.push(Chunk::File(id, size - empty_space.size()));
                } else {
                    // empty_space.size() > size
                    let new = [
                        Chunk::File(id, size),
                        Chunk::Empty(empty_space.size() - size),
                    ];
                    memory.splice(empty_index..=empty_index, new);
                }
            }
        }
    }
}

fn compact_chunks_unfragmented(memory: &mut Vec<Chunk>) {
    let memory_clone = memory.clone();
    let files: Vec<&Chunk> = memory_clone
        .iter()
        .filter(|chunk| chunk.is_file())
        .rev()
        .collect();
    for file in files {
        let file_index = match memory.iter().position(|chunk| chunk == file) {
            None => continue,
            Some(index) => index,
        };
        let empty_index = match memory.iter().enumerate().position(|(i, chunk)| {
            chunk.is_empty() && chunk.size() >= file.size() && i < file_index
        }) {
            None => continue,
            Some(index) => index,
        };

        let empty = memory[empty_index];
        memory[file_index] = Chunk::Empty(file.size());

        if empty.size() == file.size() {
            memory[empty_index] = *file;
        } else {
            // empty.size() > file.size()
            let new = [*file, Chunk::Empty(empty.size() - file.size())];
            memory.splice(empty_index..=empty_index, new);
        }
    }
}

fn compute_checksum(memory: &Vec<Chunk>) -> u64 {
    let mut index: u64 = 0;
    memory
        .iter()
        .map(|chunk| match chunk {
            Chunk::Empty(size) => {
                index += size;
                0
            }
            Chunk::File(id, size) => {
                let mut result: u64 = 0;
                for _ in 0..(*size) {
                    result += *id * index;
                    index += 1;
                }
                result
            }
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut memory = parse_input(input);
    compact_chunks_fragmented(&mut memory);
    Some(compute_checksum(&memory))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut memory = parse_input(input);
    compact_chunks_unfragmented(&mut memory);
    Some(compute_checksum(&memory))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
