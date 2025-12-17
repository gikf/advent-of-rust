use std::fs;
use std::path::Path;

const DO: &str = "do()";
const DONT: &str = "don't()";
const MUL: &str = "mul(";
const MUL_CLOSING: &str = ")";

fn main() {
    let input = Path::new("2024/day03/src/input.txt");
    let numbers = parse_memory_part_1(&fs::read_to_string(input).unwrap());

    println!("Part 1");
    println!(
        "Sum of uncorrupted multiplications: {:?}",
        uncorrupted_mul(&numbers)
    );

    let numbers2 = parse_memory_part_2(&fs::read_to_string(input).unwrap());
    println!("Part 2");
    println!(
        "Sum of enabled uncorrupted multiplications: {:?}",
        uncorrupted_mul(&numbers2)
    );
}

fn uncorrupted_mul(numbers: &[(usize, usize)]) -> usize {
    numbers.iter().map(|(a, b)| *a * *b).sum()
}

fn parse_memory_part_1(input: &str) -> Vec<(usize, usize)> {
    input
        .split(MUL)
        .into_iter()
        .filter_map(|part| {
            let split: Vec<_> = part[0..].split(MUL_CLOSING).collect();
            parse_maybe_numbers(split[0])
        })
        .collect()
}

fn parse_memory_part_2(input: &str) -> Vec<(usize, usize)> {
    let mut numbers = Vec::new();
    let mut parsing_numbers_enabled = true;
    let mut start = 0;

    loop {
        match (
            parsing_numbers_enabled,
            input[start..].find(DO),
            input[start..].find(MUL),
            input[start..].find(DONT),
        ) {
            (true, _, Some(mul_index), Some(dont_index)) if mul_index < dont_index => {
                start += mul_index + MUL.len();
                if let Some(closing_index) = input[start..].find(MUL_CLOSING)
                    && let Some(pair) = parse_maybe_numbers(&input[start..(start + closing_index)])
                {
                    numbers.push(pair);
                }
            }
            (true, _, _, Some(dont_index)) => {
                start += dont_index + DONT.len();
                parsing_numbers_enabled = false;
            }
            (true, _, Some(mul_index), None) => {
                start += mul_index + MUL.len();
                if let Some(closing_index) = input[start..].find(MUL_CLOSING)
                    && let Some(pair) = parse_maybe_numbers(&input[start..(start + closing_index)])
                {
                    numbers.push(pair);
                }
            }
            (true, _, None, _) => {
                break;
            }
            (false, Some(do_index), _, _) => {
                start += do_index + DO.len();
                parsing_numbers_enabled = true;
            }
            _ => {}
        }
    }
    numbers
}

fn parse_maybe_numbers(maybe_numbers: &str) -> Option<(usize, usize)> {
    if let [maybe_num1, maybe_num2] = maybe_numbers.split(',').collect::<Vec<_>>()[..]
        && let (Ok(a), Ok(b)) = (maybe_num1.parse(), maybe_num2.parse())
    {
        Some((a, b))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const SAMPLE2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_parse_memory() {
        let numbers = parse_memory_part_1(SAMPLE);

        assert_eq!(numbers, [(2, 4), (5, 5), (11, 8), (8, 5),])
    }

    #[test]
    fn test_part_1_sample() {
        let numbers = parse_memory_part_1(SAMPLE);
        let result = uncorrupted_mul(&numbers);

        assert_eq!(result, 161);
    }

    #[test]
    fn test_parse_memory_part_2() {
        let numbers = parse_memory_part_2(SAMPLE2);

        assert_eq!(numbers, &[(2, 4), (8, 5)]);
    }

    #[test]
    fn test_part_2_sample() {
        let numbers = parse_memory_part_2(SAMPLE2);
        let result = uncorrupted_mul(&numbers);

        assert_eq!(result, 48);
    }
}
