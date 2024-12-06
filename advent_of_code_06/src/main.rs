use anyhow::{bail, Result};
use std::fs::read_to_string;

fn main() -> Result<()> {
    let sum = number_distinct_positions("input.txt")?;
    println!("Sum: {sum}");

    let score = calculate_similarity_score("input.txt")?;
    println!("Similarity score: {score}");

    Ok(())
}

fn number_distinct_positions(filename: &str) -> Result<usize> {
    let mut map = Map::new(filename)?;
    let mut guard = Guard::new(&map)?;

    while let Some(pos) = guard.position {
        //println!("{:?}", pos);
        map.visit(pos);
        guard.go(&map);
    }

    Ok(map.visited())
}

fn calculate_similarity_score(filename: &str) -> Result<u32> {
    Ok(1)
}

struct Map {
    data: Vec<Vec<Field>>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(filename: &str) -> Result<Self> {
        let file = read_to_string(filename)?;
        let data: Vec<Vec<Field>> = file
            .lines()
            .map(|line| line.chars().map(Field::from).collect())
            .collect();
        let height = data.len();
        let width = data.first().unwrap().len();

        Ok(Map {
            data,
            width,
            height,
        })
    }

    fn visited(&self) -> usize {
        self.data
            .iter()
            .map(|row| row.iter().filter(|f| **f == Field::Visited).count())
            .sum()
    }

    fn get(&self, pos: Position) -> Field {
        self.data[pos.1][pos.0]
    }

    fn visit(&mut self, pos: Position) {
        self.data[pos.1][pos.0] = Field::Visited;
    }
}

#[derive(Debug, Clone, Copy)]
struct Position(usize, usize);

struct Guard {
    position: Option<Position>,
    direction: Direction,
}

impl Guard {
    fn new(map: &Map) -> Result<Guard> {
        for y in 0..map.height {
            for x in 0..map.width {
                if map.get(Position(x, y)) == Field::Guard {
                    return Ok(Guard {
                        position: Some(Position(x, y)),
                        direction: Direction::Up,
                    });
                }
            }
        }
        bail!("No guard found")
    }

    fn go(&mut self, map: &Map) {
        let last_position = self.position.unwrap();

        self.position = match self.direction {
            Direction::Up => {
                if last_position.1 == 0 {
                    None
                } else {
                    Some(Position(last_position.0, last_position.1 - 1))
                }
            }
            Direction::Down => {
                if last_position.1 >= map.height - 1 {
                    None
                } else {
                    Some(Position(last_position.0, last_position.1 + 1))
                }
            }
            Direction::Left => {
                if last_position.0 == 0 {
                    None
                } else {
                    Some(Position(last_position.0 - 1, last_position.1))
                }
            }
            Direction::Right => {
                if last_position.0 >= map.width - 1 {
                    None
                } else {
                    Some(Position(last_position.0 + 1, last_position.1))
                }
            }
        };

        if let Some(pos) = self.position {
            if map.get(pos) == Field::Blocked {
                self.direction = self.direction.next();
                self.position = Some(last_position);
            }
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next(&self) -> Direction {
        match self {
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Field {
    Empty,
    Visited,
    Blocked,
    Guard,
}

impl From<char> for Field {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Blocked,
            '^' => Self::Guard,
            _ => panic!("found invalid character {c}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_a() {
        let result = number_distinct_positions("input_small.txt");
        assert!(result.is_ok());
        assert_eq!(41, result.unwrap())
    }

    #[test]
    fn test_input_a() {
        let result = number_distinct_positions("input.txt");
        assert!(result.is_ok());
        assert_eq!(4656, result.unwrap())
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
