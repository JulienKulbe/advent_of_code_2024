use anyhow::Result;
use std::{collections::VecDeque, fs::read_to_string};

fn main() -> Result<()> {
    let sum = calculate_gps_sum("input.txt")?;
    println!("GPS sum: {sum}");

    let score = calculate_similarity_score("input.txt")?;
    println!("Similarity score: {score}");

    Ok(())
}

fn calculate_gps_sum(filename: &str) -> Result<usize> {
    let (mut map, mut robot) = parse_file(filename)?;

    while let Some(direction) = robot.0.pop_front() {
        map.robot_move(direction);
    }

    Ok(map.get_box_coordinates())
}

fn calculate_similarity_score(filename: &str) -> Result<u32> {
    Ok(1)
}

fn parse_file(filename: &str) -> Result<(Map, RobotMovements)> {
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

    Ok((Map::new(map), RobotMovements(movement)))
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
    fn new(data: Vec<Vec<Entity>>) -> Map {
        let mut robot = None;
        for (y, column) in data.iter().enumerate() {
            for (x, e) in column.iter().enumerate() {
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
            self.move_to(self.robot, direction);
            self.robot = self.robot.go_to(direction);
        }
    }

    fn can_move(&self, position: Position, direction: Direction) -> bool {
        let entity = self.data[position.1][position.0];
        match entity {
            Entity::Empty => true,
            Entity::Wall => false,
            _ => self.can_move(position.go_to(direction), direction),
        }
    }

    fn move_to(&mut self, position: Position, direction: Direction) {
        let entity = self.data[position.1][position.0];
        let new_position = position.go_to(direction);
        let new_entity = self.data[new_position.1][new_position.0];

        // if new position is a box then first move the box in the direction
        if new_entity == Entity::Box {
            self.move_to(new_position, direction);
        }

        self.data[position.1][position.0] = Entity::Empty;
        self.data[new_position.1][new_position.0] = entity;
    }

    fn get_box_coordinates(&mut self) -> usize {
        let mut sum = 0;
        for (y, column) in self.data.iter().enumerate() {
            for (x, e) in column.iter().enumerate() {
                if *e == Entity::Box {
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
    #[ignore = "reason"]
    fn test_small_b() {
        let result = calculate_similarity_score("input_small.txt");
        assert!(result.is_ok());
        assert_eq!(31, result.unwrap())
    }

    #[test]
    #[ignore = "reason"]
    fn test_input_b() {
        let result = calculate_similarity_score("input.txt");
        assert!(result.is_ok());
        assert_eq!(20351745, result.unwrap())
    }
}
