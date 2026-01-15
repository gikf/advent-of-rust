use std::fs;
use std::path::Path;

fn main() {
    let input = Path::new("2021/day01/src/input.txt");
    let measurements = parse_measurements(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Increased measurements: {:?}",
        count_increases(&measurements)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    let window_size = 3;
    println!("Part 2");
    println!(
        "Increased measurements in three-measurement window: {:?}",
        count_increases_in_window(&measurements, window_size)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_measurements(input: &str) -> Vec<usize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn count_increases(measurements: &[usize]) -> usize {
    measurements
        .windows(2)
        .filter(|window| if let [a, b] = window { a < b } else { false })
        .count()
}

fn count_increases_in_window(measurements: &[usize], window_length: usize) -> usize {
    measurements
        .windows(window_length)
        .zip(measurements.windows(window_length).skip(1))
        .filter(|(a, b)| a.iter().sum::<usize>() < b.iter().sum())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";

    #[test]
    fn test_part_1_sample() {
        let measurements = parse_measurements(SAMPLE);

        assert_eq!(count_increases(&measurements), 7);
    }

    #[test]
    fn test_part_2_sample() {
        let measurements = parse_measurements(SAMPLE);

        assert_eq!(count_increases_in_window(&measurements, 3), 5);
    }
}
