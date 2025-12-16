use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn main() {
    let input = Path::new("2024/day01/src/input.txt");
    let (mut left, mut right) = parse_numbers(&fs::read_to_string(input).unwrap());

    println!("Part 1");
    println!(
        "Total distance: {:?}",
        total_distance(&mut left, &mut right)
    );

    println!("Part 2");
    println!("Similarity score: {:?}", similarity_score(&left, &right));
}

fn parse_numbers(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let mut split = line.split_ascii_whitespace();
        let a = split.next().unwrap();
        let b = split.next().unwrap();

        left.push(a.parse().unwrap());
        right.push(b.parse().unwrap());
    }

    (left, right)
}

fn total_distance(left: &mut [usize], right: &mut [usize]) -> usize {
    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(a, b)| a.max(b) - a.min(b))
        .sum()
}

fn similarity_score(left: &[usize], right: &[usize]) -> usize {
    let mut right_to_count = HashMap::new();

    for number in right {
        right_to_count
            .entry(number)
            .and_modify(|value| *value += 1)
            .or_insert(1);
    }

    left.iter()
        .map(|number| number * right_to_count.get(number).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";

    #[test]
    fn test_parse_numbers() {
        let (left, right) = parse_numbers(SAMPLE);

        assert_eq!(left, [3, 4, 2, 1, 3, 3]);
        assert_eq!(right, [4, 3, 5, 3, 9, 3]);
    }

    #[test]
    fn test_part_1_sample() {
        let (mut left, mut right) = parse_numbers(SAMPLE);

        let result = total_distance(&mut left, &mut right);
        assert_eq!(result, 11);
    }

    #[test]
    fn test_part_2_sample() {
        let (left, right) = parse_numbers(SAMPLE);

        let result = similarity_score(&left, &right);
        assert_eq!(result, 31);
    }
}
