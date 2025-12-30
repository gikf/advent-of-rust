use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs;
use std::path::Path;

#[derive(Clone, Debug, PartialEq)]
enum Field {
    Corrupted,
    Empty,
    Visited(usize),
}

fn main() {
    let input = Path::new("2024/day18/src/input.txt");
    let bytes = parse_bytes(&fs::read_to_string(input).unwrap());

    let height = 70;
    let width = 70;
    let initial_corrupted_bytes = 1024;

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Minimum number of steps: {:?}",
        find_path(&bytes, initial_corrupted_bytes, 70, 70)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Coordinates of corrupting byte, preventing exit (X, Y): {:?}",
        find_byte_preventing_exit(&bytes, width, height)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_bytes(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|line| {
            let split = line.split_once(",").unwrap();
            (split.1.parse().unwrap(), split.0.parse().unwrap())
        })
        .collect()
}

fn find_path(bytes: &[(usize, usize)], count: usize, width: usize, height: usize) -> usize {
    let mut grid = vec![vec![Field::Empty; width + 1]; height + 1];

    for byte in bytes.iter().take(count) {
        grid[byte.0][byte.1] = Field::Corrupted;
    }

    let mut queue: BinaryHeap<Reverse<(isize, usize, usize, usize)>> = BinaryHeap::new();
    queue.push(Reverse((0, 0, 0, 0)));

    while let Some(Reverse((_, row, col, step))) = queue.pop() {
        if let Field::Visited(value) = grid[row][col] && value < step {
            continue;
        }

        grid[row][col] = Field::Visited(step);

        for (next_row, next_col) in moves(row, col, height + 1, width + 1) {
            let next_field = &grid[next_row][next_col];
            if matches!(next_field, Field::Empty) || matches!(next_field, Field::Visited(value) if *value > step + 1)
            {
                let distance_to_target = ((height as isize - row as isize).pow(2)
                    + (width as isize - col as isize).pow(2))
                .isqrt();
                queue.push(Reverse((distance_to_target, next_row, next_col, step + 1)));
            }
        }
    }

    if let Field::Visited(number_of_steps) = grid[height][width] {
        number_of_steps
    } else {
        usize::MAX
    }
}

fn find_byte_preventing_exit(
    bytes: &[(usize, usize)],
    width: usize,
    height: usize,
) -> (usize, usize) {
    let mut grid = vec![vec![Field::Empty; width + 1]; height + 1];

    for byte in bytes.iter() {
        grid[byte.0][byte.1] = Field::Corrupted;
    }

    let mut steps: Vec<Vec<Option<usize>>> = vec![vec![None; width + 1]; height + 1];
    let mut queue: BinaryHeap<Reverse<(isize, usize, usize, usize)>> = BinaryHeap::new();

    for (byte_row, byte_col) in bytes.iter().rev() {
        grid[*byte_row][*byte_col] = Field::Empty;
        queue.push(Reverse((0, 0, 0, 0)));

        while let Some(Reverse((_, row, col, step))) = queue.pop() {
            if steps[row][col].is_some() {
                continue;
            } else if row == height && col == width {
                return (*byte_col, *byte_row);
            }

            steps[row][col] = Some(step);

            for (next_row, next_col) in moves(row, col, height + 1, width + 1) {
                if steps[next_row][next_col].is_none() && grid[next_row][next_col] == Field::Empty {
                    let distance_to_target = ((height as isize - row as isize).pow(2)
                        + (width as isize - col as isize).pow(2))
                    .isqrt();
                    queue.push(Reverse((distance_to_target, next_row, next_col, step + 1)));
                }
            }
        }
        queue.clear();
        steps.fill(vec![None; width + 1]);
    }
    (0, 0)
}

fn moves(
    row: usize,
    col: usize,
    max_row: usize,
    max_col: usize,
) -> impl Iterator<Item = (usize, usize)> {
    [(-1, 0), (0, 1), (1, 0), (0, -1)]
        .iter()
        .filter_map(move |(row_change, col_change)| {
            let next_row = row as isize + row_change;
            if next_row < 0 || next_row >= max_row as isize {
                return None;
            }
            let next_col = col as isize + col_change;
            if next_col < 0 || next_col >= max_col as isize {
                return None;
            }
            Some((next_row as usize, next_col as usize))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "5,4\n4,2\n4,5\n3,0\n2,1\n6,3\n2,4\n1,5\n0,6\n3,3\n2,6
5,1\n1,2\n5,5\n2,5\n6,5\n1,4\n0,4\n6,4\n1,1\n6,1\n1,0\n0,5\n1,6\n2,0";

    #[test]
    fn test_part_1_sample() {
        let bytes = parse_bytes(SAMPLE);
        let height = 6;
        let width = 6;

        let min_steps = find_path(&bytes, 12, width, height);
        assert_eq!(min_steps, 22);
    }

    #[test]
    fn test_part_2_sample() {
        let bytes = parse_bytes(SAMPLE);
        let height = 6;
        let width = 6;

        let byte = find_byte_preventing_exit(&bytes, width, height);
        assert_eq!(byte, (6, 1));
    }
}
