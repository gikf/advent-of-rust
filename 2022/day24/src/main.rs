use std::collections::VecDeque;
use std::fs;
use std::path::Path;

enum Field {
    Wall,
    Ground,
}

#[derive(Debug, PartialEq)]
enum Blizzard {
    Up(usize, usize),
    Down(usize, usize),
    Left(usize, usize),
    Right(usize, usize),
}

fn main() {
    let input = Path::new("2022/day24/src/input.txt");
    let (valley, blizzards) = parse_valley(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Fewest number of minutes to reach goal: {:?}",
        fewest_minutes(&valley, &blizzards)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Fewest number of minutes there, back and again: {:?}",
        fewest_minutes_with_snack(&valley, &blizzards)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_valley(input: &str) -> (Vec<Vec<Field>>, Vec<Blizzard>) {
    let mut blizzards = Vec::new();
    let valley = input
        .lines()
        .enumerate()
        .map(|(row_no, line)| {
            line.char_indices()
                .map(|(col_no, c)| match c {
                    '.' => Field::Ground,
                    '#' => Field::Wall,
                    '^' => {
                        blizzards.push(Blizzard::Up(row_no, col_no));
                        Field::Ground
                    }
                    'v' => {
                        blizzards.push(Blizzard::Down(row_no, col_no));
                        Field::Ground
                    }
                    '<' => {
                        blizzards.push(Blizzard::Left(row_no, col_no));
                        Field::Ground
                    }
                    '>' => {
                        blizzards.push(Blizzard::Right(row_no, col_no));
                        Field::Ground
                    }
                    _ => unimplemented!(),
                })
                .collect()
        })
        .collect();
    (valley, blizzards)
}

fn fewest_minutes(valley: &[Vec<Field>], blizzards: &[Blizzard]) -> usize {
    let start = (0, 1);
    let end = (valley.len() - 1, valley[0].len() - 2);
    find_path(valley, blizzards, start, end, 0)
}

fn fewest_minutes_with_snack(valley: &[Vec<Field>], blizzards: &[Blizzard]) -> usize {
    let start = (0, 1);
    let end = (valley.len() - 1, valley[0].len() - 2);

    let there = find_path(valley, blizzards, start, end, 0);
    let back = find_path(valley, blizzards, end, start, there);
    find_path(valley, blizzards, start, end, back)
}

fn find_path(
    valley: &[Vec<Field>],
    blizzards: &[Blizzard],
    start: (usize, usize),
    target: (usize, usize),
    starting_minute: usize,
) -> usize {
    let rows = valley.len();
    let cols = valley[0].len();
    let mut blizzards_in_col = vec![vec![]; cols];
    let mut blizzards_in_row = vec![vec![]; rows];

    for blizzard in blizzards {
        match blizzard {
            Blizzard::Up(row, col) => {
                let coords: Vec<_> = (1..=*row)
                    .rev()
                    .chain(((*row + 1)..(rows - 1)).rev())
                    .map(|row| (row, *col))
                    .collect();
                blizzards_in_col[*col].push(coords);
            }
            Blizzard::Down(row, col) => {
                let coords: Vec<_> = (*row..(rows - 1))
                    .chain(1..*row)
                    .map(|row| (row, *col))
                    .collect();
                blizzards_in_col[*col].push(coords);
            }
            Blizzard::Left(row, col) => {
                let coords: Vec<_> = (1..=*col)
                    .rev()
                    .chain(((*col + 1)..(cols - 1)).rev())
                    .map(|col| (*row, col))
                    .collect();
                blizzards_in_row[*row].push(coords);
            }
            Blizzard::Right(row, col) => {
                let coords: Vec<_> = (*col..(cols - 1))
                    .chain(1..*col)
                    .map(|col| (*row, col))
                    .collect();
                blizzards_in_row[*row].push(coords);
            }
        }
    }

    let mut seen = vec![vec![vec![0; 1_000]; cols]; rows];

    let mut queue = VecDeque::with_capacity(500_000);
    queue.push_back((starting_minute, start));

    while let Some((minutes_so_far, (row, col))) = queue.pop_front() {
        if seen[row][col][minutes_so_far] == 1 {
            continue;
        }
        seen[row][col][minutes_so_far] = 1;

        if (row, col) == target {
            return minutes_so_far;
        }

        let next_minute = minutes_so_far + 1;

        for (row_change, col_change) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let next_row = row as isize + row_change;
            let next_col = col as isize + col_change;

            if next_row < 0 {
                continue;
            }

            let next_row = next_row as usize;
            let next_col = next_col as usize;

            if next_row >= rows {
                continue;
            }

            if matches!(valley[next_row][next_col], Field::Ground)
                && !will_be_blizzard(
                    next_minute,
                    next_row,
                    next_col,
                    &blizzards_in_col[next_col],
                    &blizzards_in_row[next_row],
                )
                && seen[next_row][next_col][next_minute] == 0
            {
                queue.push_back((next_minute, (next_row, next_col)))
            }
        }

        if !will_be_blizzard(
            next_minute,
            row,
            col,
            &blizzards_in_col[col],
            &blizzards_in_row[row],
        ) && seen[row][col][next_minute] == 0
        {
            queue.push_back((next_minute, (row, col)))
        }
    }
    usize::MAX
}

fn will_be_blizzard(
    minute: usize,
    row: usize,
    col: usize,
    blizzards_for_col: &[Vec<(usize, usize)>],
    blizzards_for_row: &[Vec<(usize, usize)>],
) -> bool {
    blizzards_for_col
        .iter()
        .chain(blizzards_for_row.iter())
        .any(|blizzard| blizzard[minute % blizzard.len()] == (row, col))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "#.#####
#.....#
#>....#
#.....#
#...v.#
#.....#
#####.#";
    const SAMPLE2: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    #[test]
    fn test_part_1_sample() {
        let (valley, blizzards) = parse_valley(SAMPLE2);

        assert_eq!(fewest_minutes(&valley, &blizzards), 18);
    }

    #[test]
    fn test_part_2_sample() {
        let (valley, blizzards) = parse_valley(SAMPLE2);

        assert_eq!(fewest_minutes_with_snack(&valley, &blizzards), 54);
    }
}
