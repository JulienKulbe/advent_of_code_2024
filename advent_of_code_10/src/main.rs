use anyhow::Result;
use itertools::Itertools;
use std::{fs::read_to_string, slice::Iter};

fn main() -> Result<()> {
    let count = calculate_trailheads("input.txt")?;
    println!("Trailheads: {count}");

    let rating = calculate_trailhead_ratings("input.txt")?;
    println!("Rating: {rating}");

    Ok(())
}

fn calculate_trailheads(filename: &str) -> Result<usize> {
    let map = Map::new(filename)?;

    let trailheads = map
        .get_starting_points()
        .iter()
        .map(|&start| map.get_trailheads(start))
        .sum();

    Ok(trailheads)
}

fn calculate_trailhead_ratings(filename: &str) -> Result<usize> {
    let map = Map::new(filename)?;

    let trailheads = map
        .get_starting_points()
        .iter()
        .map(|&start| map.get_trailhead_ratings(start))
        .sum();

    Ok(trailheads)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(i32, i32);

struct Map {
    data: Vec<Vec<i8>>,
    width: usize,
    heigth: usize,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn all_directions() -> Iter<'static, Self> {
        static DIRECTIONS: [Direction; 4] = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];
        DIRECTIONS.iter()
    }

    fn go_to(&self, pos: Position) -> Position {
        match self {
            Self::Up => Position(pos.0, pos.1 - 1),
            Self::Down => Position(pos.0, pos.1 + 1),
            Self::Left => Position(pos.0 - 1, pos.1),
            Self::Right => Position(pos.0 + 1, pos.1),
        }
    }
}

impl Map {
    fn new(filename: &str) -> Result<Self> {
        let file = read_to_string(filename)?;
        let data: Vec<Vec<i8>> = file
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).expect("Expected digit") as i8)
                    .collect()
            })
            .collect();
        let width = data[0].len();
        let heigth = data.len();

        Ok(Map {
            data,
            width,
            heigth,
        })
    }

    fn get(&self, pos: Position) -> Option<i8> {
        if pos.0 >= 0 && pos.1 >= 0 && pos.0 < self.width as i32 && pos.1 < self.heigth as i32 {
            Some(self.data[pos.1 as usize][pos.0 as usize])
        } else {
            None
        }
    }

    fn get_starting_points(&self) -> Vec<Position> {
        (0..self.width)
            .cartesian_product(0..self.heigth)
            .filter(|(x, y)| self.data[*y][*x] == 0)
            .map(|(x, y)| Position(x as i32, y as i32))
            .collect()
    }

    fn get_trailheads(&self, start: Position) -> usize {
        self.get_trailheads_internal(start, -1)
            .iter()
            .unique()
            .count()
    }

    fn get_trailhead_ratings(&self, start: Position) -> usize {
        self.get_trailhead_ratings_internal(start, -1)
    }

    fn get_trailheads_internal(&self, position: Position, previous: i8) -> Vec<Position> {
        let value = self.get(position);
        let mut trailheads = Vec::new();
        if let Some(value) = value {
            if value == previous + 1 {
                if value == 9 {
                    trailheads.push(position);
                } else {
                    for direction in Direction::all_directions() {
                        let next = direction.go_to(position);
                        let mut th = self.get_trailheads_internal(next, value);
                        trailheads.append(&mut th);
                    }
                }
            }
        }
        trailheads
    }

    fn get_trailhead_ratings_internal(&self, position: Position, previous: i8) -> usize {
        let value = self.get(position);
        if let Some(value) = value {
            if value != previous + 1 {
                0
            } else if value == 9 {
                1
            } else {
                Direction::all_directions()
                    .map(|direction| direction.go_to(position))
                    .map(|next| self.get_trailhead_ratings_internal(next, value))
                    .sum()
            }
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_a() {
        let result = calculate_trailheads("input_small.txt");
        assert!(result.is_ok());
        assert_eq!(36, result.unwrap())
    }

    #[test]
    fn test_input_a() {
        let result = calculate_trailheads("input.txt");
        assert!(result.is_ok());
        assert_eq!(531, result.unwrap())
    }

    #[test]
    fn test_small_b() {
        let result = calculate_trailhead_ratings("input_small.txt");
        assert!(result.is_ok());
        assert_eq!(81, result.unwrap())
    }

    #[test]
    fn test_input_b() {
        let result = calculate_trailhead_ratings("input.txt");
        assert!(result.is_ok());
        assert_eq!(1210, result.unwrap())
    }
}
