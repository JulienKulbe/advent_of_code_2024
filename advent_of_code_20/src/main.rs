use anyhow::Result;
use itertools::Itertools;
use std::{collections::HashSet, fs::read_to_string};

fn main() -> Result<()> {
    let distance = calculate_racetrack_options("input.txt", 100)?;
    println!("Total distance: {distance}");

    let score = calculate_similarity_score("input.txt")?;
    println!("Similarity score: {score}");

    Ok(())
}

fn calculate_racetrack_options(filename: &str, save_time: usize) -> Result<u32> {
    let map = Map::new(filename)?;
    let race_track = RaceTrack::new(&map);

    let mut sum = 0;
    for (index, pos) in &race_track.neighbors {
        let time = race_track.remove_wall(*index, *pos);
        if time >= save_time {
            sum += 1;
        }
    }

    Ok(sum)
}

fn calculate_similarity_score(filename: &str) -> Result<u32> {
    Ok(1)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(usize, usize);

impl Position {
    fn get_neigbors(&self) -> [Position; 4] {
        [
            Position(self.0 + 1, self.1),
            Position(self.0 - 1, self.1),
            Position(self.0, self.1 + 1),
            Position(self.0, self.1 - 1),
        ]
    }

    fn next_position(&self, previous: Position) -> Position {
        let delta_x = self.0 as i64 - previous.0 as i64;
        let delta_y = self.1 as i64 - previous.1 as i64;
        let x = self.0 as i64 + delta_x;
        let y = self.1 as i64 + delta_y;
        Position(x as usize, y as usize)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Field {
    Track,
    Wall,
    Start,
    End,
}

impl From<char> for Field {
    fn from(c: char) -> Self {
        match c {
            '#' => Field::Wall,
            '.' => Field::Track,
            'S' => Field::Start,
            'E' => Field::End,
            _ => panic!("Indvalid character found for Field"),
        }
    }
}

struct Map {
    data: Vec<Vec<Field>>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(filename: &str) -> Result<Map> {
        let file = read_to_string(filename)?;
        let data: Vec<Vec<Field>> = file
            .lines()
            .map(|line| line.chars().map(Field::from).collect())
            .collect();
        let width = data[0].len();
        let height = data.len();

        Ok(Map {
            data,
            width,
            height,
        })
    }

    fn get(&self, pos: Position) -> Field {
        self.data[pos.1][pos.0]
    }
}

struct RaceTrack {
    track: Vec<Position>,
    neighbors: HashSet<(usize, Position)>,
}

impl RaceTrack {
    fn new(map: &Map) -> RaceTrack {
        let start = (0..map.width)
            .cartesian_product(0..map.height)
            .map(|(x, y)| Position(x, y))
            .find(|pos| map.get(*pos) == Field::Start)
            .expect("No start found on race track");
        let end = (0..map.width)
            .cartesian_product(0..map.height)
            .map(|(x, y)| Position(x, y))
            .find(|pos| map.get(*pos) == Field::End)
            .expect("No end found on race track");

        let mut track = Vec::new();
        let mut neighbors = HashSet::new();
        let mut previous = start;
        let mut current = start;

        while current != end {
            let current_neighbors = current.get_neigbors();
            let index = track.len();

            let mut next = None;
            for current_neighbor in current_neighbors {
                let field = map.get(current_neighbor);
                match field {
                    Field::Wall => {
                        neighbors.insert((index, current_neighbor));
                    }
                    Field::Track | Field::End | Field::Start => {
                        if current_neighbor != previous {
                            next = Some(current_neighbor);
                        }
                    }
                }
            }

            track.push(current);
            previous = current;
            current = next.expect("No next field found");
        }

        track.push(end);

        RaceTrack { track, neighbors }
    }

    fn remove_wall(&self, index: usize, current: Position) -> usize {
        // if we remove a wall the next position should also be
        // on the track to find a valid shortcut
        let previous = self.track[index];
        let next = current.next_position(previous);

        let next_index = self.track.iter().position(|&pos| pos == next);
        if let Some(next_index) = next_index {
            if next_index > index {
                next_index - index - 2
            } else {
                0
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
    fn test_remove_invalid_wall() {
        let map = Map::new("input_small.txt").unwrap();
        let race_track = RaceTrack::new(&map);
        let race_time = race_track.remove_wall(6, Position(3, 4));
        assert_eq!(0, race_time)
    }

    #[test]
    fn test_remove_wall_1() {
        let map = Map::new("input_small.txt").unwrap();
        let race_track = RaceTrack::new(&map);
        let race_time = race_track.remove_wall(12, Position(8, 1));
        assert_eq!(12, race_time)
    }

    #[test]
    fn test_remove_wall_2() {
        let map = Map::new("input_small.txt").unwrap();
        let race_track = RaceTrack::new(&map);
        let race_time = race_track.remove_wall(18, Position(6, 7));
        assert_eq!(64, race_time)
    }

    #[test]
    fn test_small_a() {
        let result = calculate_racetrack_options("input_small.txt", 20);
        assert!(result.is_ok());
        assert_eq!(5, result.unwrap())
    }

    #[test]
    fn test_input_a() {
        let result = calculate_racetrack_options("input.txt", 100);
        assert!(result.is_ok());
        assert_eq!(1579939, result.unwrap())
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
