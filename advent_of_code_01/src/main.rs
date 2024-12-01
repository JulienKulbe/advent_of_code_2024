use anyhow::{Context, Result};
use std::fs::read_to_string;

fn parse_line(line: &str) -> Result<(u32, u32)> {
    let (left, right) = line
        .split_once(char::is_whitespace)
        .context("expected two values for each line")?;
    let left = left.trim().parse::<u32>()?;
    let right = right.trim().parse::<u32>()?;
    Ok((left, right))
}

fn parse_file(filename: &str) -> Result<(Vec<u32>, Vec<u32>)> {
    let data = read_to_string(filename)?;
    Ok(data
        .lines()
        .map(parse_line)
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .cloned()
        .unzip())
}

fn calculate_total_distance(filename: &str) -> Result<u32> {
    let (mut left, mut right) = parse_file(filename)?;

    left.sort_unstable();
    right.sort_unstable();

    Ok(left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum())
}

fn calculate_similarity_score(filename: &str) -> Result<u32> {
    let (left, right) = parse_file(filename)?;

    Ok(left
        .iter()
        .map(|left| {
            let right = right.iter().filter(|r| *r == left).count() as u32;
            left * right
        })
        .sum())
}

fn main() -> Result<()> {
    let distance = calculate_total_distance("input.txt")?;
    println!("Total distance: {}", distance);

    let score = calculate_similarity_score("input.txt")?;
    println!("Similarity score: {}", score);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_a() {
        let result = calculate_total_distance("input_small.txt");
        assert!(result.is_ok());
        assert_eq!(11, result.unwrap())
    }

    #[test]
    fn test_input_a() {
        let result = calculate_total_distance("input.txt");
        assert!(result.is_ok());
        assert_eq!(1579939, result.unwrap())
    }

    #[test]
    fn test_small_b() {
        let result = calculate_similarity_score("input_small.txt");
        assert!(result.is_ok());
        assert_eq!(31, result.unwrap())
    }

    #[test]
    fn test_input_b() {
        let result = calculate_similarity_score("input.txt");
        assert!(result.is_ok());
        assert_eq!(20351745, result.unwrap())
    }
}
