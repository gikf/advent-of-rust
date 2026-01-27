use std::collections::{HashSet, VecDeque};
use std::fs;
use std::path::Path;

const PART2_STEPS: usize = 26501365;

#[derive(Debug, PartialEq)]
enum Field {
    Garden,
    Rocks,
    Start,
}

impl From<char> for Field {
    fn from(value: char) -> Self {
        match value {
            '.' => Field::Garden,
            '#' => Field::Rocks,
            'S' => Field::Start,
            _ => unimplemented!(),
        }
    }
}

fn main() {
    let input = Path::new("2023/day21/src/input.txt");
    let map = parse_map(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Number of plots reachabale after 64 steps: {:?}",
        possible_steps_after_step(&map, 64)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Number of plots: {:?}",
        possible_steps_after_gazilion_step(&map, PART2_STEPS)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_map(input: &str) -> Vec<Vec<Field>> {
    input
        .lines()
        .map(|line| line.chars().map(Field::from).collect())
        .collect()
}

fn possible_steps_after_gazilion_step(map: &[Vec<Field>], n_steps: usize) -> usize {
    // Based on https://github.com/CalSimmon/advent-of-code/blob/main/2023/day_21/solution.py
    //
    let size = map.len();
    let edge = size / 2;

    let ys: Vec<_> = (0..3)
        .map(|i| possible_steps_after_step(map, edge + i * size))
        .collect();

    let a = (ys[2] - 2 * ys[1] + ys[0]) / 2;
    let b = ys[1] - ys[0] - a;
    let c = ys[0];

    let n = (n_steps - edge) / size;

    a * n.pow(2) + b * n + c
}

fn possible_steps_after_step(map: &[Vec<Field>], n_steps: usize) -> usize {
    let max_rows = map.len() as isize;
    let max_cols = map[0].len() as isize;

    let start = map
        .iter()
        .enumerate()
        .find_map(|(row_no, row)| {
            row.iter().enumerate().find_map(|(col_no, f)| {
                if matches!(*f, Field::Start) {
                    Some((row_no as isize, col_no as isize))
                } else {
                    None
                }
            })
        })
        .unwrap();

    let mut visited = HashSet::new();
    let mut distances = vec![0; n_steps + 1];

    let mut queue = VecDeque::new();

    queue.push_back((0, start));

    while let Some((step, (row, col))) = queue.pop_front() {
        if !visited.insert((row, col)) || step == n_steps + 1 {
            continue;
        }
        distances[step] += 1;

        for (row_change, col_change) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let next_row = row + row_change;
            let next_col = col + col_change;

            if matches!(
                map[next_row.rem_euclid(max_rows) as usize][next_col.rem_euclid(max_cols) as usize],
                Field::Rocks
            ) {
                continue;
            }

            queue.push_back((step + 1, (next_row, next_col)));
        }
    }

    distances
        .iter()
        .enumerate()
        .filter_map(|(step, count)| {
            if step % 2 == n_steps % 2 {
                Some(count)
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn test_possible_points_after_steps_10() {
        let map = parse_map(SAMPLE);
        let n_plots = possible_steps_after_step(&map, 10);

        assert_eq!(n_plots, 50);
    }

    #[test]
    fn test_possible_points_after_steps_50() {
        let map = parse_map(SAMPLE);
        let n_plots = possible_steps_after_step(&map, 50);

        assert_eq!(n_plots, 1594);
    }

    #[test]
    fn test_possible_points_after_steps_100() {
        let map = parse_map(SAMPLE);
        let n_plots = possible_steps_after_step(&map, 100);

        assert_eq!(n_plots, 6536);
    }

    #[test]
    fn test_possible_points_after_steps_500() {
        let map = parse_map(SAMPLE);
        let n_plots = possible_steps_after_step(&map, 500);

        assert_eq!(n_plots, 167004);
    }

    #[test]
    fn test_part_1_sample() {
        let map = parse_map(SAMPLE);

        let n_plots = possible_steps_after_step(&map, 6);

        assert_eq!(n_plots, 16);
    }
}
