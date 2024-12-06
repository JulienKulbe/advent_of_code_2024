use anyhow::{Context, Result};
use std::fs::read_to_string;

fn main() -> Result<()> {
    let sum = middle_page_sum("input.txt")?;
    println!("Sum: {sum}");

    let sum = fix_unordered_pages("input.txt")?;
    println!("Sum: {sum}");

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

fn fix_unordered_pages(filename: &str) -> Result<u32> {
    let input = parse_file(filename)?;

    Ok(input
        .pages
        .iter()
        .filter_map(|page| {
            if is_page_valid(page, &input.rules) {
                None
            } else {
                Some(order_page(page, &input.rules))
            }
        })
        .map(|page| page.0.get(page.0.len() / 2).unwrap().clone())
        .sum())
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

fn order_page(page: &Page, rules: &[OrderingRule]) -> Page {
    let mut ordered_page = page.clone();
    while !is_page_valid(&ordered_page, rules) {
        'outer: for (i, p) in ordered_page.0.iter().enumerate() {
            for (j, c) in ordered_page.0[0..i].iter().enumerate() {
                if !is_valid(*c, *p, rules) {
                    ordered_page.0.swap(i, j);
                    break 'outer;
                }
            }
        }
    }

    ordered_page.clone()
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

#[derive(Debug, Clone)]
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
    fn test_small_b() {
        let result = fix_unordered_pages("input_small.txt");
        assert!(result.is_ok());
        assert_eq!(123, result.unwrap())
    }

    #[test]
    fn test_input_b() {
        let result = fix_unordered_pages("input.txt");
        assert!(result.is_ok());
        assert_eq!(4480, result.unwrap())
    }
}
