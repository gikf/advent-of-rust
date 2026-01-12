use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => unimplemented!(),
        }
    }
}

fn main() {
    let input = Path::new("2023/day12/src/input.txt");
    let content = fs::read_to_string(input).unwrap();
    let records = parse_records(&content);

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!("Sum of arrangements: {:?}", sum_arrangements(&records));
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    let unfolded_records = parse_unfolded_records(&content);
    println!("Part 2");
    println!(
        "Sum of unfolded arrangements: {:?}",
        sum_arrangements(&unfolded_records)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_records(input: &str) -> Vec<(Vec<usize>, Vec<Spring>)> {
    input
        .lines()
        .map(|line| {
            let (springs, conditions) = line.split_once(" ").unwrap();
            let springs = springs.chars().map(Spring::from).collect();
            let conditions = conditions
                .split(',')
                .map(|num| num.parse().unwrap())
                .collect();

            (conditions, springs)
        })
        .collect()
}

fn parse_unfolded_records(input: &str) -> Vec<(Vec<usize>, Vec<Spring>)> {
    input
        .lines()
        .map(|line| {
            let (springs, conditions) = line.split_once(" ").unwrap();
            let springs = [springs; 5].join("?").chars().map(Spring::from).collect();

            let conditions = [conditions; 5]
                .join(",")
                .split(',')
                .map(|num| num.parse().unwrap())
                .collect();

            (conditions, springs)
        })
        .collect()
}

fn count_arrangements(spring: &(Vec<usize>, Vec<Spring>)) -> usize {
    let (expected, records) = spring;

    let take_count = records
        .iter()
        .rev()
        .skip_while(|s| matches!(s, Spring::Operational))
        .count();
    let records: Vec<_> = [Spring::Operational]
        .iter()
        .chain(records.iter().take(take_count))
        .collect();

    let mut arrangements = vec![0; records.len() + 1];
    arrangements[0] = 1;

    for (index, _) in records
        .iter()
        .take_while(|s| !matches!(s, Spring::Damaged))
        .enumerate()
    {
        arrangements[index + 1] = 1;
    }

    for &group in expected {
        let mut next_arrangements = vec![0; records.len() + 1];
        let mut length = 0;

        for (index, s) in records.iter().enumerate() {
            if !matches!(s, Spring::Operational) {
                length += 1;
            } else {
                length = 0;
            }

            if !matches!(s, Spring::Damaged) {
                next_arrangements[index + 1] += next_arrangements[index];
            }

            if length >= group && !matches!(records[index - group], Spring::Damaged) {
                next_arrangements[index + 1] += arrangements[index - group];
            }
        }
        arrangements = next_arrangements;
    }
    *arrangements.last().unwrap()
}

fn sum_arrangements(springs: &[(Vec<usize>, Vec<Spring>)]) -> usize {
    springs.iter().map(count_arrangements).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_count_arrangements() {
        let records = parse_records(SAMPLE);

        assert_eq!(count_arrangements(&records[0]), 1);
        assert_eq!(count_arrangements(&records[1]), 4);
        assert_eq!(count_arrangements(&records[2]), 1);
        assert_eq!(count_arrangements(&records[3]), 1);
        assert_eq!(count_arrangements(&records[4]), 4);
        assert_eq!(count_arrangements(&records[5]), 10);
    }

    #[test]
    fn test_unfolded_arrangements() {
        let records = parse_unfolded_records(SAMPLE);

        assert_eq!(count_arrangements(&records[0]), 1);
        assert_eq!(count_arrangements(&records[1]), 16384);
        assert_eq!(count_arrangements(&records[2]), 1);
        assert_eq!(count_arrangements(&records[3]), 16);
        assert_eq!(count_arrangements(&records[4]), 2500);
        assert_eq!(count_arrangements(&records[5]), 506250);
    }

    #[test]
    fn test_part_1_sample() {
        let records = parse_records(SAMPLE);

        assert_eq!(sum_arrangements(&records), 21);
    }

    #[test]
    fn test_part_2_sample() {
        let records = parse_unfolded_records(SAMPLE);

        assert_eq!(sum_arrangements(&records), 525152);
    }
}
