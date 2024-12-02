use anyhow::Result;
use std::{fs::read_to_string, num::ParseIntError};

fn main() -> Result<()> {
    let reports = get_safe_reports("input.txt")?;
    println!("Safe reports: {reports}");

    let reports2 = get_safe_reports_with_dampener("input.txt")?;
    println!("Similarity score: {reports2}");

    Ok(())
}

fn get_safe_reports(filename: &str) -> Result<usize> {
    let data = parse_file(filename)?;
    let safe_reports = data.iter().filter(|report| is_report_safe(report)).count();
    Ok(safe_reports)
}

fn get_safe_reports_with_dampener(filename: &str) -> Result<usize> {
    let data = parse_file(filename)?;
    let safe_reports = data
        .iter()
        .filter(|report| is_report_safe_with_dampener(report))
        .count();
    Ok(safe_reports)
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

fn is_report_safe_with_dampener(report: &[u32]) -> bool {
    if is_report_safe(report) {
        return true;
    }

    for index in 0..report.len() {
        // create a new report without the n-th level
        let new_report = report
            .iter()
            .enumerate()
            .filter_map(|(i, e)| if i != index { Some(e) } else { None })
            .cloned()
            .collect::<Vec<_>>();
        if is_report_safe(&new_report) {
            return true;
        }
    }

    false
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
        assert_eq!(299, result.unwrap())
    }

    #[test]
    fn test_small_b() {
        let result = get_safe_reports_with_dampener("input_small.txt");
        assert!(result.is_ok());
        assert_eq!(4, result.unwrap())
    }

    #[test]
    fn test_input_b() {
        let result = get_safe_reports_with_dampener("input.txt");
        assert!(result.is_ok());
        assert_eq!(364, result.unwrap())
    }
}
