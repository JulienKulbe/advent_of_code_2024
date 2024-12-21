use anyhow::Result;
use std::{collections::BTreeMap, fs::read_to_string};

fn main() -> Result<()> {
    let designs = possible_designs("input.txt")?;
    println!("Possible designs: {designs}");

    let designs = count_possible_designs("input.txt")?;
    println!("Possible designs: {designs}");

    Ok(())
}

fn possible_designs(filename: &str) -> Result<usize> {
    let (towels, designs) = parse_file(filename)?;
    let mut memo = BTreeMap::new();

    let sum = designs
        .0
        .iter()
        .filter(|design| towels.can_create_design(design, &mut memo))
        .count();

    Ok(sum)
}

fn count_possible_designs(filename: &str) -> Result<usize> {
    let (towels, designs) = parse_file(filename)?;
    let mut memo = BTreeMap::new();

    let sum = designs
        .0
        .iter()
        .map(|design| towels.count_possible_designs(design, &mut memo))
        .sum();

    Ok(sum)
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
    fn can_create_design(&self, design: &str, memo: &mut BTreeMap<String, bool>) -> bool {
        if design.is_empty() {
            return true;
        }
        if let Some(result) = memo.get(design) {
            return *result;
        }

        for towel in &self.0 {
            if design.starts_with(towel) && self.can_create_design(&design[towel.len()..], memo) {
                memo.insert(design.to_string(), true);
                return true;
            }
        }

        memo.insert(design.to_string(), false);
        false
    }

    fn count_possible_designs(&self, design: &str, memo: &mut BTreeMap<String, usize>) -> usize {
        if design.is_empty() {
            return 1;
        }
        if let Some(result) = memo.get(design) {
            return *result;
        }

        let sum = self
            .0
            .iter()
            .filter(|&towel| design.starts_with(towel))
            .map(|towel| self.count_possible_designs(&design[towel.len()..], memo))
            .sum();

        memo.insert(design.to_string(), sum);
        sum
    }
}

struct Designs(Vec<String>);

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
        assert_eq!(324, result.unwrap())
    }

    #[test]
    fn test_small_b() {
        let result = count_possible_designs("input_small.txt");
        assert!(result.is_ok());
        assert_eq!(16, result.unwrap())
    }

    #[test]
    fn test_input_b() {
        let result = count_possible_designs("input.txt");
        assert!(result.is_ok());
        assert_eq!(575227823167869, result.unwrap())
    }
}
