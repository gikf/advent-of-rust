use std::fs;
use std::path::Path;

type Point = (usize, usize);
type Line = (Point, Point);

fn main() {
    let input = Path::new("2021/day05/src/input.txt");
    let lines = parse_lines(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!("Number of points overlapping: {:?}", count_overlaps(&lines));
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Number of points overlappipng, including diagonals: {:?}",
        count_overlaps_with_diagonals(&lines)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_lines(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            let mut points = line.split(" -> ").map(|p| {
                let (x, y) = p.split_once(",").unwrap();
                (y.parse().unwrap(), x.parse().unwrap())
            });
            (points.next().unwrap(), points.next().unwrap())
        })
        .collect()
}

fn count_overlaps(lines: &[Line]) -> usize {
    let max_row = lines.iter().map(|(p1, p2)| p1.0.max(p2.0)).max().unwrap() + 1;
    let max_col = lines.iter().map(|(p1, p2)| p1.1.max(p2.1)).max().unwrap() + 1;

    let mut counts = vec![vec![0; max_col]; max_row];

    for (p1, p2) in lines {
        if p1.0 != p2.0 && p1.1 != p2.1 {
            continue;
        }

        if p1.0 == p2.0 {
            let col_min = p1.1.min(p2.1);
            let col_max = p1.1.max(p2.1);
            for value in counts[p1.0].iter_mut().take(col_max + 1).skip(col_min) {
                *value += 1;
            }
        } else {
            let row_min = p1.0.min(p2.0);
            let row_max = p1.0.max(p2.0);
            for row in counts.iter_mut().skip(row_min).take(row_max + 1 - row_min) {
                row[p1.1] += 1;
            }
        }
    }

    counts
        .iter()
        .map(|row| row.iter().filter(|overlaps| **overlaps > 1).count())
        .sum()
}

fn count_overlaps_with_diagonals(lines: &[Line]) -> usize {
    let max_row = lines.iter().map(|(p1, p2)| p1.0.max(p2.0)).max().unwrap() + 1;
    let max_col = lines.iter().map(|(p1, p2)| p1.1.max(p2.1)).max().unwrap() + 1;

    let mut counts = vec![vec![0; max_col]; max_row];

    for (p1, p2) in lines {
        if p1.0 == p2.0 && p1.1 != p2.1 {
            let col_min = p1.1.min(p2.1);
            let col_max = p1.1.max(p2.1);
            for count in counts[p1.0].iter_mut().take(col_max + 1).skip(col_min) {
                *count += 1;
            }
        } else if p1.1 == p2.1 && p1.0 != p2.0 {
            let row_min = p1.0.min(p2.0);
            let row_max = p1.0.max(p2.0);
            for row in counts.iter_mut().take(row_max + 1).skip(row_min) {
                row[p1.1] += 1;
            }
        } else {
            let rows: Box<dyn Iterator<Item = usize>> = if p1.0 <= p2.0 {
                Box::new(p1.0..=p2.0)
            } else {
                Box::new((p2.0..=p1.0).rev())
            };

            let cols: Box<dyn Iterator<Item = usize>> = if p1.1 <= p2.1 {
                Box::new(p1.1..=p2.1)
            } else {
                Box::new((p2.1..=p1.1).rev())
            };

            for (row, col) in rows.zip(cols) {
                counts[row][col] += 1;
            }
        }
    }

    counts
        .iter()
        .map(|row| row.iter().filter(|overlaps| **overlaps > 1).count())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_part_1_sample() {
        let lines = parse_lines(SAMPLE);

        assert_eq!(count_overlaps(&lines), 5);
    }

    #[test]
    fn test_part_2_sample() {
        let lines = parse_lines(SAMPLE);

        assert_eq!(count_overlaps_with_diagonals(&lines), 12);
    }
}
