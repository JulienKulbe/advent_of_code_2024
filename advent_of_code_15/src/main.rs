use anyhow::Result;
use std::{collections::VecDeque, fmt::Debug, fs::read_to_string};

fn main() -> Result<()> {
    let sum = calculate_gps_sum("input.txt")?;
    println!("GPS sum: {sum}");

    let sum = calculate_gps_sum_scaled_map("input.txt")?;
    println!("GPS: {sum}");

    Ok(())
}

fn calculate_gps_sum(filename: &str) -> Result<usize> {
    let (mut map, mut robot) = parse_file(filename, false)?;

    while let Some(direction) = robot.0.pop_front() {
        map.robot_move(direction);
    }

    Ok(map.get_box_coordinates())
}

fn calculate_gps_sum_scaled_map(filename: &str) -> Result<usize> {
    let (mut map, mut robot) = parse_file(filename, true)?;
    //println!("Initial:\n{:?}", map);

    while let Some(direction) = robot.0.pop_front() {
        map.robot_move(direction);
        //println!("{:?}:\n{:?}", direction, map);
    }

    Ok(map.get_box_coordinates())
}

fn parse_file(filename: &str, scaled: bool) -> Result<(Map, RobotMovements)> {
    let data = read_to_string(filename)?;
    let index = data.find("\n\n").expect("No blank line found");
    let map = data[..index]
        .lines()
        .map(|line| line.chars().map(Entity::from).collect())
        .collect();
    let movement = data[index..]
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(Direction::from)
        .collect();

    Ok((Map::new(map, scaled), RobotMovements(movement)))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Invalid character"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Entity {
    Empty,
    Wall,
    Box,
    LeftBox,
    RightBox,
    Robot,
}

impl From<char> for Entity {
    fn from(c: char) -> Self {
        match c {
            '#' => Entity::Wall,
            '.' => Entity::Empty,
            'O' => Entity::Box,
            '@' => Entity::Robot,
            _ => panic!("Invalid character"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position(usize, usize);

impl Position {
    fn go_to(&self, direction: Direction) -> Position {
        match direction {
            Direction::Up => Position(self.0, self.1 - 1),
            Direction::Down => Position(self.0, self.1 + 1),
            Direction::Left => Position(self.0 - 1, self.1),
            Direction::Right => Position(self.0 + 1, self.1),
        }
    }
}

struct Map {
    data: Vec<Vec<Entity>>,
    robot: Position,
}

impl Map {
    fn new(mut data: Vec<Vec<Entity>>, scaled: bool) -> Map {
        if scaled {
            let mut scaled_map = Vec::new();
            for row in data.iter() {
                let mut new_row = Vec::new();
                for row in row {
                    let entities = match *row {
                        Entity::Box => [Entity::LeftBox, Entity::RightBox],
                        Entity::Empty => [Entity::Empty, Entity::Empty],
                        Entity::Wall => [Entity::Wall, Entity::Wall],
                        Entity::Robot => [Entity::Robot, Entity::Empty],
                        _ => panic!("Invalid Entity"),
                    };
                    new_row.append(&mut Vec::from(entities));
                }
                scaled_map.push(new_row);
            }
            data = scaled_map;
        }

        let mut robot = None;
        for (y, row) in data.iter().enumerate() {
            for (x, e) in row.iter().enumerate() {
                if *e == Entity::Robot {
                    robot = Some(Position(x, y));
                }
            }
        }

        Map {
            data,
            robot: robot.expect("No Robot was found"),
        }
    }

    fn robot_move(&mut self, direction: Direction) {
        if self.can_move(self.robot, direction) {
            self.move_to(self.robot, direction, false);
            self.robot = self.robot.go_to(direction);
        }
    }

    fn can_move(&self, position: Position, direction: Direction) -> bool {
        let entity = self.data[position.1][position.0];
        match entity {
            Entity::Empty => true,
            Entity::Wall => false,
            Entity::LeftBox => {
                self.can_move_big_box(position, position.go_to(Direction::Right), direction)
            }
            Entity::RightBox => {
                self.can_move_big_box(position.go_to(Direction::Left), position, direction)
            }
            _ => self.can_move(position.go_to(direction), direction),
        }
    }

    fn can_move_big_box(&self, left: Position, right: Position, direction: Direction) -> bool {
        match direction {
            Direction::Left => self.can_move(left.go_to(Direction::Left), Direction::Left),
            Direction::Right => self.can_move(right.go_to(Direction::Right), Direction::Right),
            Direction::Up => {
                self.can_move(left.go_to(Direction::Up), Direction::Up)
                    && self.can_move(right.go_to(Direction::Up), Direction::Up)
            }
            Direction::Down => {
                self.can_move(left.go_to(Direction::Down), Direction::Down)
                    && self.can_move(right.go_to(Direction::Down), Direction::Down)
            }
        }
    }

    fn move_to(&mut self, position: Position, direction: Direction, is_big_box: bool) {
        let entity = self.data[position.1][position.0];
        let new_position = position.go_to(direction);
        let new_entity = self.data[new_position.1][new_position.0];

        // if new position is a box then first move the box in the direction
        if new_entity == Entity::Box
            || new_entity == Entity::LeftBox
            || new_entity == Entity::RightBox
        {
            self.move_to(new_position, direction, false);
        }

        if !is_big_box && (direction == Direction::Down || direction == Direction::Up) {
            if entity == Entity::LeftBox {
                self.move_to(position.go_to(Direction::Right), direction, true);
            }
            if entity == Entity::RightBox {
                self.move_to(position.go_to(Direction::Left), direction, true);
            }
        }

        self.data[position.1][position.0] = Entity::Empty;
        self.data[new_position.1][new_position.0] = entity;
    }

    fn get_box_coordinates(&mut self) -> usize {
        let mut sum = 0;
        for (y, column) in self.data.iter().enumerate() {
            for (x, e) in column.iter().enumerate() {
                if *e == Entity::Box || *e == Entity::LeftBox {
                    sum += self.get_gps(Position(x, y));
                }
            }
        }
        sum
    }

    fn get_gps(&self, position: Position) -> usize {
        position.0 + 100 * position.1
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map = String::new();
        for row in &self.data {
            let mut row_map = String::new();
            for entity in row {
                row_map.push(match entity {
                    Entity::Box => 'O',
                    Entity::Wall => '#',
                    Entity::LeftBox => '[',
                    Entity::RightBox => ']',
                    Entity::Robot => '@',
                    Entity::Empty => '.',
                });
            }
            map.push_str(&row_map);
            map.push('\n');
        }

        write!(f, "{map}")
    }
}

struct RobotMovements(VecDeque<Direction>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smaller_a() {
        let result = calculate_gps_sum("input_smaller.txt");
        assert!(result.is_ok());
        assert_eq!(2028, result.unwrap())
    }

    #[test]
    fn test_small_a() {
        let result = calculate_gps_sum("input_small.txt");
        assert!(result.is_ok());
        assert_eq!(10092, result.unwrap())
    }

    #[test]
    fn test_input_a() {
        let result = calculate_gps_sum("input.txt");
        assert!(result.is_ok());
        assert_eq!(1463512, result.unwrap())
    }

    #[test]
    fn test_small_b() {
        let result = calculate_gps_sum_scaled_map("input_small.txt");
        assert!(result.is_ok());
        assert_eq!(9021, result.unwrap())
    }

    #[test]
    fn test_input_b() {
        let result = calculate_gps_sum_scaled_map("input.txt");
        assert!(result.is_ok());
        assert_eq!(1486520, result.unwrap())
    }
}
