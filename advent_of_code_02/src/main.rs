use anyhow::{Context, Result};
use std::{fs::read_to_string, num::ParseIntError};

fn main() -> Result<()> {
    let reports = get_safe_reports("input.txt")?;
    println!("Safe reports: {reports}");

    let score = calculate_similarity_score("input.txt")?;
    println!("Similarity score: {score}");

    Ok(())
}

fn get_safe_reports(filename: &str) -> Result<usize> {
    let data = parse_file(filename)?;
    let safe_reports = data.iter().filter(|report| is_report_safe(report)).count();
    Ok(safe_reports)
}

fn calculate_similarity_score(filename: &str) -> Result<u32> {
    Ok(1)
}

fn parse_file(filename: &str) -> Result<Vec<Vec<u32>>> {
    let file = read_to_string(filename)?;
    let data = file
        .lines()
        .map(parse_line)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(data)
}

fn parse_line(line: &str) -> Result<Vec<u32>, ParseIntError> {
    line.split_whitespace()
        .map(|i| i.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()
}

fn is_report_safe(report: &[u32]) -> bool {
    let order = Order::get_order(report.first().unwrap(), report.get(1).unwrap());
    report
        .iter()
        .zip(report.iter().skip(1))
        .all(|(prev, curr)| order.is_in_order(prev, curr))
}

enum Order {
    Ascending,
    Descending,
}

impl Order {
    fn get_order(a: &u32, b: &u32) -> Order {
        if a > b {
            Order::Descending
        } else {
            Order::Ascending
        }
    }

    fn is_in_order(&self, a: &u32, b: &u32) -> bool {
        let (lower, higher) = match self {
            Order::Ascending => (a, b),
            Order::Descending => (b, a),
        };

        lower < higher && lower + 3 >= *higher
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_a() {
        let result = get_safe_reports("input_small.txt");
        assert!(result.is_ok());
        assert_eq!(2, result.unwrap())
    }

    #[test]
    fn test_input_a() {
        let result = get_safe_reports("input.txt");
        assert!(result.is_ok());
        //assert_eq!(1579939, result.unwrap())
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
