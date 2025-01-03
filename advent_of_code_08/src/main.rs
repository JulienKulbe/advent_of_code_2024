use anyhow::Result;
use itertools::Itertools;
use std::{collections::BTreeMap, fs::read_to_string};

fn main() -> Result<()> {
    let anodes = count_anodes_of_antennas("input.txt")?;
    println!("Total Anodes: {anodes}");

    let anodes = count_anodes_of_resonance_antennas("input.txt")?;
    println!("Total Anodes: {anodes}");

    Ok(())
}

fn count_anodes_of_antennas(filename: &str) -> Result<usize> {
    let map = parse_file(filename)?;
    let antennas = Antennas::new(&map);

    let mut anodes: Vec<Position> = Vec::new();
    for (_, positions) in antennas.0 {
        let mut current: Vec<Position> = positions
            .iter()
            .permutations(2)
            .flat_map(|positions| calculate_anodes(*positions[0], *positions[1]))
            .filter(|anode| map.is_valid(*anode))
            .collect();

        anodes.append(&mut current);
    }

    let count = anodes.iter().unique().count();

    Ok(count)
}

fn calculate_anodes(a: Position, b: Position) -> [Position; 2] {
    let first = Position(2 * a.0 - b.0, 2 * a.1 - b.1);
    let second = Position(2 * b.0 - a.0, 2 * b.1 - a.1);
    [first, second]
}

fn count_anodes_of_resonance_antennas(filename: &str) -> Result<usize> {
    let map = parse_file(filename)?;
    let antennas = Antennas::new(&map);

    let mut anodes: Vec<Position> = Vec::new();
    for (_, positions) in antennas.0 {
        let mut current: Vec<Position> = positions
            .iter()
            .permutations(2)
            .flat_map(|positions| calculate_resonance_anodes(&map, *positions[0], *positions[1]))
            .collect();

        anodes.append(&mut current);
    }

    let count = anodes.iter().unique().count();

    Ok(count)
}

fn calculate_resonance_anodes(map: &Map, a: Position, b: Position) -> Vec<Position> {
    let mut anodes = Vec::new();
    anodes.append(&mut calculate_resonance_anodes_unidirectinal(map, a, b));
    anodes.append(&mut calculate_resonance_anodes_unidirectinal(map, b, a));
    anodes
}

fn calculate_resonance_anodes_unidirectinal(
    map: &Map,
    start: Position,
    end: Position,
) -> Vec<Position> {
    let mut anodes = Vec::new();
    let diff = Position(start.0 - end.0, start.1 - end.1);
    for i in 1.. {
        let anode = Position(start.0 - i * diff.0, start.1 - i * diff.1);
        if map.is_valid(anode) {
            anodes.push(anode);
        } else {
            break;
        }
    }
    anodes
}

fn parse_file(filename: &str) -> Result<Map> {
    let file = read_to_string(filename)?;
    let data: Vec<Vec<char>> = file.lines().map(|line| line.chars().collect()).collect();
    let height = data.len();
    let width = data[0].len();

    Ok(Map {
        data,
        width,
        height,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(i32, i32);

struct Map {
    data: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Map {
    fn is_valid(&self, pos: Position) -> bool {
        pos.0 >= 0 && pos.0 < self.width as i32 && pos.1 >= 0 && pos.1 < self.height as i32
    }
}

struct Antennas(BTreeMap<char, Vec<Position>>);

impl Antennas {
    fn new(map: &Map) -> Self {
        let mut antennas: BTreeMap<char, Vec<Position>> = BTreeMap::new();

        for y in 0..map.height {
            for x in 0..map.width {
                let c = map.data[y][x];
                if c != '.' {
                    let position = Position(x as i32, y as i32);
                    let values = antennas.get_mut(&c);

                    if let Some(values) = values {
                        values.push(position);
                    } else {
                        antennas.insert(c, vec![position]);
                    }
                }
            }
        }

        Self(antennas)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_a() {
        let result = count_anodes_of_antennas("input_small.txt");
        assert!(result.is_ok());
        assert_eq!(14, result.unwrap())
    }

    #[test]
    fn test_input_a() {
        let result = count_anodes_of_antennas("input.txt");
        assert!(result.is_ok());
        assert_eq!(280, result.unwrap())
    }

    #[test]
    fn test_small_b() {
        let result = count_anodes_of_resonance_antennas("input_small.txt");
        assert!(result.is_ok());
        assert_eq!(34, result.unwrap())
    }

    #[test]
    fn test_input_b() {
        let result = count_anodes_of_resonance_antennas("input.txt");
        assert!(result.is_ok());
        assert_eq!(958, result.unwrap())
    }
}
