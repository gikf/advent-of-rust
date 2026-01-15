use std::collections::VecDeque;
use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq)]
enum Tile {
    Empty,
    MirrorTopRight,
    MirrorTopLeft,
    HorizontalSplitter,
    VerticalSplitter,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '/' => Self::MirrorTopRight,
            '\\' => Self::MirrorTopLeft,
            '-' => Self::HorizontalSplitter,
            '|' => Self::VerticalSplitter,
            _ => unimplemented!(),
        }
    }
}

fn main() {
    let input = Path::new("2023/day16/src/input.txt");
    let layout = parse_layout(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Number of energized tiles: {:?}",
        energetize_layout(&layout, ((0, 0), (0, 1)))
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Largest number of tiles energized: {:?}",
        largest_number_of_tiles_energized(&layout)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_layout(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| line.chars().map(Tile::from).collect())
        .collect()
}

fn energetize_layout(layout: &[Vec<Tile>], start: ((usize, usize), (isize, isize))) -> usize {
    let rows = layout.len();
    let cols = layout[0].len();

    let mut energetized: Vec<u8> = vec![0; rows * cols];
    let mut visited: Vec<[u8; 4]> = vec![[0; 4]; rows * cols * 4];

    let mut beam_queue: VecDeque<((usize, usize), (isize, isize))> = VecDeque::with_capacity(10);
    beam_queue.push_front(start);

    while let Some(((row, col), (row_change, col_change))) = beam_queue.pop_front() {
        let index = row * cols + col;
        let offset = match (row_change, col_change) {
            (-1, 0) => 0,
            (1, 0) => 1,
            (0, 1) => 2,
            (0, -1) => 3,
            _ => unimplemented!(),
        };
        if visited[index][offset] == 1 {
            continue;
        }
        visited[index][offset] = 1;
        energetized[row * cols + col] = 1;

        let tile = &layout[row][col];

        let beaming = match tile {
            Tile::Empty => vec![(row_change, col_change)],
            Tile::MirrorTopLeft => match (row_change, col_change) {
                (1, 0) => vec![(0, 1)],
                (-1, 0) => vec![(0, -1)],
                (0, 1) => vec![(1, 0)],
                (0, -1) => vec![(-1, 0)],
                _ => unimplemented!(),
            },
            Tile::MirrorTopRight => match (row_change, col_change) {
                (1, 0) => vec![(0, -1)],
                (-1, 0) => vec![(0, 1)],
                (0, 1) => vec![(-1, 0)],
                (0, -1) => vec![(1, 0)],
                _ => unimplemented!(),
            },
            Tile::HorizontalSplitter => match (row_change, col_change) {
                (0, -1) | (0, 1) => vec![(row_change, col_change)],
                _ => vec![(0, -1), (0, 1)],
            },
            Tile::VerticalSplitter => match (row_change, col_change) {
                (1, 0) | (-1, 0) => vec![(row_change, col_change)],
                _ => vec![(-1, 0), (1, 0)],
            },
        };

        for (row_change, col_change) in beaming {
            let next_row = (row as isize) + row_change;
            let next_col = (col as isize) + col_change;

            if next_row < 0 || next_row >= rows as isize {
                continue;
            }
            if next_col < 0 || next_col >= cols as isize {
                continue;
            }
            beam_queue.push_back((
                (next_row as usize, next_col as usize),
                (row_change, col_change),
            ));
        }
    }

    energetized.iter().filter(|v| **v == 1).count()
}

fn largest_number_of_tiles_energized(layout: &[Vec<Tile>]) -> usize {
    let rows = layout.len();
    let cols = layout[0].len();
    let mut highest = 0;
    for (row_no, _) in layout.iter().enumerate() {
        for start in [((row_no, 0), (0, 1)), ((row_no, cols - 1), (0, -1))] {
            let count = energetize_layout(layout, start);
            if count > highest {
                highest = count;
            }
        }
    }

    for col_no in 0..cols {
        for start in [((0, col_no), (1, 0)), ((rows - 1, col_no), (-1, 0))] {
            let count = energetize_layout(layout, start);
            if count > highest {
                highest = count;
            }
        }
    }
    highest
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

    #[test]
    fn test_part_1_sample() {
        let layout = parse_layout(SAMPLE);

        assert_eq!(energetize_layout(&layout, ((0, 0), (0, 1))), 46);
    }

    #[test]
    fn test_part_2_sample() {
        let layout = parse_layout(SAMPLE);

        assert_eq!(largest_number_of_tiles_energized(&layout), 51);
    }
}
