use anyhow::Result;
use itertools::Itertools;
use std::{collections::VecDeque, fs::read_to_string, slice::Iter};

fn main() -> Result<()> {
    let price = calculate_fencing_price("input.txt")?;
    println!("Price: {price}");

    let score = calculate_similarity_score("input.txt")?;
    println!("Similarity score: {score}");

    Ok(())
}

fn calculate_fencing_price(filename: &str) -> Result<usize> {
    let mut map = parse_file(filename)?;

    let mut regions = Vec::new();
    for (x, y) in (0..map.width).cartesian_product(0..map.height) {
        if let Field::Garden(key) = map.get(x, y) {
            let region = get_region(&map, Position::new(x, y), key);
            for pos in &region.positions {
                map.visit(*pos);
            }
            regions.push(region);
        }
    }

    let price = regions
        .iter()
        .map(|region| region.perimeter * region.area)
        .sum();

    Ok(price)
}

fn calculate_similarity_score(filename: &str) -> Result<u32> {
    Ok(1)
}

fn parse_file(filename: &str) -> Result<Map> {
    let file = read_to_string(filename)?;
    let data: Vec<Vec<Field>> = file
        .lines()
        .map(|line| line.chars().map(Field::Garden).collect())
        .collect();
    let width = data[0].len();
    let height = data.len();

    Ok(Map {
        data,
        width,
        height,
    })
}

fn get_region(map: &Map, pos: Position, key: char) -> Region {
    let positions = get_region_positions(map, pos, key);
    let area = positions.len();
    let perimeter = get_region_perimeter(&positions);
    Region {
        key,
        positions,
        area,
        perimeter,
    }
}

fn get_region_positions(map: &Map, pos: Position, key: char) -> Vec<Position> {
    let mut closed_map = Vec::new();
    let mut open_map = VecDeque::from([pos]);

    while let Some(pos) = open_map.pop_front() {
        closed_map.push(pos);

        for direction in Direction::get_all_directions() {
            let next = pos.go_to(*direction);
            if let Some(Field::Garden(f)) = map.get_pos(next) {
                if f == key && !closed_map.contains(&next) && !open_map.contains(&next) {
                    open_map.push_back(next);
                }
            }
        }
    }

    closed_map
}

fn get_region_perimeter(positions: &Vec<Position>) -> usize {
    let mut perimeter = 0;
    for pos in positions {
        for direction in Direction::get_all_directions() {
            let near = pos.go_to(*direction);
            if !positions.contains(&near) {
                perimeter += 1;
            }
        }
    }

    perimeter
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: usize, y: usize) -> Position {
        Position {
            x: x as i32,
            y: y as i32,
        }
    }

    fn go_to(&self, direction: Direction) -> Position {
        match direction {
            Direction::Up => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Position {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Position {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

struct Region {
    key: char,
    positions: Vec<Position>,
    area: usize,
    perimeter: usize,
}

struct Map {
    data: Vec<Vec<Field>>,
    width: usize,
    height: usize,
}

impl Map {
    fn get(&self, x: usize, y: usize) -> Field {
        self.data[y][x]
    }

    fn get_pos(&self, pos: Position) -> Option<Field> {
        if pos.x >= 0 && pos.y >= 0 && pos.x < self.width as i32 && pos.y < self.height as i32 {
            Some(self.data[pos.y as usize][pos.x as usize])
        } else {
            None
        }
    }

    fn visit(&mut self, pos: Position) {
        self.data[pos.y as usize][pos.x as usize] = Field::Visited;
    }
}

#[derive(Debug, Clone, Copy)]
enum Field {
    Visited,
    Garden(char),
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn get_all_directions() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];
        DIRECTIONS.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_a() {
        let result = calculate_fencing_price("input_small_a.txt");
        assert!(result.is_ok());
        assert_eq!(140, result.unwrap())
    }

    #[test]
    fn test_small_b() {
        let result = calculate_fencing_price("input_small_b.txt");
        assert!(result.is_ok());
        assert_eq!(772, result.unwrap())
    }

    #[test]
    fn test_small_c() {
        let result = calculate_fencing_price("input_small_c.txt");
        assert!(result.is_ok());
        assert_eq!(1930, result.unwrap())
    }

    #[test]
    fn test_input_a() {
        let result = calculate_fencing_price("input.txt");
        assert!(result.is_ok());
        assert_eq!(1550156, result.unwrap())
    }

    #[test]
    #[ignore = "reason"]
    fn test_input_b() {
        let result = calculate_similarity_score("input.txt");
        assert!(result.is_ok());
        assert_eq!(20351745, result.unwrap())
    }
}
