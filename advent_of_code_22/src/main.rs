use anyhow::Result;
use itertools::Itertools;
use std::{collections::HashSet, fs::read_to_string};

fn main() -> Result<()> {
    let sum = sum_secret_numbers("input.txt")?;
    println!("Sum: {sum}");

    let max = get_max_price("input.txt")?;
    println!("Max Price: {max}");

    Ok(())
}

fn sum_secret_numbers(filename: &str) -> Result<u64> {
    let seeds = parse_file(filename)?;
    let sum = seeds
        .iter()
        .map(|seed| get_secret_number(*seed, 2000).0)
        .sum();
    Ok(sum)
}

fn get_max_price(filename: &str) -> Result<u64> {
    let seeds = parse_file(filename)?;
    let secrets: Vec<Vec<Secret>> = seeds
        .iter()
        .map(|seed| get_secret_numbers(*seed, 2000))
        .collect();

    let prices: Vec<Vec<Price>> = secrets.iter().map(|secrets| get_prices(secrets)).collect();
    let diffs: HashSet<[i64; 4]> = prices
        .iter()
        .flat_map(|prices| get_diff_prices(prices, 10))
        .collect();

    let max = diffs
        .iter()
        .map(|diff| {
            prices
                .iter()
                .filter_map(|prices| get_price_for_diff(prices, diff))
                .sum::<u64>()
        })
        .max()
        .unwrap();

    Ok(max)
}

fn parse_file(filename: &str) -> Result<Vec<Secret>> {
    let data = read_to_string(filename)?;
    let lines = data.lines().map(Secret::new).collect();
    Ok(lines)
}

fn get_secret_number(start: Secret, iterations: usize) -> Secret {
    let mut current = start;
    for _ in 0..iterations {
        current = current.next();
    }
    current
}

fn get_secret_numbers(start: Secret, iterations: usize) -> Vec<Secret> {
    let mut current = start;
    let mut secrets = vec![current];
    for _ in 0..iterations {
        current = current.next();
        secrets.push(current);
    }
    secrets
}

fn get_prices(secrets: &[Secret]) -> Vec<Price> {
    secrets
        .iter()
        .map(|secret| secret.price())
        .tuple_windows()
        .map(|(prev, curr)| Price::new(prev, curr))
        .collect()
}

fn get_diff_prices(prices: &[Price], count: usize) -> Vec<[i64; 4]> {
    prices
        .iter()
        .tuple_windows()
        .map(|(a, b, c, d)| ([a.diff, b.diff, c.diff, d.diff], d.total))
        .sorted_by(|a, b| Ord::cmp(&b.1, &a.1))
        .take(count)
        .map(|(diff, _)| diff)
        .collect()
}

fn get_price_for_diff(prices: &[Price], diff: &[i64; 4]) -> Option<u64> {
    prices
        .iter()
        .tuple_windows()
        .find(|(a, b, c, d)| [a.diff, b.diff, c.diff, d.diff] == *diff)
        .map(|(_, _, _, d)| d.total)
}

#[derive(Debug, Clone, Copy)]
struct Secret(u64);

impl Secret {
    fn new(value: &str) -> Self {
        let value = value.parse().expect("Not an integer value");
        Secret(value)
    }

    fn next(&self) -> Self {
        self.step(|value| value * 64)
            .step(|value| value / 32)
            .step(|value| value * 2048)
    }

    fn price(&self) -> u64 {
        self.0 % 10
    }

    fn step(&self, op: impl Fn(u64) -> u64) -> Self {
        let mix = op(self.0);
        self.mix(mix).prune()
    }

    fn mix(&self, value: u64) -> Self {
        let next = self.0 ^ value;
        Secret(next)
    }

    fn prune(&self) -> Self {
        let next = self.0 % 16777216;
        Secret(next)
    }
}

struct Price {
    total: u64,
    diff: i64,
}

impl Price {
    fn new(prev: u64, current: u64) -> Self {
        Price {
            total: current,
            diff: current as i64 - prev as i64,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_secrets() {
        let start = Secret(123);

        let next = start.next();
        assert_eq!(15887950, next.0);

        let next = next.next();
        assert_eq!(16495136, next.0);

        let next = next.next();
        assert_eq!(527345, next.0);

        let next = next.next();
        assert_eq!(704524, next.0);

        let next = next.next();
        assert_eq!(1553684, next.0);

        let next = next.next();
        assert_eq!(12683156, next.0);

        let next = next.next();
        assert_eq!(11100544, next.0);

        let next = next.next();
        assert_eq!(12249484, next.0);

        let next = next.next();
        assert_eq!(7753432, next.0);

        let next = next.next();
        assert_eq!(5908254, next.0);
    }

    #[test]
    fn test_small_a() {
        let result = sum_secret_numbers("input_small.txt");
        assert!(result.is_ok());
        assert_eq!(37327623, result.unwrap())
    }

    #[test]
    fn test_input_a() {
        let result = sum_secret_numbers("input.txt");
        assert!(result.is_ok());
        assert_eq!(14082561342, result.unwrap())
    }

    #[test]
    fn test_small_b() {
        let seeds = [Secret(1), Secret(2), Secret(3), Secret(2024)];
        let secrets: Vec<Vec<Secret>> = seeds
            .iter()
            .map(|seed| get_secret_numbers(*seed, 2000))
            .collect();

        let prices: Vec<Vec<Price>> = secrets.iter().map(|secrets| get_prices(secrets)).collect();

        let diff: [i64; 4] = [-2, 1, -1, 3];
        let max = prices
            .iter()
            .filter_map(|prices| get_price_for_diff(prices, &diff))
            .sum::<u64>();

        assert_eq!(23, max)
    }

    #[test]
    fn test_input_b() {
        let result = get_max_price("input.txt");
        assert!(result.is_ok());
        assert_eq!(1568, result.unwrap())
    }
}
