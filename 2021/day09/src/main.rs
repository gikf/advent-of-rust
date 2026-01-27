use std::collections::HashSet;
use std::fs;
use std::path::Path;

fn main() {
    let input = Path::new("2021/day09/src/input.txt");
    let map = parse_map(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Sum of risk levels of all low points: {:?}",
        sum_risk_levels(&map)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Sizes of three largest basins multiplied: {:?}",
        n_largest_basin_sizes_multiplied(&map, 3)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_map(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn find_low_points(map: &[Vec<u8>]) -> Vec<(usize, usize)> {
    let rows = map.len() as isize;
    let cols = map[0].len() as isize;
    let mut low_points = Vec::new();
    map.iter().enumerate().for_each(|(row_no, row)| {
        row.iter().enumerate().for_each(|(col_no, point)| {
            for (row_change, col_change) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let next_row_no = row_no as isize + row_change;
                let next_col_no = col_no as isize + col_change;
                if next_row_no < 0 || next_row_no >= rows || next_col_no < 0 || next_col_no >= cols
                {
                    continue;
                } else if map[next_row_no as usize][next_col_no as usize] <= *point {
                    return;
                }
            }
            low_points.push((row_no, col_no));
        })
    });
    low_points
}

fn sum_risk_levels(map: &[Vec<u8>]) -> usize {
    find_low_points(map)
        .iter()
        .map(|(row, col)| (map[*row][*col] + 1) as usize)
        .sum()
}

fn n_largest_basin_sizes_multiplied(map: &[Vec<u8>], n: usize) -> usize {
    let low_points = find_low_points(map);
    let mut basin_sizes: Vec<_> = find_basins(map, low_points)
        .iter()
        .map(|basin| basin.len())
        .collect();
    basin_sizes.sort_by(|a, b| b.cmp(a));

    basin_sizes[..n].iter().product()
}

fn find_basins(map: &[Vec<u8>], low_points: Vec<(usize, usize)>) -> Vec<HashSet<(usize, usize)>> {
    let rows = map.len() as isize;
    let cols = map[0].len() as isize;
    let mut basins = Vec::new();

    for (row, col) in low_points {
        let mut stack = Vec::new();
        stack.push((row, col));
        let mut basin_points = HashSet::new();

        while let Some((row, col)) = stack.pop() {
            if !basin_points.insert((row, col)) {
                continue;
            }
            let height = map[row][col];

            for (row_change, col_change) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let next_row = row as isize + row_change;
                let next_col = col as isize + col_change;
                if next_row < 0 || next_row >= rows || next_col < 0 || next_col >= cols {
                    continue;
                }

                let next_row = next_row as usize;
                let next_col = next_col as usize;

                let next_height = map[next_row][next_col];
                if next_height == 9 || next_height < height + 1 {
                    continue;
                }
                stack.push((next_row, next_col));
            }
        }

        basins.push(basin_points);
    }

    basins
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test_part_1_sample() {
        let map = parse_map(SAMPLE);

        assert_eq!(sum_risk_levels(&map), 15);
    }

    #[test]
    fn test_part_2_sample() {
        let map = parse_map(SAMPLE);

        assert_eq!(n_largest_basin_sizes_multiplied(&map, 3), 1134);
    }
}
