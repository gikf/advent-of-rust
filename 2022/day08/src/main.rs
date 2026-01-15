use std::collections::HashSet;
use std::fs;
use std::path::Path;

fn main() {
    let input = Path::new("2022/day08/src/input.txt");
    let grid = parse_grid(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Number of trees visible from outside of the grid: {:?}",
        count_visible_trees(&grid)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!("Highest scenic score: {:?}", highest_scenic_score(&grid));
    println!("In {:?}", part2.elapsed());
}

fn parse_grid(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|digit| digit.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn count_visible_trees(grid: &[Vec<u8>]) -> usize {
    let mut visible = HashSet::new();

    for (row_no, row) in grid.iter().enumerate() {
        let mut cur_highest = row[0];
        visible.insert((row_no, 0));
        for (col_no, tree) in row.iter().enumerate().skip(1) {
            if *tree > cur_highest {
                cur_highest = *tree;
                visible.insert((row_no, col_no));
            }
            if cur_highest == 9 {
                break;
            }
        }
    }

    for (row_no, row) in grid.iter().enumerate() {
        let mut cur_highest = row[row.len() - 1];
        visible.insert((row_no, row.len() - 1));
        for (col_no, tree) in row.iter().enumerate().rev().skip(1) {
            if *tree > cur_highest {
                cur_highest = *tree;
                visible.insert((row_no, col_no));
            }
            if cur_highest == 9 {
                break;
            }
        }
    }

    for col_no in 0..(grid[0].len() - 1) {
        let mut cur_highest = grid[0][col_no];
        visible.insert((0, col_no));
        for (row_no, row) in grid.iter().enumerate().skip(1) {
            let tree = row[col_no];
            if tree > cur_highest {
                cur_highest = tree;
                visible.insert((row_no, col_no));
            }
            if cur_highest == 9 {
                break;
            }
        }
    }

    for col_no in 0..(grid[0].len() - 1) {
        let mut cur_highest = grid[grid.len() - 1][col_no];
        visible.insert((grid.len() - 1, col_no));
        for (row_no, row) in grid.iter().enumerate().rev().skip(1) {
            let tree = row[col_no];
            if tree > cur_highest {
                cur_highest = tree;
                visible.insert((row_no, col_no));
            }
            if cur_highest == 9 {
                break;
            }
        }
    }

    visible.len()
}

fn highest_scenic_score(grid: &[Vec<u8>]) -> usize {
    grid.iter()
        .enumerate()
        .map(|(row_no, row)| {
            row.iter()
                .enumerate()
                .map(|(col_no, _)| scenic_score(grid, row_no, col_no))
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

fn scenic_score(grid: &[Vec<u8>], row_no: usize, col_no: usize) -> usize {
    let mut up = 0;
    let source_tree = grid[row_no][col_no];
    for row in grid.iter().take(row_no).rev() {
        let tree = row[col_no];
        up += 1;
        if tree >= source_tree {
            break;
        }
    }

    let mut left = 0;
    for &tree in grid[row_no].iter().take(col_no).rev() {
        left += 1;
        if tree >= source_tree {
            break;
        }
    }

    let mut right = 0;
    for &tree in grid[row_no].iter().take(grid[0].len()).skip(col_no + 1) {
        right += 1;
        if tree >= source_tree {
            break;
        }
    }

    let mut down = 0;
    for row in grid.iter().skip(row_no + 1) {
        let tree = row[col_no];
        down += 1;
        if tree >= source_tree {
            break;
        }
    }

    up * left * right * down
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_scenic_score() {
        let grid = parse_grid(SAMPLE);

        assert_eq!(scenic_score(&grid, 1, 2), 4);
        assert_eq!(scenic_score(&grid, 3, 2), 8);
    }

    #[test]
    fn test_part_1_sample() {
        let grid = parse_grid(SAMPLE);

        assert_eq!(count_visible_trees(&grid), 21);
    }

    #[test]
    fn test_part_2_sample() {
        let grid = parse_grid(SAMPLE);

        assert_eq!(highest_scenic_score(&grid), 8);
    }
}
