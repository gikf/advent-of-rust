use std::fs;
use std::path::Path;

fn main() {
    let input = Path::new("2024/day02/src/input.txt");
    let reports = parse_reports(&fs::read_to_string(input).unwrap());

    println!("Part 1");
    println!(
        "Number of safe reports: {:?}",
        safe_reports(&reports, 3, is_report_safe)
    );

    println!("Part 2");
    println!(
        "Number of safe reports, including single bad level: {:?}",
        safe_reports(&reports, 3, is_report_safe_or_single_bad)
    );
}

fn safe_reports<Validator: Fn(&[usize], usize) -> bool>(
    reports: &[Vec<usize>],
    max_difference: usize,
    validator: Validator,
) -> usize {
    reports
        .iter()
        .filter(|report| validator(report, max_difference))
        .count()
}

fn is_report_safe_or_single_bad(report: &[usize], max_difference: usize) -> bool {
    is_almost(report, max_difference, ascending_comparator, is_ascending)
        || is_almost(report, max_difference, descending_comparator, is_descending)
}

fn ascending_comparator(a: usize, b: usize, max_difference: usize) -> bool {
    a < b && b - a <= max_difference
}

fn descending_comparator(a: usize, b: usize, max_difference: usize) -> bool {
    a > b && a - b <= max_difference
}

fn is_almost<
    Validator: Fn(&[usize], usize) -> bool,
    Comparator: Fn(usize, usize, usize) -> bool,
>(
    numbers: &[usize],
    max_difference: usize,
    comparator: Comparator,
    validator: Validator,
) -> bool {
    for (prev_index, number) in numbers.iter().skip(1).enumerate() {
        if !(comparator(numbers[prev_index], *number, max_difference)) {
            return (prev_index..=prev_index + 1)
                .map(|index_to_exclude| {
                    numbers
                        .iter()
                        .enumerate()
                        .filter_map(|(index, number)| {
                            if index != index_to_exclude {
                                Some(*number)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .any(|numbers| validator(&numbers, max_difference));
        }
    }
    true
}

fn is_ascending(numbers: &[usize], max_difference: usize) -> bool {
    numbers
        .windows(2)
        .all(|window| ascending_comparator(window[0], window[1], max_difference))
}

fn is_descending(numbers: &[usize], max_difference: usize) -> bool {
    numbers
        .windows(2)
        .all(|window| descending_comparator(window[0], window[1], max_difference))
}

fn is_report_safe(report: &[usize], max_difference: usize) -> bool {
    is_ascending(report, max_difference) || is_descending(report, max_difference)
}

fn parse_reports(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num| num.parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";

    #[test]
    fn test_parse_reports() {
        let reports = parse_reports(SAMPLE);

        assert_eq!(
            reports,
            [
                Vec::from([7, 6, 4, 2, 1]),
                Vec::from([1, 2, 7, 8, 9]),
                Vec::from([9, 7, 6, 2, 1]),
                Vec::from([1, 3, 2, 4, 5]),
                Vec::from([8, 6, 4, 4, 1]),
                Vec::from([1, 3, 6, 7, 9])
            ]
        )
    }
    #[test]
    fn test_part_1_sample() {
        let reports = parse_reports(SAMPLE);

        let number_of_safe_reports = safe_reports(&reports, 3, is_report_safe);
        assert_eq!(number_of_safe_reports, 2)
    }

    #[test]
    fn test_part_2_sample() {
        let reports = parse_reports(SAMPLE);

        let number_of_safe_reports = safe_reports(&reports, 3, is_report_safe_or_single_bad);
        assert_eq!(number_of_safe_reports, 4)
    }
}
