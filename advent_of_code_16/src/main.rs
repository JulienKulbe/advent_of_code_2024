use anyhow::Result;
use itertools::Itertools;
use std::fs::read_to_string;

fn main() -> Result<()> {
    let score = get_lowest_score("input.txt")?;
    println!("Lowest score: {score}");

    let path = get_shortest_path("input.txt")?;
    println!("Shortest path: {path}");

    Ok(())
}

fn get_lowest_score(filename: &str) -> Result<usize> {
    let map = parse_file(filename)?;
    let score = Graph::find_shortest_path_cost(&map);
    Ok(score)
}

fn get_shortest_path(filename: &str) -> Result<usize> {
    let map = parse_file(filename)?;
    let score = Graph::find_shortest_path_length(&map);
    Ok(score)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    position: Position,
    distance: usize,
    previous: Option<usize>,
    visited: bool,
}

impl Node {
    fn new(position: Position) -> Self {
        Node {
            position,
            distance: usize::MAX,
            previous: None,
            visited: false,
        }
    }
}

struct Graph {
    nodes: Vec<Node>,
    start: usize,
    end: usize,
}

impl Graph {
    fn find_shortest_path_cost(map: &Map) -> usize {
        let mut graph = Graph::new(map);
        graph.calculate_dijkstra();
        graph.cost_to_end()
    }

    fn find_shortest_path_length(map: &Map) -> usize {
        let mut graph = Graph::new(map);
        graph.calculate_dijkstra();
        graph.path_lenth()
    }

    fn new(map: &Map) -> Graph {
        let height = map.data.len();
        let width = map.data[0].len();

        let nodes: Vec<Node> = (0..width)
            .cartesian_product(0..height)
            .map(|(x, y)| Position(x, y))
            .filter(|pos| map.get(*pos) != Field::Wall)
            .map(Node::new)
            .collect();

        let start = nodes
            .iter()
            .position(|node| map.get(node.position) == Field::Start)
            .expect("No start field");
        let end = nodes
            .iter()
            .position(|node| map.get(node.position) == Field::End)
            .expect("No end field");

        Graph { nodes, start, end }
    }

    fn calculate_dijkstra(&mut self) {
        self.nodes[self.start].distance = 0;

        while let Some(u) = self.get_minimum_dist() {
            self.nodes[u].visited = true;

            for neighbor in self.nodes[u].position.get_neighbors() {
                if let Some(v) = self.index(neighbor) {
                    let alt = self.nodes[u].distance + self.get_cost(self.nodes[u], self.nodes[v]);
                    if alt < self.nodes[v].distance {
                        self.nodes[v].distance = alt;
                        self.nodes[v].previous = Some(u);
                    }
                }
            }
        }
    }

    fn cost_to_end(&self) -> usize {
        self.nodes[self.end].distance
    }

    fn path_lenth(&self) -> usize {
        let mut length = 0;
        let mut current = Some(self.end);

        while let Some(index) = current {
            length += 1;
            current = self.nodes[index].previous;
        }

        length
    }

    fn index(&self, position: Position) -> Option<usize> {
        self.nodes.iter().position(|node| node.position == position)
    }

    fn get_minimum_dist(&self) -> Option<usize> {
        let node = self
            .nodes
            .iter()
            .filter(|node| !node.visited)
            .min_by_key(|node| node.distance);
        if let Some(node) = node {
            self.index(node.position)
        } else {
            None
        }
    }

    fn get_cost(&self, curr: Node, next: Node) -> usize {
        let previous = if let Some(index) = curr.previous {
            self.nodes[index].position
        } else {
            Position(curr.position.0 - 1, curr.position.1)
        };

        if previous.0 == next.position.0 || previous.1 == next.position.1 {
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
        assert_eq!(98484, result.unwrap())
    }

    #[test]
    fn test_small_b() {
        let result = get_shortest_path("input_small.txt");
        assert!(result.is_ok());
        assert_eq!(64, result.unwrap())
    }

    #[test]
    #[ignore = "reason"]
    fn test_input_b() {
        let result = get_shortest_path("input.txt");
        assert!(result.is_ok());
        assert_eq!(20351745, result.unwrap())
    }
}
