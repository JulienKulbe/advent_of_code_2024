use anyhow::Result;
use itertools::Itertools;
use std::fs::read_to_string;

fn main() -> Result<()> {
    let score = get_lowest_score("input.txt")?;
    println!("Lowest score: {score}");

    let score = calculate_similarity_score("input.txt")?;
    println!("Similarity score: {score}");

    Ok(())
}

fn get_lowest_score(filename: &str) -> Result<usize> {
    let map = parse_file(filename)?;
    let score = Graph::find_shortest_path(&map);
    Ok(score)
}

fn calculate_similarity_score(filename: &str) -> Result<u32> {
    Ok(1)
}

fn parse_file(filename: &str) -> Result<Map> {
    let file = read_to_string(filename)?;
    let data = file
        .lines()
        .map(|line| line.chars().map(Field::from).collect())
        .collect();

    Ok(Map { data })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Field {
    Empty,
    Wall,
    Start,
    End,
}

impl From<char> for Field {
    fn from(c: char) -> Self {
        match c {
            '#' => Field::Wall,
            '.' => Field::Empty,
            'S' => Field::Start,
            'E' => Field::End,
            _ => panic!("Undefined Field"),
        }
    }
}

struct Map {
    data: Vec<Vec<Field>>,
}

impl Map {
    fn get(&self, pos: Position) -> Field {
        self.data[pos.1][pos.0]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position(usize, usize);

impl Position {
    fn get_neighbors(&self) -> [Position; 4] {
        [
            Position(self.0 + 1, self.1),
            Position(self.0 - 1, self.1),
            Position(self.0, self.1 + 1),
            Position(self.0, self.1 - 1),
        ]
    }
}

struct Graph {
    data: Vec<Position>,
    dist: Vec<usize>,
    prev: Vec<Option<usize>>,
    queue: Vec<Option<usize>>,
    start: Position,
    end: Position,
}

impl Graph {
    fn find_shortest_path(map: &Map) -> usize {
        let mut graph = Graph::new(map);
        graph.calculate_dijkstra();
        graph.cost_to_end()
    }

    fn new(map: &Map) -> Graph {
        let height = map.data.len();
        let width = map.data[0].len();

        let data: Vec<Position> = (0..width)
            .cartesian_product(0..height)
            .map(|(x, y)| Position(x, y))
            .filter(|pos| map.get(*pos) != Field::Wall)
            .collect();

        let dist = vec![usize::MAX; data.len()];
        let prev = vec![None; data.len()];
        let queue = data.iter().enumerate().map(|(i, _)| Some(i)).collect();

        let start = *data
            .iter()
            .find(|&&pos| map.get(pos) == Field::Start)
            .expect("No start field");
        let end = *data
            .iter()
            .find(|&&pos| map.get(pos) == Field::End)
            .expect("No end field");

        Graph {
            data,
            dist,
            prev,
            queue,
            start,
            end,
        }
    }

    fn calculate_dijkstra(&mut self) {
        let start = self.index(self.start).unwrap();
        self.dist[start] = 0;

        while self.queue.iter().any(|i| i.is_some()) {
            let u = self.get_minimum_dist();
            self.queue[u] = None;

            for neighbor in self.data[u].get_neighbors() {
                if let Some(v) = self.index(neighbor) {
                    if let Some(v) = self.queue[v] {
                        let alt = self.dist[u] + self.get_cost(u, v);
                        if alt < self.dist[v] {
                            self.dist[v] = alt;
                            self.prev[v] = Some(u);
                        }
                    }
                }
            }
        }
    }

    fn cost_to_end(&self) -> usize {
        let end = self.index(self.end).unwrap();
        self.dist[end]
    }

    fn index(&self, position: Position) -> Option<usize> {
        self.data.iter().position(|&pos| pos == position)
    }

    fn get_minimum_dist(&self) -> usize {
        self.queue
            .iter()
            .zip(self.dist.iter())
            .filter(|(&i, _)| i.is_some())
            .min_by_key(|(_, &dist)| dist)
            .map(|(i, _)| i.unwrap())
            .unwrap()
    }

    fn get_cost(&self, curr: usize, next: usize) -> usize {
        let next = self.data[next];
        let previous = self.prev[curr];
        let previous = if let Some(index) = previous {
            self.data[index]
        } else {
            let current = self.data[curr];
            Position(current.0 - 1, current.1)
        };

        if previous.0 == next.0 || previous.1 == next.1 {
            1
        } else {
            1001
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_a() {
        let result = get_lowest_score("input_small.txt");
        assert!(result.is_ok());
        assert_eq!(7036, result.unwrap())
    }

    #[test]
    fn test_small_2_a() {
        let result = get_lowest_score("input_small_2.txt");
        assert!(result.is_ok());
        assert_eq!(11048, result.unwrap())
    }

    #[test]
    fn test_input_a() {
        let result = get_lowest_score("input.txt");
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
