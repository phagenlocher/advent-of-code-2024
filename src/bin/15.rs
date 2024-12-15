use std::collections::HashMap;

advent_of_code::solution!(15);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2 {
    x: u32,
    y: u32,
}

impl Vec2 {
    fn next_in_direction(&self, direction: &Direction) -> Vec2 {
        match direction {
            Direction::Up => Vec2 {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Down => Vec2 {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Vec2 {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Vec2 {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum MovableObject {
    Wall,
    Box,
}

struct Warehouse {
    robot: Vec2,
    objects: HashMap<Vec2, MovableObject>,
}

impl Warehouse {
    fn move_robot(&mut self, direction: &Direction) {
        let new_pos = self.robot.next_in_direction(direction);
        if self.move_object(&new_pos, direction) {
            self.robot = new_pos
        }
    }

    fn move_object(&mut self, pos: &Vec2, direction: &Direction) -> bool {
        match self.objects.remove(pos) {
            None => true,
            Some(MovableObject::Wall) => {
                self.objects.insert(*pos, MovableObject::Wall);
                false
            }
            Some(MovableObject::Box) => {
                let new_pos = pos.next_in_direction(direction);
                if self.move_object(&new_pos, direction) {
                    self.objects.insert(new_pos, MovableObject::Box);
                    true
                } else {
                    self.objects.insert(*pos, MovableObject::Box);
                    false
                }
            }
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        let (x_dim, y_dim) = self.objects.keys().fold((0, 0), |(x_max, y_max), pos| {
            (u32::max(x_max, pos.x), u32::max(y_max, pos.y))
        });

        for y in 0..=y_dim {
            for x in 0..=x_dim {
                let pos = Vec2 { x, y };
                match self.objects.get(&pos) {
                    None => {
                        if self.robot == pos {
                            print!("@")
                        } else {
                            print!(".")
                        }
                    }
                    Some(MovableObject::Wall) => print!("#"),
                    Some(MovableObject::Box) => print!("O"),
                }
            }
            print!("\n");
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ObjectPart {
    Wall,
    BoxLeft,
    BoxRight,
}

struct WideWarehouse {
    robot: Vec2,
    objects: HashMap<Vec2, ObjectPart>,
}

impl WideWarehouse {
    fn move_robot(&mut self, direction: &Direction) {
        let new_pos = self.robot.next_in_direction(direction);
        if self.move_object(&new_pos, direction) {
            self.robot = new_pos
        }
    }

    fn move_object(&mut self, pos: &Vec2, direction: &Direction) -> bool {
        match self.objects.remove(pos) {
            None => true,
            Some(ObjectPart::Wall) => {
                self.objects.insert(*pos, ObjectPart::Wall);
                false
            }
            Some(ObjectPart::BoxLeft) => {
                let box_left_pos = *pos;
                let box_right_pos = pos.next_in_direction(&Direction::Right);

                // Remove right part
                self.objects.remove(&box_right_pos);

                match direction {
                    Direction::Right => {
                        let could_move = self.move_object(
                            &box_right_pos.next_in_direction(&Direction::Right),
                            direction,
                        );
                        if could_move {
                            self.objects.insert(
                                box_left_pos.next_in_direction(direction),
                                ObjectPart::BoxLeft,
                            );
                            self.objects.insert(
                                box_right_pos.next_in_direction(direction),
                                ObjectPart::BoxRight,
                            );
                            true
                        } else {
                            self.objects.insert(box_left_pos, ObjectPart::BoxLeft);
                            self.objects.insert(box_right_pos, ObjectPart::BoxRight);
                            false
                        }
                    }
                    Direction::Left => panic!("This shove should have been handled elsewhere!"),
                    _ => {
                        let new_left_pos = box_left_pos.next_in_direction(direction);
                        let new_right_pos = box_right_pos.next_in_direction(direction);

                        let old_objects = self.objects.clone();
                        let left_moved = self.move_object(&new_left_pos, direction);
                        let right_moved = self.move_object(&new_right_pos, direction);

                        if left_moved && right_moved {
                            self.objects.insert(new_left_pos, ObjectPart::BoxLeft);
                            self.objects.insert(new_right_pos, ObjectPart::BoxRight);
                            true
                        } else {
                            self.objects = old_objects;
                            self.objects.insert(box_left_pos, ObjectPart::BoxLeft);
                            self.objects.insert(box_right_pos, ObjectPart::BoxRight);
                            false
                        }
                    }
                }
            }
            Some(ObjectPart::BoxRight) => {
                let box_left_pos = pos.next_in_direction(&Direction::Left);
                let box_right_pos = *pos;

                // Remove left part
                self.objects.remove(&box_left_pos);

                match direction {
                    Direction::Left => {
                        let could_move = self.move_object(
                            &box_left_pos.next_in_direction(&Direction::Left),
                            direction,
                        );
                        if could_move {
                            self.objects.insert(
                                box_left_pos.next_in_direction(direction),
                                ObjectPart::BoxLeft,
                            );
                            self.objects.insert(
                                box_right_pos.next_in_direction(direction),
                                ObjectPart::BoxRight,
                            );
                            true
                        } else {
                            self.objects.insert(box_left_pos, ObjectPart::BoxLeft);
                            self.objects.insert(box_right_pos, ObjectPart::BoxRight);
                            false
                        }
                    }
                    Direction::Right => panic!("This shove should have been handled elsewhere!"),
                    _ => {
                        let new_left_pos = box_left_pos.next_in_direction(direction);
                        let new_right_pos = box_right_pos.next_in_direction(direction);

                        let old_objects = self.objects.clone();
                        let left_moved = self.move_object(&new_left_pos, direction);
                        let right_moved = self.move_object(&new_right_pos, direction);

                        if left_moved && right_moved {
                            self.objects.insert(new_left_pos, ObjectPart::BoxLeft);
                            self.objects.insert(new_right_pos, ObjectPart::BoxRight);
                            true
                        } else {
                            self.objects = old_objects;
                            self.objects.insert(box_left_pos, ObjectPart::BoxLeft);
                            self.objects.insert(box_right_pos, ObjectPart::BoxRight);
                            false
                        }
                    }
                }
            }
        }
    }

    fn from_warehouse(warehouse: &Warehouse) -> WideWarehouse {
        let mut objects = HashMap::new();
        for (pos, obj) in warehouse.objects.iter() {
            let pos_scaled = Vec2 {
                x: pos.x * 2,
                y: pos.y,
            };
            match obj {
                MovableObject::Wall => {
                    objects.insert(pos_scaled, ObjectPart::Wall);
                    objects.insert(
                        pos_scaled.next_in_direction(&Direction::Right),
                        ObjectPart::Wall,
                    );
                }
                MovableObject::Box => {
                    objects.insert(pos_scaled, ObjectPart::BoxLeft);
                    objects.insert(
                        pos_scaled.next_in_direction(&Direction::Right),
                        ObjectPart::BoxRight,
                    );
                }
            }
        }
        WideWarehouse {
            robot: Vec2 {
                x: warehouse.robot.x * 2,
                y: warehouse.robot.y,
            },
            objects,
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        let (x_dim, y_dim) = self.objects.keys().fold((0, 0), |(x_max, y_max), pos| {
            (u32::max(x_max, pos.x), u32::max(y_max, pos.y))
        });

        for y in 0..=y_dim {
            for x in 0..=x_dim {
                let pos = Vec2 { x, y };
                match self.objects.get(&pos) {
                    None => {
                        if self.robot == pos {
                            print!("@")
                        } else {
                            print!(".")
                        }
                    }
                    Some(ObjectPart::Wall) => print!("#"),
                    Some(ObjectPart::BoxLeft) => print!("["),
                    Some(ObjectPart::BoxRight) => print!("]"),
                }
            }
            print!("\n");
        }
    }
}

fn parse_input(input: &str) -> (Warehouse, Vec<Direction>) {
    let mut splits = input.split("\n\n");
    let warehouse_str = splits.next().unwrap();
    let directions_str = splits.next().unwrap();

    let mut warehouse_objs = HashMap::new();
    let mut robot_pos = None;

    for (y, line) in warehouse_str.split("\n").enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = Vec2 {
                x: x as u32,
                y: y as u32,
            };
            match c {
                '@' => robot_pos = Some(pos),
                '#' => {
                    warehouse_objs.insert(pos, MovableObject::Wall);
                    ()
                }
                'O' => {
                    warehouse_objs.insert(pos, MovableObject::Box);
                    ()
                }
                _ => (),
            }
        }
    }

    let warehouse = Warehouse {
        robot: robot_pos.unwrap(),
        objects: warehouse_objs,
    };

    let directions = directions_str
        .chars()
        .filter_map(|c| match c {
            '^' => Some(Direction::Up),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            _ => None,
        })
        .collect();

    (warehouse, directions)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut warehouse, directions) = parse_input(input);
    for direction in directions {
        warehouse.move_robot(&direction);
    }
    Some(
        warehouse
            .objects
            .iter()
            .filter_map(|(pos, obj)| match obj {
                MovableObject::Wall => None,
                MovableObject::Box => Some(pos.y * 100 + pos.x),
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (warehouse, directions) = parse_input(input);
    let mut warehouse = WideWarehouse::from_warehouse(&warehouse);
    for direction in directions {
        warehouse.move_robot(&direction);
    }
    Some(
        warehouse
            .objects
            .iter()
            .filter_map(|(pos, obj)| match obj {
                ObjectPart::BoxLeft => Some(pos.y * 100 + pos.x),
                _ => None,
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_ex_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(10092));
    }
    #[test]
    fn test_part_one_ex_two() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(9021));
    }
}
