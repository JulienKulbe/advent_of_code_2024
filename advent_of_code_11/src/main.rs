use anyhow::Result;
use std::fs::read_to_string;

fn main() -> Result<()> {
    let stones = count_stones("input.txt")?;
    println!("#Stones: {stones}");

    let score = calculate_similarity_score("input.txt")?;
    println!("Similarity score: {score}");

    Ok(())
}

fn count_stones(filename: &str) -> Result<usize> {
    let mut stones = parse_file(filename)?;

    for _ in 0..25 {
        stones = apply_stone_behavior(stones);
    }

    Ok(stones.len())
}

fn calculate_similarity_score(filename: &str) -> Result<u32> {
    Ok(1)
}

fn parse_file(filename: &str) -> Result<Vec<u64>> {
    let data = read_to_string(filename)?;
    let stones = data
        .split_whitespace()
        .map(|v| v.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?;
    Ok(stones)
}

fn apply_stone_behavior(current: Vec<u64>) -> Vec<u64> {
    let mut result = Vec::new();
    for stone in current {
        let mut next = match stone {
            0 => vec![1],
            x if has_even_digits(x) => slit_stone(x),
            y => vec![y * 2024],
        };
        result.append(&mut next);
    }

    result
}

fn has_even_digits(value: u64) -> bool {
    let digits = value.checked_ilog10().unwrap_or(0) + 1;
    digits % 2 == 0
}

fn slit_stone(value: u64) -> Vec<u64> {
    let digits = value.checked_ilog10().unwrap_or(0) + 1;
    let divider = 10_u64.pow(digits / 2);
    let left = value / divider;
    let right = value % divider;

    vec![left, right]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_a() {
        let result = count_stones("input_small.txt");
        assert!(result.is_ok());
        assert_eq!(55312, result.unwrap())
    }

    #[test]
    fn test_input_a() {
        let result = count_stones("input.txt");
        assert!(result.is_ok());
        assert_eq!(197157, result.unwrap())
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
