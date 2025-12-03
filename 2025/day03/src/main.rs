use std::fs;
use std::path::Path;

fn main() {
    let input = Path::new("2025/day03/src/input.txt");
    let batteries = &fs::read_to_string(input).unwrap();

    println!("Step 1");
    println!("Total output: {}", total_output(batteries, 2));

    println!("Step 2");
    println!("Total output: {}", total_output(batteries, 12));
}

fn find_joltage(batteries: &str, batteries_on: usize) -> u64 {
    let length = batteries.len();
    let mut digits = Vec::new();
    let mut start = 0;

    for offset in (0..batteries_on).rev() {
        let to_take = length - offset - start;

        let relevant = &batteries
            .char_indices()
            .skip(start)
            .take(to_take)
            .collect::<Vec<_>>();

        let (position, digit) = relevant
            .iter()
            .max_by_key(|(index, d)| (*d, -(*index as i64)))
            .unwrap();

        start = position + 1;

        digits.push(*digit);
    }
    digits.iter().collect::<String>().parse().unwrap()
}

fn total_output(batteries: &str, batteries_on: usize) -> u64 {
    batteries
        .lines()
        .map(|battery| find_joltage(battery, batteries_on))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "987654321111111\n811111111111119\n234234234234278\n818181911112111";

    #[test]
    fn test_find_joltage_step_1() {
        assert_eq!(find_joltage("987654321111111", 2), 98_u64);
        assert_eq!(find_joltage("811111111111119", 2), 89_u64);
        assert_eq!(find_joltage("234234234234278", 2), 78_u64);
        assert_eq!(find_joltage("818181911112111", 2), 92_u64);
    }

    #[test]
    fn test_total_output_step_1() {
        assert_eq!(total_output(SAMPLE, 2), 357);
    }

    #[test]
    fn test_find_joltage_step_2() {
        assert_eq!(find_joltage("987654321111111", 12), 987654321111_u64);
        assert_eq!(find_joltage("811111111111119", 12), 811111111119_u64);
        assert_eq!(find_joltage("234234234234278", 12), 434234234278_u64);
        assert_eq!(find_joltage("818181911112111", 12), 888911112111_u64);
    }

    #[test]
    fn test_total_output_step_2() {
        assert_eq!(total_output(SAMPLE, 12), 3121910778619);
    }
}
