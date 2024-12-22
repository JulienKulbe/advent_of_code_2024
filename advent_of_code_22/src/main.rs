use anyhow::Result;
use std::fs::read_to_string;

fn main() -> Result<()> {
    let sum = sum_secret_numbers("input.txt")?;
    println!("Sum: {sum}");

    let score = calculate_similarity_score("input.txt")?;
    println!("Similarity score: {score}");

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

fn calculate_similarity_score(filename: &str) -> Result<u32> {
    Ok(1)
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
        assert_eq!(1579939, result.unwrap())
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
