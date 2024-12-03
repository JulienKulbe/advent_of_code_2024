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

    let mut sum = 0;
    for (_, [mult1, mult2]) in re.captures_iter(&haystack).map(|c| c.extract()) {
        sum += mult1.parse::<u32>()? * mult2.parse::<u32>()?;
    }

    Ok(sum)
}

fn calculate_enabled_sum(filename: &str) -> Result<u32> {
    let haystack = parse_file(filename)?;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;

    let mut mults = Vec::new();
    for (start, (_, [mult1, mult2])) in re
        .captures_iter(&haystack)
        .map(|c| (c.get(1).unwrap().start(), c.extract()))
    {
        mults.push((start, mult1.parse::<u32>()? * mult2.parse::<u32>()?));
    }

    let mut sum = 0;
    for (start, mult) in mults {
        // find position of last do() and don't() starting from the regex start position
        let last_do = haystack[0..start].rfind("do()");
        let last_dont = haystack[0..start].rfind("don't()");

        // set mult to None if don't() was after the last do()
        let mut mult = Some(mult);
        if let Some(last_dont) = last_dont {
            if let Some(last_do) = last_do {
                if last_dont > last_do {
                    mult = None
                }
            } else {
                mult = None;
            }
        }

        if let Some(mult) = mult {
            sum += mult;
        }
    }

    Ok(sum)
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
