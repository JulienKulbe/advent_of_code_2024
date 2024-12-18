use anyhow::Result;
use itertools::Itertools;
use std::fs::read_to_string;

fn main() -> Result<()> {
    let path = calculate_shortest_path("input.txt", 1024, 71)?;
    println!("Shortest path: {path}");

    let score = calculate_similarity_score("input.txt")?;
    println!("Similarity score: {score}");

    Ok(())
}

fn calculate_shortest_path(
    filename: &str,
    corrupted_cells: usize,
    dimension: usize,
) -> Result<usize> {
    let memory = parse_file(filename)?;
    let length = Graph::find_shortest_path_length(&memory.0[..corrupted_cells], dimension);

    Ok(length)
}

fn calculate_similarity_score(filename: &str) -> Result<u32> {
    Ok(1)
}

fn parse_file(filename: &str) -> Result<CorruptedMemory> {
    let data = read_to_string(filename)?;
    let positions = data
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(x, y)| Position::new(x, y))
        .collect();
    Ok(CorruptedMemory(positions))
}

struct CorruptedMemory(Vec<Position>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position(i32, i32);

impl Position {
    fn new(x: &str, y: &str) -> Position {
        Position(
            x.parse().expect("Invalid X coordinate"),
            y.parse().expect("Invalid Y coordinate"),
        )
    }
}

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

struct Node {
    position: Position,
    distance: usize,
    previous: Option<usize>,
    visited: bool,
}

impl Node {
    fn new(position: Position) -> Node {
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
    start: Position,
    end: Position,
}

impl Graph {
    fn find_shortest_path_length(memory: &[Position], dimension: usize) -> usize {
        let mut graph = Graph::new(memory, dimension);
        graph.calculate_dijkstra();
        graph.path_lenth()
    }

    fn new(memory: &[Position], dimension: usize) -> Graph {
        let nodes: Vec<Node> = (0..dimension)
            .cartesian_product(0..dimension)
            .map(|(x, y)| Position(x as i32, y as i32))
            .filter(|pos| !memory.contains(pos))
            .map(Node::new)
            .collect();

        Graph {
            nodes,
            start: Position(0, 0),
            end: Position(dimension as i32 - 1, dimension as i32 - 1),
        }
    }

    fn calculate_dijkstra(&mut self) {
        let start = self.index(self.start).unwrap();
        self.nodes[start].distance = 0;

        while let Some(u) = self.get_minimum_dist() {
            self.nodes[u].visited = true;

            for neighbor in self.nodes[u].position.get_neighbors() {
                if let Some(v) = self.index(neighbor) {
                    let alt = self.nodes[u].distance + 1;
                    if alt < self.nodes[v].distance {
                        self.nodes[v].distance = alt;
                        self.nodes[v].previous = Some(u);
                    }
                }
            }
        }
    }

    fn path_lenth(&self) -> usize {
        let mut length = 0;
        let mut current = self.index(self.end);

        while let Some(index) = current {
            length += 1;
            current = self.nodes[index].previous;
        }

        length - 1
    }

    fn index(&self, position: Position) -> Option<usize> {
        self.nodes.iter().position(|node| node.position == position)
    }

    fn get_minimum_dist(&self) -> Option<usize> {
        self.nodes
            .iter()
            .enumerate()
            .filter(|(_, node)| !node.visited)
            .min_by_key(|(_, node)| node.distance)
            .map(|(i, _)| i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_a() {
        let result = calculate_shortest_path("input_small.txt", 12, 7);
        assert!(result.is_ok());
        assert_eq!(22, result.unwrap())
    }

    #[test]
    fn test_input_a() {
        let result = calculate_shortest_path("input.txt", 1024, 71);
        assert!(result.is_ok());
        //assert_eq!(1579939, result.unwrap())
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
