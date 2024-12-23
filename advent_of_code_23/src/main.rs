use anyhow::Result;
use itertools::Itertools;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::read_to_string,
};

fn main() -> Result<()> {
    let sets = find_connected_sets("input.txt")?;
    println!("Founds sets: {sets}");

    let score = calculate_similarity_score("input.txt")?;
    println!("Similarity score: {score}");

    Ok(())
}

fn find_connected_sets(filename: &str) -> Result<usize> {
    let connections = parse_file(filename)?;
    let t_connections = get_connections_with_t(&connections);
    let triplets = get_connections_with_two_sinks(&connections, &t_connections);

    Ok(triplets.len())
}

fn calculate_similarity_score(filename: &str) -> Result<u32> {
    Ok(1)
}

fn parse_file(filename: &str) -> Result<Vec<(String, String)>> {
    let data = read_to_string(filename)?;
    let lines = data
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .map(|(a, b)| (a.to_string(), b.to_string()))
        .collect();
    Ok(lines)
}

fn get_connections_with_t(connections: &[(String, String)]) -> BTreeMap<String, Vec<String>> {
    let mut map = BTreeMap::new();
    for (client_a, client_b) in connections {
        add_connection_with_t(client_a, client_b, &mut map);
        add_connection_with_t(client_b, client_a, &mut map);
    }
    map
}

fn add_connection_with_t(source: &String, sink: &String, map: &mut BTreeMap<String, Vec<String>>) {
    if source.starts_with('t') {
        if let Some(sinks) = map.get_mut(source) {
            sinks.push(sink.clone());
        } else {
            let sinks = vec![sink.clone()];
            map.insert(source.clone(), sinks);
        }
    }
}

fn get_connections_with_two_sinks(
    c: &[(String, String)],
    t: &BTreeMap<String, Vec<String>>,
) -> BTreeSet<[String; 3]> {
    let mut sets = BTreeSet::new();

    for (key, values) in t {
        if values.len() > 1 {
            for sinks in values.iter().combinations(2) {
                if contains_connection_between(c, sinks[0], sinks[1]) {
                    let mut set = [key.clone(), sinks[0].clone(), sinks[1].clone()];
                    set.sort_unstable();
                    sets.insert(set);
                }
            }
        }
    }

    sets
}

fn contains_connection_between(c: &[(String, String)], source: &str, sink: &str) -> bool {
    c.iter()
        .any(|(a, b)| (a == source && b == sink) || (a == sink && b == source))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_a() {
        let result = find_connected_sets("input_small.txt");
        assert!(result.is_ok());
        assert_eq!(7, result.unwrap())
    }

    #[test]
    fn test_input_a() {
        let result = find_connected_sets("input.txt");
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
