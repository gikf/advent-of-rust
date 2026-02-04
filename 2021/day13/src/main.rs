use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq)]
enum Fold {
    Row(usize),
    Col(usize),
}

impl Fold {
    fn from_fold(fold: &str) -> Self {
        let (row_col, coord) = fold.split_once("=").unwrap();
        match row_col {
            "y" => Self::Row(coord.parse().unwrap()),
            "x" => Self::Col(coord.parse().unwrap()),
            _ => unimplemented!(),
        }
    }
}

fn main() {
    let input = Path::new("2021/day13/src/input.txt");
    let (paper, folds) = parse_manual(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Dots visible after first fold: {:?}",
        count_dots(&fold_paper(&paper, &folds[0]))
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    let folded = do_folds(&paper, &folds);
    println!("Part 2");
    println!("Code to activate infrared camera:");
    print_paper(&folded);
    println!("In {:?}", part2.elapsed());
}

fn parse_manual(input: &str) -> (Vec<Vec<u8>>, Vec<Fold>) {
    let mut lines = input.lines();

    let mut dots = Vec::new();
    let mut max_row = 0;
    let mut max_col = 0;

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let (x, y) = line.split_once(",").unwrap();
        let x: usize = x.parse().unwrap();
        let y: usize = y.parse().unwrap();

        max_row = max_row.max(y);
        max_col = max_col.max(x);

        dots.push((y, x));
    }

    let mut paper = vec![vec![0; max_col + 1]; max_row + 1];

    for (row, col) in dots {
        paper[row][col] = 1;
    }

    let mut folds = Vec::new();

    for line in lines {
        let fold = line.split_ascii_whitespace().last().unwrap();
        folds.push(Fold::from_fold(fold));
    }

    (paper, folds)
}

fn print_paper(grid: &[Vec<u8>]) {
    for row in grid {
        println!(
            "{:?}",
            row.iter()
                .map(|v| if *v == 1 { "#" } else { "." })
                .collect::<String>()
        );
    }
}

fn fold_paper(dots_grid: &[Vec<u8>], fold: &Fold) -> Vec<Vec<u8>> {
    match fold {
        Fold::Row(fold_row) => {
            let mut grid = vec![vec![0; dots_grid[0].len()]; *fold_row];

            for (row_no, row) in dots_grid[..*fold_row].iter().enumerate() {
                for (col_no, value) in row.iter().enumerate() {
                    grid[row_no][col_no] = *value;
                }
            }

            for (row_no, row) in dots_grid[(fold_row + 1)..].iter().rev().enumerate() {
                for (col_no, value) in row.iter().enumerate() {
                    if *value == 1 {
                        grid[row_no][col_no] = 1;
                    }
                }
            }
            grid
        }
        Fold::Col(fold_col) => {
            let mut grid = vec![vec![0; *fold_col]; dots_grid.len()];

            for (row_no, row) in dots_grid.iter().enumerate() {
                for (col_no, value) in row[..*fold_col].iter().enumerate() {
                    grid[row_no][col_no] = *value;
                }
            }

            for (row_no, row) in dots_grid.iter().enumerate() {
                for (col_no, value) in row[(fold_col + 1)..].iter().rev().enumerate() {
                    if *value == 1 {
                        grid[row_no][col_no] = 1;
                    }
                }
            }

            grid
        }
    }
}

fn do_folds(paper: &[Vec<u8>], folds: &[Fold]) -> Vec<Vec<u8>> {
    let mut folded = fold_paper(paper, &folds[0]);
    for fold in folds[1..].iter() {
        folded = fold_paper(&folded, fold)
    }

    folded
}

fn count_dots(paper: &[Vec<u8>]) -> usize {
    paper
        .iter()
        .map(|row| row.iter().filter(|v| **v == 1).count())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "6,10
0,14\n9,10\n0,3\n10,4\n4,11\n6,0\n6,12
4,1\n0,13\n10,12\n3,4\n3,0\n8,4\n1,10\n2,14\n8,10\n9,0

fold along y=7
fold along x=5";

    #[test]
    fn test_part_1_sample() {
        let (paper, folds) = parse_manual(SAMPLE);

        let folded = fold_paper(&paper, &folds[0]);
        assert_eq!(count_dots(&folded), 17);

        let folded2 = fold_paper(&folded, &folds[1]);
        assert_eq!(count_dots(&folded2), 16);
    }

    #[test]
    fn test_part_2_sample() {
        let (paper, folds) = parse_manual(SAMPLE);

        print_paper(&do_folds(&paper, &folds));
    }
}
