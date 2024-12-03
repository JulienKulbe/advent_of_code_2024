use anyhow::Result;
use regex::Regex;
use std::fs::read_to_string;

fn main() -> Result<()> {
    let sum = calculates_sum("input.txt")?;
    println!("Sum: {sum}");

    let sum = calculate_enabled_sum("input.txt")?;
    println!("Enabled sum: {sum}");

    Ok(())
}

fn calculates_sum(filename: &str) -> Result<u32> {
    let haystack = parse_file(filename)?;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;

    let sum: u32 = re
        .captures_iter(&haystack)
        .map(|c| c.extract())
        .map(|(_, [mult1, mult2])| multiply(mult1, mult2))
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum();

    Ok(sum)
}

fn calculate_enabled_sum(filename: &str) -> Result<u32> {
    let haystack = parse_file(filename)?;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;

    let sum: u32 = re
        .captures_iter(&haystack)
        .map(|c| (c.get(1).unwrap().start(), c.extract()))
        .filter_map(|(index, (_, [mult1, mult2]))| {
            let last_do = haystack[0..index].rfind("do()");
            let last_dont = haystack[0..index].rfind("don't()");
            if should_add(last_dont, last_do) {
                Some(multiply(mult1, mult2))
            } else {
                None
            }
        })
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum();

    Ok(sum)
}

fn multiply(a: &str, b: &str) -> Result<u32> {
    let a = a.parse::<u32>()?;
    let b = b.parse::<u32>()?;
    Ok(a * b)
}

fn should_add(last_dont: Option<usize>, last_do: Option<usize>) -> bool {
    if let Some(last_dont) = last_dont {
        if let Some(last_do) = last_do {
            last_dont < last_do // check if last don't() is befor the last do()
        } else {
            false // if there is a don't() but no do() then multiplication is disabled
        }
    } else {
        true // if there was no don't() then multiplication is enabled
    }
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
    fn test_small_b() {
        let result = calculate_enabled_sum("input_small_b.txt");
        assert!(result.is_ok());
        assert_eq!(48, result.unwrap())
    }

    #[test]
    fn test_input_b() {
        let result = calculate_enabled_sum("input.txt");
        assert!(result.is_ok());
        assert_eq!(104083373, result.unwrap())
    }
}
