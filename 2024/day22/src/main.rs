use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

fn main() {
    let input = Path::new("2024/day22/src/input.txt");
    let numbers = parse_input(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Sum of the 2000th secret numbers: {:?}",
        sum_nth_secret_number(&numbers, 2000)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!("Most bananas to get: {:?}", most_bananas(&numbers, 2000));
    println!("In {:?}", part2.elapsed());
}

fn parse_input(input: &str) -> Vec<usize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn mix(num: usize, other: usize) -> usize {
    num ^ other
}

fn prune(num: usize) -> usize {
    num % 16777216
}

fn calculate(secret: usize) -> usize {
    let step1 = prune(mix(secret * 64, secret));
    let step2 = prune(mix(step1 / 32, step1));
    prune(mix(step2 * 2048, step2))
}

fn calculate_nth(secret: usize, n: u16) -> usize {
    let mut result = secret;
    for _ in 0..n {
        result = calculate(result);
    }
    result
}

fn sum_nth_secret_number(numbers: &[usize], n: u16) -> usize {
    numbers.iter().map(|number| calculate_nth(*number, n)).sum()
}

fn most_bananas(numbers: &[usize], n: u16) -> i16 {
    let mut sequence_to_bananas: HashMap<[i8; 4], i16> = HashMap::new();
    let mut sequence: [i8; 4] = [0, 0, 0, 0];
    let mut buyer_sequences: HashSet<[i8; 4]> = HashSet::new();
    for initial_number in numbers.iter() {
        buyer_sequences.clear();
        let mut secret = *initial_number;
        let mut banana_count = (initial_number % 10) as i16;
        for index in 0..n {
            secret = calculate(secret);
            let next_bananas = (secret % 10) as i16;
            let change = next_bananas - banana_count;
            banana_count = next_bananas;

            let [_, b, c, d] = sequence;
            sequence = [b, c, d, change as i8];
            if index < 4 || !buyer_sequences.insert(sequence) {
                continue;
            }

            sequence_to_bananas
                .entry(sequence)
                .and_modify(|count| *count += banana_count)
                .or_insert(banana_count);
        }
    }
    *sequence_to_bananas.values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: [usize; 10] = [
        15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432, 5908254,
    ];
    const SAMPLE2: [usize; 4] = [1, 10, 100, 2024];
    const SAMPLE3: [usize; 4] = [1, 2, 3, 2024];

    #[test]
    fn test_calculate() {
        assert_eq!(calculate(123), 15887950);
        assert_eq!(calculate(15887950), 16495136);
    }

    #[test]
    fn test_calculate_nth() {
        for (no, result) in SAMPLE.iter().enumerate() {
            assert_eq!(calculate_nth(123, (no + 1) as u16), *result);
        }

        assert_eq!(calculate_nth(1, 2000), 8685429);
        assert_eq!(calculate_nth(10, 2000), 4700978);
        assert_eq!(calculate_nth(100, 2000), 15273692);
        assert_eq!(calculate_nth(2024, 2000), 8667524);
    }

    #[test]
    fn test_sum_nth_secrets() {
        assert_eq!(sum_nth_secret_number(&SAMPLE2, 2000), 37327623);
    }

    #[test]
    fn test_bananas() {
        assert_eq!(most_bananas(&SAMPLE3, 2000), 23);
    }
}
