use std::fs;
use std::path::Path;

const SAND_SOURCE_COL: usize = 500;

fn main() {
    let input = Path::new("2022/day14/src/input.txt");
    let mut map = parse_map_offsetted(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Fallen units of sand before they starts flowing into abyss: {:?}",
        fall_sand(&mut map)
    );
    println!("In {:?}", part1.elapsed());

    let mut map = parse_map(&fs::read_to_string(input).unwrap());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Fallen units of sand it blocks sand source: {:?}",
        fall_sand(&mut map)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_map_offsetted(input: &str) -> Vec<Vec<char>> {
    let mut max_row = 0;
    let mut min_row = usize::MIN;
    let mut max_col = 0;
    let mut min_col = usize::MAX;

    let mut lines = Vec::new();

    input.lines().for_each(|line| {
        let points: Vec<(usize, usize)> = line
            .split(" -> ")
            .map(|pair| {
                let (col, row) = pair.split_once(",").unwrap();
                let col = col.parse().unwrap();
                let row = row.parse().unwrap();
                max_row = max_row.max(row);
                min_row = min_row.min(row);
                max_col = max_col.max(col);
                min_col = min_col.min(col);
                (row, col)
            })
            .collect();
        lines.push(points);
    });

    let mut map = vec![vec!['.'; max_col - min_col + 1]; max_row + 1];

    for line in lines {
        for window in line.windows(2) {
            if let &[(row1, col1), (row2, col2)] = window {
                let col1 = col1 - min_col;
                let col2 = col2 - min_col;
                if row1 == row2 {
                    let start = col1.min(col2);
                    let end = col1.max(col2);
                    for col in map[row1].iter_mut().take(end + 1).skip(start) {
                        *col = '#';
                    }
                } else if col1 == col2 {
                    let start = row1.min(row2);
                    let end = row1.max(row2);
                    for row in map.iter_mut().take(end + 1).skip(start) {
                        row[col1] = '#';
                    }
                }
            }
        }
    }
    map[0][SAND_SOURCE_COL - min_col] = '+';
    map
}

fn parse_map(input: &str) -> Vec<Vec<char>> {
    let mut max_row = 0;
    let mut min_row = usize::MIN;

    let mut lines = Vec::new();

    input.lines().for_each(|line| {
        let points: Vec<(usize, usize)> = line
            .split(" -> ")
            .map(|pair| {
                let (col, row) = pair.split_once(",").unwrap();
                let col = col.parse().unwrap();
                let row = row.parse().unwrap();
                max_row = max_row.max(row);
                min_row = min_row.min(row);
                (row, col)
            })
            .collect();
        lines.push(points);
    });

    let mut map = vec![vec!['.'; SAND_SOURCE_COL * 2]; max_row + 2];
    map.push(vec!['#'; SAND_SOURCE_COL]);

    for line in lines {
        for window in line.windows(2) {
            if let &[(row1, col1), (row2, col2)] = window {
                if row1 == row2 {
                    let start = col1.min(col2);
                    let end = col1.max(col2);
                    for col in map[row1].iter_mut().take(end + 1).skip(start) {
                        *col = '#';
                    }
                } else if col1 == col2 {
                    let start = row1.min(row2);
                    let end = row1.max(row2);
                    for row in map.iter_mut().take(end + 1).skip(start) {
                        row[col1] = '#';
                    }
                }
            }
        }
    }
    map[0][SAND_SOURCE_COL] = '+';
    map
}

fn fall_sand(map: &mut [Vec<char>]) -> usize {
    let rows = map.len();
    let cols = map[0].len();
    let start = map[0].iter().position(|c| *c == '+').unwrap();

    for count in 0.. {
        let mut sand = (0, start);

        loop {
            let (row, col) = sand;
            if row + 1 >= rows {
                return count;
            }

            if map[row + 1][col] == '.' {
                sand.0 += 1;
            } else if col == 0 {
                return count;
            } else if map[row + 1][col - 1] == '.' {
                sand = (row + 1, col - 1);
            } else if col + 1 == cols {
                return count;
            } else if map[row + 1][col + 1] == '.' {
                sand = (row + 1, col + 1);
            } else if row == 0 && col == start && map[row][col] == 'O' {
                return count;
            } else {
                map[row][col] = 'O';
                break;
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_part_1_sample() {
        let mut map = parse_map_offsetted(SAMPLE);

        assert_eq!(fall_sand(&mut map), 24);
    }

    #[test]
    fn test_part_2_sample() {
        let mut map = parse_map(SAMPLE);

        assert_eq!(fall_sand(&mut map), 93);
    }
}
