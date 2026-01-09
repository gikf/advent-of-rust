use std::fs;
use std::path::Path;

fn main() {
    let input = Path::new("2023/day09/src/input.txt");
    let histories = parse_histories(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Sum of extrapolated values: {:?}",
        sum_of_extrapolated_values(&histories)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Sum of backward extrapolation values: {:?}",
        sum_of_backward_extrapolated_values(&histories)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_histories(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect()
}

fn sum_of_extrapolated_values(histories: &[Vec<isize>]) -> isize {
    histories.iter().map(|history| extrapolate(history)).sum()
}

fn sum_of_backward_extrapolated_values(histories: &[Vec<isize>]) -> isize {
    histories
        .iter()
        .map(|history| extrapolate(&history.iter().copied().rev().collect::<Vec<_>>()))
        .sum()
}

fn extrapolate(history: &[isize]) -> isize {
    let mut diffs: Vec<Vec<isize>> = vec![history.to_vec()];
    diffs.push(
        history
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect(),
    );

    while diffs.last().unwrap().iter().any(|num| *num != 0) {
        diffs.push(
            diffs
                .last()
                .unwrap()
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect(),
        );
    }

    let mut increment = 0;
    diffs.iter_mut().rev().skip(1).for_each(|diff| {
        let next_value = increment + diff.last().unwrap();
        diff.push(next_value);
        increment = next_value;
    });
    *diffs.first().unwrap().last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_parse_histories() {
        let histories = parse_histories(SAMPLE);

        assert_eq!(
            histories,
            [
                Vec::from([0, 3, 6, 9, 12, 15]),
                Vec::from([1, 3, 6, 10, 15, 21]),
                Vec::from([10, 13, 16, 21, 30, 45]),
            ]
        );
    }

    #[test]
    fn test_extrapolate() {
        let histories = parse_histories(SAMPLE);

        assert_eq!(extrapolate(&histories[0]), 18);
        assert_eq!(extrapolate(&histories[1]), 28);
        assert_eq!(extrapolate(&histories[2]), 68);
    }

    #[test]
    fn test_extrapolate_backwards() {
        let histories = parse_histories(SAMPLE);

        assert_eq!(
            extrapolate(&histories[0].iter().copied().rev().collect::<Vec<_>>()),
            -3
        );
        assert_eq!(
            extrapolate(&histories[1].iter().copied().rev().collect::<Vec<_>>()),
            0
        );
        assert_eq!(
            extrapolate(&histories[2].iter().copied().rev().collect::<Vec<_>>()),
            5
        );
    }

    #[test]
    fn test_sum_of_extrapolated_values() {
        let histories = parse_histories(SAMPLE);

        assert_eq!(sum_of_extrapolated_values(&histories), 114);
    }

    #[test]
    fn test_sum_of_backward_extrapolated_values() {
        let histories = parse_histories(SAMPLE);
        assert_eq!(sum_of_backward_extrapolated_values(&histories), 2);
    }
}
