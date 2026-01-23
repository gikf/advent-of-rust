use std::fs;
use std::path::Path;

const DAYS_PART1: usize = 80;
const DAYS_PART2: usize = 256;

fn main() {
    let input = Path::new("2021/day06/src/input.txt");
    let fishes = parse_fishes(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Fishes after {:?} days: {:?}",
        DAYS_PART1,
        count_fishes_after_n_days(&fishes, DAYS_PART1)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Fishes after {:?} days: {:?}",
        DAYS_PART2,
        count_fishes_after_n_days(&fishes, DAYS_PART2)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_fishes(input: &str) -> [usize; 9] {
    let mut fishes = [0; 9];
    input.split(',').for_each(|num| {
        fishes[num.parse::<usize>().unwrap()] += 1;
    });

    fishes
}

fn count_fishes_after_n_days(fishes: &[usize; 9], days: usize) -> usize {
    let mut fishes = *fishes;

    for _ in 0..days {
        let mut next_fishes = [0; 9];
        for index in (0..8).rev() {
            next_fishes[index] = fishes[index + 1];
        }
        next_fishes[6] += fishes[0];
        next_fishes[8] = fishes[0];

        fishes = next_fishes;
    }
    fishes.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "3,4,3,1,2";

    #[test]
    fn test_part_1_sample() {
        let fishes = parse_fishes(SAMPLE);

        assert_eq!(count_fishes_after_n_days(&fishes, DAYS_PART1), 5934);
    }

    #[test]
    fn test_part_2_sample() {
        let fishes = parse_fishes(SAMPLE);

        assert_eq!(count_fishes_after_n_days(&fishes, DAYS_PART2), 26984457539);
    }
}
