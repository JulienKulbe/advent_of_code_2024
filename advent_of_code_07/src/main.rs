use anyhow::{Context, Result};
use std::fs::read_to_string;

fn main() -> Result<()> {
    let sum = get_calibration_results("input.txt")?;
    println!("Calibration results: {sum}");

    let score = calculate_similarity_score("input.txt")?;
    println!("Similarity score: {score}");

    Ok(())
}

fn get_calibration_results(filename: &str) -> Result<u64> {
    let equations = parse_file(filename)?;
    let sum = equations
        .iter()
        .filter(|&equation| is_valid_equation(equation))
        .map(|equation| equation.test_value)
        .sum();

    Ok(sum)
}

fn calculate_similarity_score(filename: &str) -> Result<u32> {
    Ok(1)
}

fn parse_file(filename: &str) -> Result<Vec<Equation>> {
    let data = read_to_string(filename)?;
    let equations = data
        .lines()
        .map(parse_line)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(equations)
}

fn parse_line(line: &str) -> Result<Equation> {
    let (test_value, values) = line.split_once(':').context("Invalid Equation")?;
    let values = values
        .split_whitespace()
        .map(|v| v.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?;
    let test_value = test_value.parse::<u64>()?;
    Ok(Equation { test_value, values })
}

fn is_valid_equation(equation: &Equation) -> bool {
    if equation.values.len() == 1 {
        equation.test_value == equation.values[0]
    } else if equation.test_value < equation.values[0] {
        false
    } else {
        let mut equation_mult = equation.clone();
        let mut equation_add = equation.clone();

        let mult_result = equation_mult.values.remove(1) * equation_mult.values.remove(0);
        let add_result = equation_add.values.remove(1) + equation_add.values.remove(0);

        equation_mult.values.insert(0, mult_result);
        equation_add.values.insert(0, add_result);

        is_valid_equation(&equation_mult) || is_valid_equation(&equation_add)
    }
}

#[derive(Debug, Clone)]
struct Equation {
    test_value: u64,
    values: Vec<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_a() {
        let result = get_calibration_results("input_small.txt");
        assert!(result.is_ok());
        assert_eq!(3749, result.unwrap())
    }

    #[test]
    fn test_input_a() {
        let result = get_calibration_results("input.txt");
        assert!(result.is_ok());
        assert_eq!(4998764814652, result.unwrap())
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
