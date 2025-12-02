use std::collections::HashSet;
use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq)]
struct Range {
    start: u64,
    end: u64,
}

fn main() {
    let input = Path::new("2025/day02/src/input.txt");
    let ranges = parse_input(&fs::read_to_string(input).unwrap());

    println!("Step 1");
    println!(
        "Sum of invalid IDs: {}",
        count_invalid_ranges(&ranges, is_valid_step1)
    );

    println!("Step 2");
    println!(
        "Sum of invalid IDs: {}",
        count_invalid_ranges(&ranges, is_valid_step2)
    );
}

fn parse_input(input: &str) -> Vec<Range> {
    input
        .split(',')
        .map(|range| match range.split_once('-') {
            Some((start, end)) => Range {
                start: start.parse::<u64>().expect("Not a number"),
                end: end.parse::<u64>().expect("Not a number"),
            },
            None => unreachable!("{:?}", range),
        })
        .collect()
}

fn is_valid_step1(id: &u64) -> bool {
    let id_as_string = id.to_string();
    let length = id_as_string.len();
    if !length.is_multiple_of(2) {
        true
    } else {
        let middle = length / 2;
        id_as_string
            .chars()
            .take(middle)
            .zip(id_as_string.chars().skip(middle))
            .any(|(left, right)| left != right)
    }
}

fn is_valid_step2(id: &u64) -> bool {
    let id_as_string = id.to_string();
    let length = id_as_string.len();
    for chunk_size in (1..=(length.div_euclid(2))).rev() {
        if !length.is_multiple_of(chunk_size) {
            continue;
        }
        let uniq = &id_as_string
            .as_bytes()
            .chunks(chunk_size)
            .collect::<HashSet<_>>();
        if uniq.len() == 1 {
            return false;
        }
    }
    true
}

fn count_invalid_ranges<T: Fn(&u64) -> bool>(ranges: &[Range], func: T) -> u64 {
    ranges
        .iter()
        .flat_map(|range| (range.start..=range.end).filter(|candidate| !func(candidate)))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_is_valid_step1() {
        assert_eq!(is_valid_step1(&11_u64), false);
        assert_eq!(is_valid_step1(&1010_u64), false);
        assert_eq!(is_valid_step1(&38593859_u64), false);
        assert_eq!(is_valid_step1(&111_u64), true);
        assert_eq!(is_valid_step1(&101_u64), true);
        assert_eq!(is_valid_step1(&1312_u64), true);
        assert_eq!(is_valid_step1(&1331_u64), true);
    }

    #[test]
    fn test_is_valid_step2() {
        assert_eq!(is_valid_step2(&11_u64), false);
        assert_eq!(is_valid_step2(&1010_u64), false);
        assert_eq!(is_valid_step2(&38593859_u64), false);
        assert_eq!(is_valid_step2(&111_u64), false);
        assert_eq!(is_valid_step2(&565656_u64), false);
        assert_eq!(is_valid_step2(&824824824_u64), false);
        assert_eq!(is_valid_step2(&2121212121_u64), false);
        assert_eq!(is_valid_step2(&101_u64), true);
        assert_eq!(is_valid_step2(&1312_u64), true);
        assert_eq!(is_valid_step2(&1331_u64), true);
    }

    #[test]
    fn test_parse_input() {
        let result = parse_input(SAMPLE);
        assert_eq!(
            Vec::from([
                Range {
                    start: 11_u64,
                    end: 22_u64,
                },
                Range {
                    start: 95_u64,
                    end: 115_u64,
                },
                Range {
                    start: 998_u64,
                    end: 1012_u64,
                },
                Range {
                    start: 1188511880_u64,
                    end: 1188511890_u64,
                },
                Range {
                    start: 222220_u64,
                    end: 222224_u64,
                },
                Range {
                    start: 1698522_u64,
                    end: 1698528_u64,
                },
                Range {
                    start: 446443_u64,
                    end: 446449_u64,
                },
                Range {
                    start: 38593856_u64,
                    end: 38593862_u64,
                },
                Range {
                    start: 565653_u64,
                    end: 565659_u64,
                },
                Range {
                    start: 824824821_u64,
                    end: 824824827_u64,
                },
                Range {
                    start: 2121212118_u64,
                    end: 2121212124_u64,
                },
            ]),
            result
        );
    }

    #[test]
    fn test_step_1_sample_input() {
        let ranges = parse_input(SAMPLE);
        assert_eq!(count_invalid_ranges(&ranges, is_valid_step1), 1227775554);
    }

    #[test]
    fn test_step_2_sample_input() {
        let ranges = parse_input(SAMPLE);
        assert_eq!(count_invalid_ranges(&ranges, is_valid_step2), 4174379265);
    }
}
