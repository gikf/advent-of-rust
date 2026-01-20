use std::collections::VecDeque;
use std::fs;
use std::path::Path;

fn main() {
    let input = Path::new("2022/day12/src/input.txt");
    let map = parse_map(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Fewest steps to location with best signal: {:?}",
        shortest_path(&map)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Fewest steps from end to location with height 'a': {:?}",
        shortest_path_from_end_to_lowest(&map)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_map(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_coords(map: &[Vec<char>], target: &char) -> (usize, usize) {
    map.iter()
        .enumerate()
        .find_map(|(row_no, row)| {
            row.iter().enumerate().find_map(|(col_no, c)| {
                if c == target {
                    Some((row_no, col_no))
                } else {
                    None
                }
            })
        })
        .unwrap()
}

fn shortest_path(map: &[Vec<char>]) -> usize {
    let mut steps = vec![vec![0; map[0].len()]; map.len()];
    let start = find_coords(map, &'S');
    let end = find_coords(map, &'E');
    let max_rows = map.len();
    let max_cols = map[0].len();
    let mut visited = vec![0; max_rows * max_cols];

    let mut queue = VecDeque::new();
    queue.push_front((0, start));

    while let Some((step, (row, col))) = queue.pop_front() {
        let visited_index = row * max_cols + col;
        if visited[visited_index] == 1 {
            continue;
        }

        visited[visited_index] = 1;
        steps[row][col] = step;

        if row == end.0 && col == end.1 {
            break;
        }

        let current = map[row][col];
        let height = get_height(current);

        for (row_change, col_change) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let next_row = isize::try_from(row).unwrap() + row_change;
            let next_col = isize::try_from(col).unwrap() + col_change;

            if next_row < 0
                || next_row >= isize::try_from(max_rows).unwrap()
                || next_col < 0
                || next_col >= isize::try_from(max_cols).unwrap()
            {
                continue;
            }

            let next_row = usize::try_from(next_row).unwrap();
            let next_col = usize::try_from(next_col).unwrap();
            let next = map[next_row][next_col];
            let next_height = get_height(next);
            let next_index = next_row * max_cols + next_col;

            if (height >= next_height || height + 1 == next_height) && visited[next_index] == 0 {
                queue.push_back((step + 1, (next_row, next_col)));
            }
        }
    }

    steps[end.0][end.1]
}

fn shortest_path_from_end_to_lowest(map: &[Vec<char>]) -> usize {
    let mut steps = vec![vec![0; map[0].len()]; map.len()];

    let start = find_coords(map, &'E');
    let max_rows = map.len();
    let max_cols = map[0].len();
    let mut visited = vec![0; max_rows * max_cols];

    let mut queue = VecDeque::new();
    queue.push_front((0, start));

    while let Some((step, (row, col))) = queue.pop_front() {
        let visited_index = row * max_cols + col;
        if visited[visited_index] == 1 {
            continue;
        }

        visited[visited_index] = 1;
        steps[row][col] = step;

        let current = map[row][col];
        let height = get_height(current);

        if height == 'a'.to_ascii_lowercase() as u8 {
            return step;
        }

        for (row_change, col_change) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let next_row = isize::try_from(row).unwrap() + row_change;
            let next_col = isize::try_from(col).unwrap() + col_change;

            if next_row < 0
                || next_row >= isize::try_from(max_rows).unwrap()
                || next_col < 0
                || next_col >= isize::try_from(max_cols).unwrap()
            {
                continue;
            }

            let next_row = usize::try_from(next_row).unwrap();
            let next_col = usize::try_from(next_col).unwrap();
            let next = map[next_row][next_col];
            let next_height = get_height(next);
            let next_visited_index = next_row * max_cols + next_col;

            if (next_height >= height || next_height + 1 == height)
                && visited[next_visited_index] == 0
            {
                queue.push_back((step + 1, (next_row, next_col)));
            }
        }
    }

    0
}

fn get_height(c: char) -> u8 {
    match c {
        'S' => 'a'.to_ascii_lowercase() as u8,
        'E' => 'z'.to_ascii_lowercase() as u8,
        c => c.to_ascii_lowercase() as u8,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_part_1_sample() {
        let map = parse_map(SAMPLE);

        assert_eq!(shortest_path(&map), 31);
    }

    #[test]
    fn test_part_2_sample() {
        let map = parse_map(SAMPLE);

        assert_eq!(shortest_path_from_end_to_lowest(&map), 29);
    }
}
