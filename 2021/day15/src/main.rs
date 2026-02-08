use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};
use std::fs;
use std::path::Path;

fn main() {
    let input = Path::new("2021/day15/src/input.txt");
    let cavern = parse_cavern(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!("Lowest total risk: {:?}", lowest_risk(&cavern));
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Lowest total risk in actual cave: {:?}",
        lowest_risk_repeating_map(&cavern, 5)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_cavern(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|v| v.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn lowest_risk(cavern: &[Vec<usize>]) -> usize {
    let rows = cavern.len();
    let cols = cavern[0].len();
    let mut risks = vec![vec![usize::MAX; cols]; rows];
    let start = (0, 0);

    let mut queue = VecDeque::new();
    queue.push_back((0, start));

    while let Some((risk_so_far, (row, col))) = queue.pop_front() {
        if risks[row][col] <= risk_so_far {
            continue;
        }
        risks[row][col] = risk_so_far;

        for (row_change, col_change) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let next_row = row as isize + row_change;
            let next_col = col as isize + col_change;

            if next_row < 0
                || next_col < 0
                || next_row >= (rows as isize)
                || next_col >= (cols as isize)
            {
                continue;
            }

            let next_row = next_row as usize;
            let next_col = next_col as usize;

            let next_risk = cavern[next_row][next_col];

            queue.push_back((risk_so_far + next_risk, (next_row, next_col)));
        }
    }
    *risks.last().unwrap().last().unwrap()
}

fn lowest_risk_repeating_map(cavern: &[Vec<usize>], repeats: usize) -> usize {
    let map_rows = cavern.len();
    let map_cols = cavern[0].len();
    let rows = map_rows * repeats;
    let cols = map_cols * repeats;
    let mut risks = vec![vec![0; cols]; rows];

    let start = (0, 0);
    let end = (rows - 1, cols - 1);

    let max_row = (rows - 1) as isize;
    let max_col = (cols - 1) as isize;

    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, start)));

    while let Some(Reverse((risk_so_far, (row, col)))) = queue.pop() {
        if risks[row][col] > 0 && risks[row][col] <= risk_so_far {
            continue;
        }
        risks[row][col] = risk_so_far;

        if (row, col) == end {
            break;
        }

        for (row_change, col_change) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let next_row = row as isize + row_change;
            let next_col = col as isize + col_change;

            if next_row < 0 || next_col < 0 || next_row > max_row || next_col > max_col {
                continue;
            }

            let next_row = next_row as usize;
            let next_col = next_col as usize;
            if next_row == row && next_col == col {
                continue;
            }
            let original_risk =
                cavern[next_row.rem_euclid(map_rows)][next_col.rem_euclid(map_cols)];
            let risk_offset = next_row / (map_rows) + next_col / (map_cols);
            let risk = original_risk + risk_offset;
            let next_risk = if risk == 9 { 9 } else { risk.rem_euclid(9) };
            let total_next = risk_so_far + next_risk;

            let current_risk = risks[next_row][next_col];
            if current_risk == 0 || current_risk > total_next {
                queue.push(Reverse((total_next, (next_row, next_col))));
            }
        }
    }
    *risks.last().unwrap().last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn test_part_1_sample() {
        let cavern = parse_cavern(SAMPLE);

        assert_eq!(lowest_risk(&cavern), 40);
    }

    #[test]
    fn test_part_2_sample() {
        let cavern = parse_cavern(SAMPLE);

        assert_eq!(lowest_risk_repeating_map(&cavern, 5), 315);
    }
}
