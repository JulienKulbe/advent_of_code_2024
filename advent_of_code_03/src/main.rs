use anyhow::{Context, Result};
use regex::Regex;
use std::fs::read_to_string;

fn main() -> Result<()> {
    let sum = calculates_sum("input.txt")?;
    println!("Sum: {sum}");

    let score = calculate_similarity_score("input.txt")?;
    println!("Similarity score: {score}");

    Ok(())
}

fn calculates_sum(filename: &str) -> Result<u32> {
    let haystack = parse_file(filename)?;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;

    let mut sum = 0;
    for (_, [mult1, mult2]) in re.captures_iter(&haystack).map(|c| c.extract()) {
        sum += mult1.parse::<u32>()? * mult2.parse::<u32>()?;
    }

    Ok(sum)
}

fn calculate_similarity_score(filename: &str) -> Result<u32> {
    Ok(1)
}

fn parse_file(filename: &str) -> Result<String> {
    let data = read_to_string(filename)?;
    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_a() {
        let result = calculates_sum("input_small.txt");
        assert!(result.is_ok());
        assert_eq!(161, result.unwrap())
    }

    #[test]
    fn test_input_a() {
        let result = calculates_sum("input.txt");
        assert!(result.is_ok());
        assert_eq!(192767529, result.unwrap())
    }

    #[test]
    #[ignore]
    fn test_small_b() {
        let result = calculate_similarity_score("input_small.txt");
        assert!(result.is_ok());
        assert_eq!(31, result.unwrap())
    }

    #[test]
    #[ignore]
    fn test_input_b() {
        let result = calculate_similarity_score("input.txt");
        assert!(result.is_ok());
        assert_eq!(20351745, result.unwrap())
    }
}
