use anyhow::Result;
use regex::Regex;
use std::fs::read_to_string;

fn main() -> Result<()> {
    let tokens = calculate_tokens("input.txt")?;
    println!("Minimum tokens: {tokens}");

    let score = calculate_similarity_score("input.txt")?;
    println!("Similarity score: {score}");

    Ok(())
}

fn calculate_tokens(filename: &str) -> Result<i64> {
    let claw_machines = parse_file(filename)?;
    let tokens = claw_machines
        .iter()
        .filter_map(|cm| cm.required_tokens())
        .sum();

    Ok(tokens)
}

fn calculate_similarity_score(filename: &str) -> Result<u32> {
    Ok(1)
}

fn parse_file(filename: &str) -> Result<Vec<ClawMachine>> {
    let data = read_to_string(filename)?;

    let re = Regex::new(
        r"Button A: X\+(\d*), Y\+(\d*)\s*Button B: X\+(\d*), Y\+(\d*)\s*Prize: X=(\d*), Y=(\d*)",
    )?;
    let claw_machines = re
        .captures_iter(&data)
        .map(|c| c.extract())
        .map(|(_, [ax, ay, bx, by, px, py])| ClawMachine {
            a: Position(ax.parse().unwrap(), ay.parse().unwrap()),
            b: Position(bx.parse().unwrap(), by.parse().unwrap()),
            prize: Position(px.parse().unwrap(), py.parse().unwrap()),
        })
        .collect();

    Ok(claw_machines)
}

#[derive(Debug, Clone, Copy)]
struct Position(i64, i64);

struct ClawMachine {
    a: Position,
    b: Position,
    prize: Position,
}

impl ClawMachine {
    fn required_tokens(&self) -> Option<i64> {
        let b = (self.prize.1 * self.a.0 - self.prize.0 * self.a.1)
            / (self.a.0 * self.b.1 - self.b.0 * self.a.1);
        let a = (self.prize.0 - b * self.b.0) / (self.a.0);

        if a * self.a.0 + b * self.b.0 == self.prize.0
            && a * self.a.1 + b * self.b.1 == self.prize.1
        {
            Some(a * 3 + b)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_a() {
        let result = calculate_tokens("input_small.txt");
        assert!(result.is_ok());
        assert_eq!(480, result.unwrap())
    }

    #[test]
    fn test_input_a() {
        let result = calculate_tokens("input.txt");
        assert!(result.is_ok());
        assert_eq!(35255, result.unwrap())
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
