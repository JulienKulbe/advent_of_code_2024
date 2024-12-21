use anyhow::Result;
use std::{collections::BTreeMap, fs::read_to_string};

fn main() -> Result<()> {
    let designs = possible_designs("input.txt")?;
    println!("Possible designs: {designs}");

    let score = calculate_similarity_score("input.txt")?;
    println!("Similarity score: {score}");

    Ok(())
}

fn possible_designs(filename: &str) -> Result<usize> {
    let (towels, designs) = parse_file(filename)?;
    let mut memo = Memo(BTreeMap::new());

    let sum = designs
        .0
        .iter()
        .filter(|design| towels.can_create_design(design, &mut memo))
        .count();

    Ok(sum)
}

fn calculate_similarity_score(filename: &str) -> Result<u32> {
    Ok(1)
}

fn parse_file(filename: &str) -> Result<(Towels, Designs)> {
    let data = read_to_string(filename)?;
    let mut lines: Vec<String> = data.lines().map(|line| line.to_string()).collect();

    let towels = lines[0]
        .split(',')
        .map(|towel| towel.trim().to_string())
        .collect();

    lines.remove(1);
    lines.remove(0);

    Ok((Towels(towels), Designs(lines)))
}

struct Towels(Vec<String>);

impl Towels {
    fn can_create_design(&self, design: &str, memo: &mut Memo) -> bool {
        if design.is_empty() {
            return true;
        }
        if let Some(result) = memo.0.get(design) {
            return *result;
        }

        for towel in &self.0 {
            if design.starts_with(towel) && self.can_create_design(&design[towel.len()..], memo) {
                memo.0.insert(design.to_string(), true);
                return true;
            }
        }

        memo.0.insert(design.to_string(), false);
        false
    }
}

struct Designs(Vec<String>);

struct Memo(BTreeMap<String, bool>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_a() {
        let result = possible_designs("input_small.txt");
        assert!(result.is_ok());
        assert_eq!(6, result.unwrap())
    }

    #[test]
    fn test_input_a() {
        let result = possible_designs("input.txt");
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
