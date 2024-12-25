use anyhow::Result;
use itertools::Itertools;
use std::fs::read_to_string;

fn main() -> Result<()> {
    let fits = get_lock_key_pairs("input.txt")?;
    println!("Fits: {fits}");

    let score = calculate_similarity_score("input.txt")?;
    println!("Similarity score: {score}");

    Ok(())
}

fn get_lock_key_pairs(filename: &str) -> Result<usize> {
    let elements = parse_file(filename)?;

    let mut fits = 0;
    for key in &elements.keys {
        for lock in &elements.locks {
            if fit_key_lock(key, lock) {
                fits += 1;
            }
        }
    }

    Ok(fits)
}

fn calculate_similarity_score(filename: &str) -> Result<u32> {
    Ok(1)
}

fn parse_file(filename: &str) -> Result<Elements> {
    let data = read_to_string(filename)?;
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for lines in &data.lines().chunks(8) {
        match parse_element(lines.collect()) {
            Element::Lock(lock) => {
                locks.push(lock);
            }
            Element::Key(key) => {
                keys.push(key);
            }
        }
    }

    Ok(Elements { locks, keys })
}

fn parse_element(lines: Vec<&str>) -> Element {
    if lines[0] == "#####" {
        Element::Lock(Lock(parse_data(lines)))
    } else if lines[6] == "#####" {
        Element::Key(Key(parse_data(lines)))
    } else {
        panic!("Invalid Element found")
    }
}

fn parse_data(lines: Vec<&str>) -> [u8; 5] {
    let mut data = [0, 0, 0, 0, 0];
    for line in lines.iter().skip(1).take(5) {
        for (i, c) in line.char_indices() {
            if c == '#' {
                data[i] += 1;
            }
        }
    }
    data
}

fn fit_key_lock(key: &Key, lock: &Lock) -> bool {
    for i in 0..5 {
        if key.0[i] + lock.0[i] > 5 {
            return false;
        }
    }
    true
}

enum Element {
    Lock(Lock),
    Key(Key),
}

struct Elements {
    locks: Vec<Lock>,
    keys: Vec<Key>,
}

struct Lock([u8; 5]);

struct Key([u8; 5]);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_a() {
        let result = get_lock_key_pairs("input_small.txt");
        assert!(result.is_ok());
        assert_eq!(3, result.unwrap())
    }

    #[test]
    fn test_input_a() {
        let result = get_lock_key_pairs("input.txt");
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
