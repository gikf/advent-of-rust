use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Clone, PartialEq)]
enum Field {
    RoundedRock,
    CubeShapedRock,
    Empty,
}

impl From<char> for Field {
    fn from(value: char) -> Self {
        match value {
            'O' => Self::RoundedRock,
            '#' => Self::CubeShapedRock,
            '.' => Self::Empty,
            _ => unimplemented!(),
        }
    }
}

impl std::fmt::Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            Field::RoundedRock => 'O',
            Field::CubeShapedRock => '#',
            Field::Empty => '.',
        };
        write!(f, "{:?}", ch)
    }
}

fn main() {
    let input = Path::new("2023/day14/src/input.txt");
    let platform = parse_platform(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    let mut part1_platform = platform.clone();
    println!("Part 1");
    println!(
        "Total load on north support beams: {:?}",
        total_load(&mut part1_platform)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    let mut platform = platform;
    let cycles = 1_000_000_000;
    println!("Part 2");
    println!(
        "Total load on north support beams, after 1000000000 spin cycles: {:?}",
        total_load_after_cycles(&mut platform, cycles)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_platform(input: &str) -> Vec<Vec<Field>> {
    input
        .lines()
        .map(|line| line.chars().map(Field::from).collect())
        .collect()
}

#[allow(unused)]
fn draw_state(rounded_rocks: &[(u8, u8)], cube_rocks: &[(u8, u8)], rows: usize, cols: usize) {
    let mut state = vec![vec![Field::Empty; cols]; rows];

    for (row, col) in rounded_rocks {
        state[*row as usize][*col as usize] = Field::RoundedRock;
    }

    for (row, col) in cube_rocks {
        if !matches!(state[*row as usize][*col as usize], Field::Empty) {
            println!("overwritten coordinate {:?} {:?}", row, col);
        }
        state[*row as usize][*col as usize] = Field::CubeShapedRock;
    }

    println!();
    for row in state {
        println!("{:?}", row);
    }
}

fn tilt_platform_up(rounded_rocks: &mut Vec<(u8, u8)>, cube_rocks: &[(u8, u8)], cols: u8) {
    let movable_rocks = rounded_rocks.len();
    let mut closest: HashMap<i8, Vec<(u8, u8)>> = HashMap::with_capacity(cols as usize);
    for col_no in 0..cols {
        let stops_in_column: Vec<_> = cube_rocks.iter().filter(|(_, c)| *c == col_no).collect();
        for rock in rounded_rocks.extract_if(.., |(_, c)| *c == col_no) {
            let stops_above: Vec<_> = stops_in_column
                .iter()
                .filter(|(r, _)| *r < rock.0)
                .collect();
            let entry = if stops_above.is_empty() {
                -1
            } else {
                stops_above
                    .iter()
                    .map(|(r, _)| (rock.0 - *r, *r as i8))
                    .min()
                    .unwrap()
                    .1
            };
            closest
                .entry(entry)
                .and_modify(|rocks| rocks.push(rock))
                .or_insert_with(|| {
                    let mut v = Vec::with_capacity(movable_rocks);
                    v.push(rock);
                    v
                });
        }

        for (row_no, mut rocks_stopped) in closest.drain() {
            let start = (row_no + 1) as u8;
            let end = start + (rocks_stopped.len() as u8);

            for new_row in start..end {
                let (_, c) = rocks_stopped.pop().unwrap();
                rounded_rocks.push((new_row, c));
            }
        }
    }
}

fn tilt_platform_down(
    rounded_rocks: &mut Vec<(u8, u8)>,
    cube_rocks: &[(u8, u8)],
    rows: u8,
    cols: u8,
) {
    let movable_rocks = rounded_rocks.len();
    let mut closest: HashMap<u8, Vec<(u8, u8)>> = HashMap::with_capacity(cols as usize);
    for col_no in 0..cols {
        let stops_in_column: Vec<_> = cube_rocks.iter().filter(|(_, c)| *c == col_no).collect();
        let affected_rocks = rounded_rocks.extract_if(.., |(_, c)| *c == col_no);
        for rock in affected_rocks {
            let stops_below: Vec<_> = stops_in_column
                .iter()
                .filter(|(r, _)| *r > rock.0)
                .collect();
            let entry = if stops_below.is_empty() {
                rows
            } else {
                stops_below
                    .iter()
                    .map(|(r, _)| (*r - rock.0, *r))
                    .min()
                    .unwrap()
                    .1
            };
            closest
                .entry(entry)
                .and_modify(|rocks| rocks.push(rock))
                .or_insert_with(|| {
                    let mut v = Vec::with_capacity(movable_rocks);
                    v.push(rock);
                    v
                });
        }

        for (row_no, mut rocks_stopped) in closest.drain() {
            let end = row_no;
            let start = end - (rocks_stopped.len() as u8);

            for new_row in start..end {
                let (_, c) = rocks_stopped.pop().unwrap();
                rounded_rocks.push((new_row, c));
            }
        }
    }
}

fn tilt_platform_left(rounded_rocks: &mut Vec<(u8, u8)>, cube_rocks: &[(u8, u8)], rows: u8) {
    let movable_rocks = rounded_rocks.len();
    let mut closest: HashMap<i8, Vec<(u8, u8)>> = HashMap::with_capacity(rows as usize);
    for row_no in 0..rows {
        let stops_in_row: Vec<_> = cube_rocks.iter().filter(|(r, _)| *r == row_no).collect();
        let affected_rocks = rounded_rocks.extract_if(.., |(r, _)| *r == row_no);
        for rock in affected_rocks {
            let stops_on_left: Vec<_> = stops_in_row.iter().filter(|(_, c)| *c < rock.1).collect();
            let entry = if stops_on_left.is_empty() {
                -1
            } else {
                stops_on_left
                    .iter()
                    .map(|(_, c)| (rock.1 - *c, *c as i8))
                    .min()
                    .unwrap()
                    .1
            };
            closest
                .entry(entry)
                .and_modify(|rocks| rocks.push(rock))
                .or_insert_with(|| {
                    let mut v = Vec::with_capacity(movable_rocks);
                    v.push(rock);
                    v
                });
        }

        for (col_no, mut rocks_stopped) in closest.drain() {
            let start = (col_no + 1) as u8;
            let end = start + (rocks_stopped.len() as u8);

            for new_col in start..end {
                let (r, _) = rocks_stopped.pop().unwrap();
                rounded_rocks.push((r, new_col));
            }
        }
    }
}

fn tilt_platform_right(
    rounded_rocks: &mut Vec<(u8, u8)>,
    cube_rocks: &[(u8, u8)],
    rows: u8,
    cols: u8,
) {
    let movable_rocks = rounded_rocks.len();
    let mut closest: HashMap<u8, Vec<(u8, u8)>> = HashMap::with_capacity(rows as usize);
    for row_no in 0..rows {
        let stops_in_row: Vec<_> = cube_rocks.iter().filter(|(r, _)| *r == row_no).collect();
        let affected_rocks = rounded_rocks.extract_if(.., |(r, _)| *r == row_no);
        for rock in affected_rocks {
            let stops_on_right: Vec<_> = stops_in_row.iter().filter(|(_, c)| *c > rock.1).collect();
            let entry = if stops_on_right.is_empty() {
                cols
            } else {
                stops_on_right
                    .iter()
                    .map(|(_, c)| (*c - rock.1, *c))
                    .min()
                    .unwrap()
                    .1
            };
            closest
                .entry(entry)
                .and_modify(|rocks| rocks.push(rock))
                .or_insert_with(|| {
                    let mut v = Vec::with_capacity(movable_rocks);
                    v.push(rock);
                    v
                });
        }

        for (col_no, mut rocks_stopped) in closest.drain() {
            let end = col_no;
            let start = end - (rocks_stopped.len() as u8);

            for new_col in start..end {
                let (r, _) = rocks_stopped.pop().unwrap();
                rounded_rocks.push((r, new_col));
            }
        }
    }
}

fn tilt_up(platform: &mut [Vec<Field>]) {
    for col_no in 0..(platform[0].len()) {
        let mut stop_at = 0;
        let mut rounded_rocks = 0;
        let mut to_tilt = Vec::new();
        for (row_no, row) in platform
            .iter()
            .chain([vec![Field::CubeShapedRock; platform[0].len()]].iter())
            .enumerate()
        {
            let field = &row[col_no];

            if matches!(field, Field::RoundedRock) {
                rounded_rocks += 1;
            } else if matches!(field, Field::CubeShapedRock) {
                if rounded_rocks > 0 {
                    to_tilt.push((stop_at, row_no, rounded_rocks));
                }
                stop_at = row_no + 1;
                rounded_rocks = 0;
            }
        }

        for (stop_at, end, mut rounded_rocks) in to_tilt {
            for row in platform.iter_mut().take(end).skip(stop_at) {
                if rounded_rocks > 0 {
                    row[col_no] = Field::RoundedRock;
                    rounded_rocks -= 1;
                } else {
                    row[col_no] = Field::Empty;
                }
            }
        }
    }
}

fn total_load(platform: &mut [Vec<Field>]) -> usize {
    tilt_up(platform);

    (0..=platform.len())
        .rev()
        .zip(platform.iter())
        .map(|(row_no, row)| {
            row.iter()
                .filter(|field| matches!(field, Field::RoundedRock))
                .count()
                * row_no
        })
        .sum()
}

fn total_load_after_cycles(platform: &mut [Vec<Field>], cycles: usize) -> usize {
    let mut rounded_rocks = Vec::new();
    let mut cube_rocks = Vec::new();

    let rows = platform.len() as u8;
    let cols = platform[0].len() as u8;

    platform.iter().enumerate().for_each(|(row_no, row)| {
        row.iter()
            .enumerate()
            .for_each(|(col_no, field)| match field {
                Field::CubeShapedRock => cube_rocks.push((row_no as u8, col_no as u8)),
                Field::RoundedRock => rounded_rocks.push((row_no as u8, col_no as u8)),
                Field::Empty => {}
            })
    });
    let mut unique = HashMap::new();
    let mut states = Vec::new();

    for i in 0..cycles {
        tilt_platform_up(&mut rounded_rocks, &cube_rocks, cols);
        tilt_platform_left(&mut rounded_rocks, &cube_rocks, rows);
        tilt_platform_down(&mut rounded_rocks, &cube_rocks, rows, cols);
        tilt_platform_right(&mut rounded_rocks, &cube_rocks, rows, cols);
        let mut current = rounded_rocks.clone();
        current.sort();
        if unique.contains_key(&current) {
            let rest = cycles - i;
            let loop_start = unique.get(&current).unwrap();
            let loop_end = i;
            let loop_results: &[Vec<_>] = &states[*loop_start..loop_end];

            rounded_rocks = loop_results[(rest % loop_results.len()) - 1].clone();
            break;
        }
        states.push(current.clone());
        unique.insert(current, i);
    }
    rounded_rocks.iter().map(|(r, _)| (rows - r) as usize).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_total_load() {
        let mut platform = parse_platform(SAMPLE);

        assert_eq!(total_load(&mut platform), 136);
    }

    #[test]
    fn test_part_2_sample() {
        let mut platform = parse_platform(SAMPLE);

        let cycles = 1_000_000_000;

        assert_eq!(total_load_after_cycles(&mut platform, cycles), 64);
    }
}
