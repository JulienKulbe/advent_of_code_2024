use anyhow::Result;
use regex::Regex;
use std::{cmp::Ordering, fmt::Debug, fs::read_to_string};

fn main() -> Result<()> {
    //let distance = calculate_safety_factor("input_small.txt", 11, 7);
    let safety = calculate_safety_factor("input.txt", 101, 103);
    println!("Safety Factor: {safety}");

    let tree = find_christmas_tree("input.txt", 101, 103);
    println!("Tree: {tree}");

    Ok(())
}

fn calculate_safety_factor(filename: &str, width: i32, height: i32) -> usize {
    let mut map = Map::new(filename, width, height);

    for _ in 0..1000 {
        map.simulate();
    }

    map.safety_factor()
}

fn find_christmas_tree(filename: &str, width: i32, height: i32) -> usize {
    let mut map = Map::new(filename, width, height);

    for i in 0..10000 {
        map.simulate();
        let mut flood_map = FloodMap::new(&map);
        flood_map.fill();

        if flood_map.dry_robots() > 5 {
            println!("{:?}", map);
            return i + 1;
        }
    }
    panic!("No tree found")
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

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut data = String::new();
        for y in 0..self.height {
            let mut line = ".".repeat(self.width as usize) + "\n";
            for robot in &self.robots {
                if robot.position.1 == y {
                    let x = robot.position.0 as usize;
                    line.replace_range(x..x + 1, "O");
                }
            }

            data += &line;
        }
        write!(f, "{data}")
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FloodField {
    Dry,
    Filled,
    DryRobot,
}

struct FloodMap {
    map: Vec<Vec<FloodField>>,
    width: i32,
    height: i32,
}

impl FloodMap {
    fn new(map: &Map) -> FloodMap {
        // create dry flood map
        let mut flood_map: Vec<Vec<FloodField>> = (0..map.height)
            .map(|_| vec![FloodField::Dry; map.width as usize])
            .collect();

        // fill every robot with water
        for robot in &map.robots {
            flood_map[robot.position.1 as usize][robot.position.0 as usize] = FloodField::DryRobot;
        }

        FloodMap {
            map: flood_map,
            width: map.width,
            height: map.height,
        }
    }

    fn fill(&mut self) {
        self.fill_from_pos(Position(0, 0));
        self.fill_from_pos(Position(self.width - 1, 0));
        self.fill_from_pos(Position(0, self.height - 1));
        self.fill_from_pos(Position(self.width - 1, self.height - 1));
    }

    fn fill_from_pos(&mut self, pos: Position) {
        // illegal position
        if pos.0 < 0 || pos.1 < 0 || pos.0 >= self.width || pos.1 >= self.height {
            return;
        }

        let field = &mut self.map[pos.1 as usize][pos.0 as usize];
        if *field == FloodField::Filled {
            return;
        }

        let was_robot = *field == FloodField::DryRobot;
        *field = FloodField::Filled;

        // only fill robot but don't continue
        // so that only the outer robots will get wet
        if was_robot {
            return;
        }

        // fill neighbor fields
        self.fill_from_pos(Position(pos.0 - 1, pos.1));
        self.fill_from_pos(Position(pos.0 + 1, pos.1));
        self.fill_from_pos(Position(pos.0, pos.1 - 1));
        self.fill_from_pos(Position(pos.0, pos.1 + 1));
    }

    fn dry_robots(&self) -> usize {
        let mut sum = 0;
        for y in 0..self.height as usize {
            for x in 0..self.width as usize {
                if self.map[y][x] == FloodField::DryRobot {
                    sum += 1;
                }
            }
        }
        sum
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
    fn test_flood_fill() {
        let mut robots = Vec::new();
        for y in 2..=6 {
            for x in 2..=6 {
                robots.push(Robot {
                    position: Position(x, y),
                    velocity: Velocity(0, 0),
                });
            }
        }

        let map = Map {
            robots,
            width: 10,
            height: 10,
        };
        let mut flood_map = FloodMap::new(&map);
        flood_map.fill();
        assert_ne!(0, flood_map.dry_robots())
    }

    #[test]
    fn test_input_b() {
        let result = find_christmas_tree("input.txt", 101, 103);
        assert_eq!(6644, result)
    }
}
