use anyhow::Result;
use regex::Regex;
use std::{cmp::Ordering, fs::read_to_string};

fn main() -> Result<()> {
    let distance = calculate_safety_factor("input.txt", 101, 103);
    println!("Total distance: {distance}");

    let score = calculate_similarity_score("input.txt")?;
    println!("Similarity score: {score}");

    Ok(())
}

fn calculate_safety_factor(filename: &str, width: i32, height: i32) -> usize {
    let mut map = Map::new(filename, width, height);
    for _ in 0..100 {
        map.simulate();
    }

    map.safety_factor()
}

fn calculate_similarity_score(filename: &str) -> Result<u32> {
    Ok(1)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(i32, i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Velocity(i32, i32);

struct Robot {
    position: Position,
    velocity: Velocity,
}

struct Map {
    robots: Vec<Robot>,
    width: i32,
    height: i32,
}

impl Map {
    fn new(filename: &str, width: i32, height: i32) -> Map {
        let data = read_to_string(filename).unwrap();
        let robot_regex = Regex::new(r"p=(\d*),(\d*) v=(-?\d*),(-?\d*)").unwrap();
        let robots = robot_regex
            .captures_iter(&data)
            .map(|c| c.extract())
            .map(|(_, [xpos, ypos, xvel, yvel])| {
                let position = Position(xpos.parse().unwrap(), ypos.parse().unwrap());
                let velocity = Velocity(xvel.parse().unwrap(), yvel.parse().unwrap());
                Robot { position, velocity }
            })
            .collect();

        Map {
            robots,
            width,
            height,
        }
    }

    fn simulate(&mut self) {
        for robot in &mut self.robots {
            let mut x = robot.position.0 + robot.velocity.0;
            if x < 0 {
                x += self.width;
            }
            if x >= self.width {
                x -= self.width;
            }

            let mut y = robot.position.1 + robot.velocity.1;
            if y < 0 {
                y += self.height;
            }
            if y >= self.height {
                y -= self.height;
            }

            robot.position = Position(x, y);
        }
    }

    fn safety_factor(&self) -> usize {
        let mut quadrants = [0, 0, 0, 0];
        for robot in &self.robots {
            match (
                robot.position.0.cmp(&(self.width / 2)),
                robot.position.1.cmp(&(self.height / 2)),
            ) {
                (Ordering::Less, Ordering::Less) => {
                    quadrants[0] += 1;
                }
                (Ordering::Less, Ordering::Greater) => {
                    quadrants[1] += 1;
                }
                (Ordering::Greater, Ordering::Less) => {
                    quadrants[2] += 1;
                }
                (Ordering::Greater, Ordering::Greater) => {
                    quadrants[3] += 1;
                }
                (_, _) => {}
            }
        }

        quadrants.iter().product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_a() {
        let result = calculate_safety_factor("input_small.txt", 11, 7);
        assert_eq!(12, result)
    }

    #[test]
    fn test_input_a() {
        let result = calculate_safety_factor("input.txt", 101, 103);
        assert_eq!(224554908, result)
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
