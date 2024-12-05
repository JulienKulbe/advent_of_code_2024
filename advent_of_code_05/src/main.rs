use anyhow::{Context, Result};
use std::fs::read_to_string;

fn main() -> Result<()> {
    let sum = middle_page_sum("input.txt")?;
    println!("Sum: {sum}");

    let score = calculate_similarity_score("input.txt")?;
    println!("Similarity score: {score}");

    Ok(())
}

fn middle_page_sum(filename: &str) -> Result<u32> {
    let input = parse_file(filename)?;
    Ok(input
        .pages
        .iter()
        .filter(|page| is_page_valid(page, &input.rules))
        .map(|page| page.0.get(page.0.len() / 2).unwrap())
        .sum())
}

fn calculate_similarity_score(filename: &str) -> Result<u32> {
    Ok(1)
}

fn parse_file(filename: &str) -> Result<InputData> {
    let data = read_to_string(filename)?;
    let mut found_blank_line = false;

    let mut rules = Vec::new();
    let mut pages = Vec::new();

    for line in data.lines() {
        if line.is_empty() {
            found_blank_line = true;
        } else if !found_blank_line {
            let (pre, succ) = line.split_once('|').context("Rule must contain a '|'")?;
            let rule = OrderingRule {
                predecessor: pre.parse()?,
                successor: succ.parse()?,
            };
            rules.push(rule);
        } else {
            let page = line
                .split(',')
                .map(|s| s.parse::<u32>())
                .collect::<Result<Vec<_>, _>>()?;
            pages.push(Page(page));
        }
    }

    Ok(InputData { rules, pages })
}

fn is_page_valid(page: &Page, rules: &[OrderingRule]) -> bool {
    page.0
        .iter()
        .enumerate()
        .all(|(i, p)| page.0[0..i].iter().all(|c| is_valid(*c, *p, rules)))
}

fn is_valid(from: u32, to: u32, rules: &[OrderingRule]) -> bool {
    rules
        .iter()
        .all(|rule| rule.predecessor != to || rule.successor != from)
}

struct InputData {
    rules: Vec<OrderingRule>,
    pages: Vec<Page>,
}

struct Page(Vec<u32>);

struct OrderingRule {
    predecessor: u32,
    successor: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_a() {
        let result = middle_page_sum("input_small.txt");
        assert!(result.is_ok());
        assert_eq!(143, result.unwrap())
    }

    #[test]
    fn test_input_a() {
        let result = middle_page_sum("input.txt");
        assert!(result.is_ok());
        assert_eq!(4185, result.unwrap())
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
