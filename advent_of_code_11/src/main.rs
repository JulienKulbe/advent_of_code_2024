use anyhow::Result;
use std::{collections::BTreeMap, fs::read_to_string};

fn main() -> Result<()> {
    let stones = count_stones("input.txt")?;
    println!("#Stones: {stones}");

    let stones = count_stones_opt("input.txt")?;
    println!("Stones: {stones}");

    Ok(())
}

fn count_stones(filename: &str) -> Result<usize> {
    let mut stones = parse_file(filename)?;

    for _ in 0..25 {
        stones = apply_stone_behavior(stones);
    }

    Ok(stones.len())
}

fn count_stones_opt(filename: &str) -> Result<u64> {
    let mut stones = BTreeMap::new();
    for stone in parse_file(filename)? {
        stones.insert(stone, 1);
    }

    let mut memory = BTreeMap::new();

    for i in 0..75 {
        stones = apply_stone_behavior_opt(stones, &mut memory);
        println!("Iteration: {i}, Stones: {}", get_stone_count(&stones));
    }

    Ok(get_stone_count(&stones))
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

fn apply_stone_behavior_opt(
    current: BTreeMap<u64, u64>,
    memory: &mut BTreeMap<u64, Vec<u64>>,
) -> BTreeMap<u64, u64> {
    let mut result: BTreeMap<u64, u64> = BTreeMap::new();
    for (stone, n) in current.iter() {
        let stones = if let Some(stones) = memory.get(stone) {
            stones.clone()
        } else {
            let stones = apply_stone_behavior(vec![*stone]);
            memory.insert(*stone, stones.clone());
            stones
        };

        for stone in stones.iter() {
            if let Some(count) = result.get_mut(stone) {
                *count += n;
            } else {
                result.insert(*stone, *n);
            }
        }
    }

    result
}

fn get_stone_count(stones: &BTreeMap<u64, u64>) -> u64 {
    stones.iter().map(|(_, c)| *c).sum()
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
    fn test_input_b() {
        let result = count_stones_opt("input.txt");
        assert!(result.is_ok());
        assert_eq!(234430066982597, result.unwrap())
    }
}
