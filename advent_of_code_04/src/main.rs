use anyhow::Result;
use std::{fs::read_to_string, slice::Iter};

fn main() -> Result<()> {
    let sum = count_xmas("input.txt")?;
    println!("Xmas found: {sum}");

    let sum = count_x_mas("input.txt")?;
    println!("X-Mas found: {sum}");

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

fn count_x_mas(filename: &str) -> Result<usize> {
    let map = Map::new(filename);

    let mut sum = 0;
    for x in 0..map.width {
        for y in 0..map.height {
            let pos = Position(x, y);
            if map.get(pos) == Some('A') {
                let top_left = map.go_to(pos, Direction::TopLeft);
                let down_left = map.go_to(pos, Direction::DownLeft);
                let top_right = map.go_to(pos, Direction::TopRight);
                let down_right = map.go_to(pos, Direction::DownRight);

                let mut diagonal1 = [map.get(top_left), map.get(down_right)];
                let mut diagonal2 = [map.get(top_right), map.get(down_left)];

                diagonal1.sort_unstable();
                diagonal2.sort_unstable();

                if diagonal1 == [Some('M'), Some('S')] && diagonal2 == [Some('M'), Some('S')] {
                    sum += 1;
                }
            }
        }
    }

    Ok(sum)
}

fn count_xmas_from_pos(map: &Map, pos: Position) -> usize {
    Direction::get_all_directions()
        .filter(|direction| contains_xmas(map, pos, **direction))
        .count()
}

fn contains_xmas(map: &Map, mut pos: Position, direction: Direction) -> bool {
    let xmas = ['X', 'M', 'A', 'S'];

    for character in xmas {
        if map.get(pos) != Some(character) {
            return false;
        }
        pos = map.go_to(pos, direction);
    }

    true
}

#[derive(Debug, Clone, Copy)]
struct Position(i32, i32);

struct Map {
    data: Vec<Vec<char>>,
    height: i32,
    width: i32,
}

impl Map {
    fn new(filename: &str) -> Map {
        let file = read_to_string(filename).unwrap();
        let data: Vec<Vec<char>> = file.lines().map(|line| line.chars().collect()).collect();
        let height = data.len() as i32;
        let width = data.first().unwrap().len() as i32;

        Map {
            data,
            height,
            width,
        }
    }

    fn get(&self, position: Position) -> Option<char> {
        if position.0 >= 0 && position.0 < self.height && position.1 >= 0 && position.1 < self.width
        {
            let c = self.data[position.0 as usize][position.1 as usize];
            Some(c)
        } else {
            None
        }
    }

    fn go_to(&self, position: Position, direction: Direction) -> Position {
        match direction {
            Direction::Top => Position(position.0, position.1 - 1),
            Direction::Down => Position(position.0, position.1 + 1),
            Direction::Left => Position(position.0 - 1, position.1),
            Direction::Right => Position(position.0 + 1, position.1),
            Direction::TopLeft => Position(position.0 - 1, position.1 - 1),
            Direction::TopRight => Position(position.0 + 1, position.1 - 1),
            Direction::DownLeft => Position(position.0 - 1, position.1 + 1),
            Direction::DownRight => Position(position.0 + 1, position.1 + 1),
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
    fn test_small_b() {
        let result = count_x_mas("input_small_b.txt");
        assert!(result.is_ok());
        assert_eq!(9, result.unwrap())
    }

    #[test]
    fn test_input_b() {
        let result = count_x_mas("input.txt");
        assert!(result.is_ok());
        assert_eq!(1950, result.unwrap())
    }
}
