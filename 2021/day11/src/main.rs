use std::collections::HashSet;
use std::fs;
use std::path::Path;

const PART1_STEPS: usize = 100;
const MOVES: [(isize, isize); 8] = [
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
];

fn main() {
    let input = Path::new("2021/day11/src/input.txt");
    let octopuses = parse_octopuses(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Flashes after {:?} steps: {:?}",
        PART1_STEPS,
        flashes_after_steps(&octopuses, PART1_STEPS)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "First synchronized flash at step: {:?}",
        first_synchronized_flash(&octopuses)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_octopuses(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn flashes_after_steps(octopuses: &[Vec<u32>], steps: usize) -> usize {
    let mut count = 0;
    let mut octopuses = octopuses.to_vec();

    for _ in 0..steps {
        let mut next_octopuses = vec![vec![0; octopuses[0].len()]; octopuses.len()];

        for (row_no, row) in octopuses.iter().enumerate() {
            for (col_no, energy) in row.iter().enumerate() {
                next_octopuses[row_no][col_no] = *energy + 1;
            }
        }

        let mut flashed = flash_octopuses(&mut next_octopuses);
        for (row, col) in flashed.drain() {
            next_octopuses[row][col] = 0;
            count += 1;
        }
        octopuses = next_octopuses;
    }
    count
}

fn first_synchronized_flash(octopuses: &[Vec<u32>]) -> usize {
    let mut octopuses = octopuses.to_vec();
    let rows = octopuses.len();
    let cols = octopuses[0].len();
    let octopuses_count = rows * cols;

    for step in 1.. {
        let mut next_octopuses = vec![vec![0; octopuses[0].len()]; octopuses.len()];

        for (row_no, row) in octopuses.iter().enumerate() {
            for (col_no, energy) in row.iter().enumerate() {
                next_octopuses[row_no][col_no] = *energy + 1;
            }
        }

        let mut flashed = flash_octopuses(&mut next_octopuses);
        if flashed.len() == octopuses_count {
            return step;
        }

        for (row, col) in flashed.drain() {
            next_octopuses[row][col] = 0;
        }

        octopuses = next_octopuses;
    }
    unreachable!()
}

fn flash_octopuses(octopuses: &mut [Vec<u32>]) -> HashSet<(usize, usize)> {
    let rows = octopuses.len() as isize;
    let cols = octopuses[0].len() as isize;
    let mut flashed = HashSet::new();
    loop {
        let mut new_flashes = HashSet::new();
        for (row_no, row) in octopuses.iter().enumerate() {
            for (col_no, energy) in row.iter().enumerate() {
                if *energy > 9 && !flashed.contains(&(row_no, col_no)) {
                    new_flashes.insert((row_no, col_no));
                    flashed.insert((row_no, col_no));
                }
            }
        }

        if new_flashes.is_empty() {
            break;
        }

        for (row, col) in new_flashes.drain() {
            for (row_change, col_change) in &MOVES {
                let next_row = row as isize + row_change;
                let next_col = col as isize + col_change;
                if next_row < 0 || next_row >= rows || next_col < 0 || next_col >= cols {
                    continue;
                }
                octopuses[next_row as usize][next_col as usize] += 1;
            }
        }
    }
    flashed
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = "11111
19991
19191
19991
11111";
    const SAMPLE2: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test_part_1_sample() {
        let octopuses1 = parse_octopuses(SAMPLE1);

        assert_eq!(flashes_after_steps(&octopuses1, 1), 9);

        let octopuses2 = parse_octopuses(SAMPLE2);

        assert_eq!(flashes_after_steps(&octopuses2, 100), 1656);
    }

    #[test]
    fn test_part_2_sample() {
        let octopuses = parse_octopuses(SAMPLE2);

        assert_eq!(first_synchronized_flash(&octopuses), 195);
    }
}
