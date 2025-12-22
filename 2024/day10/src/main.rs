use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::path::Path;

const MOVES: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn main() {
    let input = Path::new("2024/day10/src/input.txt");
    let map = parse_map(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!("Map score: {:?}", calculate_for_map(&map, score_trailhead));
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!("Map rating: {:?}", calculate_for_map(&map, rate_trailhead));
    println!("In {:?}", part2.elapsed());
}

fn parse_map(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => 0,
                    c => c.to_digit(10).unwrap() as usize,
                })
                .collect()
        })
        .collect()
}

fn calculate_for_map<Calc: Fn((usize, usize), &[Vec<usize>]) -> usize>(
    map: &[Vec<usize>],
    func: Calc,
) -> usize {
    map.iter()
        .enumerate()
        .map(|(row_no, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(col_no, height)| {
                    if *height == 0 {
                        Some(func((row_no, col_no), map))
                    } else {
                        None
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

fn rate_trailhead((start_row, start_col): (usize, usize), map: &[Vec<usize>]) -> usize {
    let rows = map.len() as isize;
    let cols = map[0].len() as isize;

    let mut nines_reached = follow_trail((start_row, start_col), map);
    let mut to_visit = VecDeque::from_iter(nines_reached.drain());
    let mut ratings: HashMap<(usize, usize), usize> = HashMap::new();

    while let Some((row, col)) = to_visit.pop_front() {
        if ratings.contains_key(&(row, col)) {
            continue;
        }

        let height = map[row][col];
        if height == 9 {
            ratings.insert((row, col), 1);
        }

        for (next_row, next_col) in next_points(row, col, rows, cols) {
            let next_height = map[next_row][next_col];
            if next_height == height + 1 {
                let next_rating = *ratings.get(&(next_row, next_col)).unwrap_or(&0usize);

                ratings
                    .entry((row, col))
                    .and_modify(|rating| *rating += next_rating)
                    .or_insert(next_rating);
            } else if height > 0 && next_height == height - 1 {
                to_visit.push_back((next_row, next_col));
            }
        }

        if height == 0 && row == start_row && col == start_col {
            break;
        }
    }
    *ratings.get(&(start_row, start_col)).unwrap()
}

fn score_trailhead((start_row, start_col): (usize, usize), map: &[Vec<usize>]) -> usize {
    follow_trail((start_row, start_col), map).len()
}

fn follow_trail(
    (start_row, start_col): (usize, usize),
    map: &[Vec<usize>],
) -> HashSet<(usize, usize)> {
    let rows = map.len() as isize;
    let cols = map[0].len() as isize;

    let mut nines_reached = HashSet::new();
    let mut to_visit = vec![(start_row, start_col)];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    while let Some((row, col)) = to_visit.pop() {
        if visited.contains(&(row, col)) {
            continue;
        }
        visited.insert((row, col));
        let height = map[row][col];
        if height == 9 {
            nines_reached.insert((row, col));
            continue;
        }

        for (next_row, next_col) in next_points(row, col, rows, cols) {
            let next_height = map[next_row][next_col];
            if next_height == height + 1 {
                to_visit.push((next_row, next_col));
            }
        }
    }
    nines_reached
}

fn next_points(
    row: usize,
    col: usize,
    max_rows: isize,
    max_cols: isize,
) -> impl Iterator<Item = (usize, usize)> {
    MOVES.iter().filter_map(move |(row_change, col_change)| {
        let new_row = row as isize + row_change;
        let new_col = col as isize + col_change;
        if new_row < 0 || new_row >= max_rows || new_col < 0 || new_col >= max_cols {
            None
        } else {
            Some((new_row as usize, new_col as usize))
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
    const SAMPLE2: &str = "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9";
    const SAMPLE3: &str = "..90..9
...1.98
...2..7
6543456
765.987
876....
987....";
    const SAMPLE4: &str = "10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01";
    const SAMPLE5: &str = ".....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....";
    const SAMPLE6: &str = "012345
123456
234567
345678
4.6789
56789.";

    #[test]
    fn test_score_trailhead() {
        let map1 = parse_map(SAMPLE2);
        let map2 = parse_map(SAMPLE3);
        let map3 = parse_map(SAMPLE4);

        assert_eq!(score_trailhead((0, 3), &map1), 2);
        assert_eq!(score_trailhead((0, 3), &map2), 4);
        assert_eq!(score_trailhead((0, 1), &map3), 1);
        assert_eq!(score_trailhead((6, 5), &map3), 2);
    }

    #[test]
    fn test_rate_trailhead() {
        let map1 = parse_map(SAMPLE5);
        let map2 = parse_map(SAMPLE3);
        let map3 = parse_map(SAMPLE6);

        assert_eq!(rate_trailhead((0, 5), &map1), 3);
        assert_eq!(rate_trailhead((0, 3), &map2), 13);
        assert_eq!(rate_trailhead((0, 0), &map3), 227);
    }

    #[test]
    fn test_part_1_sample() {
        let map = parse_map(SAMPLE);

        let result = calculate_for_map(&map, score_trailhead);
        assert_eq!(result, 36);
    }

    #[test]
    fn test_part_2_sample() {
        let map = parse_map(SAMPLE);

        let result = calculate_for_map(&map, rate_trailhead);
        assert_eq!(result, 81);
    }
}
