use anyhow::Result;
use std::{fs::read_to_string, slice::Iter};

fn main() -> Result<()> {
    let sum = count_xmas("input.txt")?;
    println!("Xmas found: {sum}");

    let score = calculate_similarity_score("input.txt")?;
    println!("Similarity score: {score}");

    Ok(())
}

fn count_xmas(filename: &str) -> Result<usize> {
    let map = Map::new(filename);

    let mut sum = 0;
    for x in 0..map.width {
        for y in 0..map.height {
            sum += count_xmas_from_pos(&map, Position(x, y));
        }
    }

    Ok(sum)
}

fn calculate_similarity_score(filename: &str) -> Result<u32> {
    Ok(1)
}

fn count_xmas_from_pos(map: &Map, pos: Position) -> usize {
    Direction::get_all_directions()
        .filter(|direction| contains_xmas(map, pos, **direction))
        .count()
}

fn contains_xmas(map: &Map, pos: Position, direction: Direction) -> bool {
    let xmas = ['X', 'M', 'A', 'S'];

    let mut current = Some(pos);
    for character in xmas {
        if let Some(pos) = current {
            if map.get(pos) != character {
                return false;
            }
            current = map.go_to(pos, direction);
        } else {
            return false;
        }
    }

    true
}

#[derive(Debug, Clone, Copy)]
struct Position(usize, usize);

struct Map {
    data: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

impl Map {
    fn new(filename: &str) -> Map {
        let file = read_to_string(filename).unwrap();
        let data: Vec<Vec<char>> = file.lines().map(|line| line.chars().collect()).collect();
        let height = data.len();
        let width = data.first().unwrap().len();

        Map {
            data,
            height,
            width,
        }
    }

    fn get(&self, position: Position) -> char {
        self.data[position.0][position.1]
    }

    fn go_to(&self, position: Position, direction: Direction) -> Option<Position> {
        match direction {
            Direction::Top => {
                if position.1 > 0 {
                    Some(Position(position.0, position.1 - 1))
                } else {
                    None
                }
            }
            Direction::Down => {
                if position.1 < self.height - 1 {
                    Some(Position(position.0, position.1 + 1))
                } else {
                    None
                }
            }
            Direction::Left => {
                if position.0 > 0 {
                    Some(Position(position.0 - 1, position.1))
                } else {
                    None
                }
            }
            Direction::Right => {
                if position.0 < self.width - 1 {
                    Some(Position(position.0 + 1, position.1))
                } else {
                    None
                }
            }
            Direction::TopLeft => {
                if position.0 > 0 && position.1 > 0 {
                    Some(Position(position.0 - 1, position.1 - 1))
                } else {
                    None
                }
            }
            Direction::TopRight => {
                if position.0 < self.width - 1 && position.1 > 0 {
                    Some(Position(position.0 + 1, position.1 - 1))
                } else {
                    None
                }
            }
            Direction::DownLeft => {
                if position.0 > 0 && position.1 < self.height - 1 {
                    Some(Position(position.0 - 1, position.1 + 1))
                } else {
                    None
                }
            }
            Direction::DownRight => {
                if position.0 < self.width - 1 && position.1 < self.height - 1 {
                    Some(Position(position.0 + 1, position.1 + 1))
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Top,
    Down,
    Left,
    Right,
    TopLeft,
    TopRight,
    DownLeft,
    DownRight,
}

impl Direction {
    fn get_all_directions() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 8] = [
            Direction::Top,
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::TopLeft,
            Direction::TopRight,
            Direction::DownLeft,
            Direction::DownRight,
        ];
        DIRECTIONS.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_a() {
        let result = count_xmas("input_small.txt");
        assert!(result.is_ok());
        assert_eq!(18, result.unwrap())
    }

    #[test]
    fn test_input_a() {
        let result = count_xmas("input.txt");
        assert!(result.is_ok());
        assert_eq!(2593, result.unwrap())
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
