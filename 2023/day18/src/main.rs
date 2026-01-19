use std::collections::{HashSet, VecDeque};
use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl std::str::FromStr for Direction {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" | "3" => Ok(Self::Up),
            "D" | "1" => Ok(Self::Down),
            "L" | "2" => Ok(Self::Left),
            "R" | "0" => Ok(Self::Right),
            _ => unimplemented!(),
        }
    }
}

impl Direction {
    fn step(&self, (row, col): (isize, isize)) -> (isize, isize) {
        match self {
            Self::Up => (row - 1, col),
            Self::Down => (row + 1, col),
            Self::Left => (row, col - 1),
            Self::Right => (row, col + 1),
        }
    }

    fn step_n(&self, (row, col): (isize, isize), n: isize) -> (isize, isize) {
        match self {
            Self::Up => (row - n, col),
            Self::Down => (row + n, col),
            Self::Left => (row, col - n),
            Self::Right => (row, col + n),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Digging {
    direction: Direction,
    length: usize,
}

impl Digging {
    fn from_plan(line: &str) -> Self {
        let (direction, rest) = line.split_once(" ").unwrap();
        let (value, _) = rest.split_once(" ").unwrap();

        Self {
            direction: direction.parse().unwrap(),
            length: value.parse().unwrap(),
        }
    }
}

fn main() {
    let input = Path::new("2023/day18/src/input.txt");
    let contents = fs::read_to_string(input).unwrap();
    let plan = parse_plan(&contents);

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!("Cubic meters of lava laggon can hold: {:?}", dig_it(&plan));
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    let points = parse_points(&contents);
    println!("Part 2");
    println!(
        "Cubic meters of lava lagoon can hold: {:?}",
        calculate_area(&points)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_plan(input: &str) -> Vec<Digging> {
    input.lines().map(Digging::from_plan).collect()
}

fn parse_points(input: &str) -> Vec<(isize, isize)> {
    let mut points = vec![];

    let mut position = (0, 0);
    input.lines().for_each(|line| {
        let (_, hex) = line.rsplit_once(" ").unwrap();
        let hex = &hex[2..(hex.len() - 1)];
        let direction: Direction = hex[(hex.len() - 1)..].parse().unwrap();
        let length = isize::from_str_radix(&hex[..(hex.len() - 1)], 16).unwrap();

        position = direction.step_n(position, length);
        points.push(position);
    });

    points
}

fn dig_it(plan: &[Digging]) -> usize {
    let mut site: HashSet<(isize, isize)> = HashSet::new();
    let mut position = (0, 0);
    site.insert(position);

    for dig in plan {
        for _ in 0..dig.length {
            position = dig.direction.step(position);
            site.insert(position);
        }
    }

    let mut max_rows = 0;
    let mut min_rows = isize::MAX;
    let mut max_cols = 0;
    let mut min_cols = isize::MAX;

    site.iter().for_each(|(row, col)| {
        max_rows = max_rows.max(*row);
        min_rows = min_rows.min(*row);
        max_cols = max_cols.max(*col);
        min_cols = min_cols.min(*col);
    });

    let row_offset = if min_rows < 0 { min_rows.abs() } else { 0 };
    let col_offset = if min_cols < 0 { min_cols.abs() } else { 0 };

    let max_rows = (max_rows + row_offset + 3) as usize;
    let max_cols = (max_cols + col_offset + 3) as usize;

    let mut grid = vec![vec!["."; max_cols]; max_rows];

    for (row, col) in &site {
        let r = (row_offset + row + 1) as usize;
        let c = (col_offset + col + 1) as usize;
        grid[r][c] = "#";
    }

    let mut outside = HashSet::new();

    let mut queue = VecDeque::new();
    queue.push_front((0, 0));
    while let Some((row, col)) = queue.pop_front() {
        if !outside.insert((row, col))
            || site.contains(&(row - row_offset - 1, col - col_offset - 1))
        {
            continue;
        }

        for (row_change, col_change) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let next_row = row_change + row;
            let next_col = col_change + col;

            if next_row < 0
                || next_col < 0
                || next_row >= (max_rows as isize)
                || next_col >= (max_cols as isize)
            {
                continue;
            }

            if !site.contains(&(next_row - row_offset - 1, next_col - col_offset - 1)) {
                queue.push_back((next_row, next_col));
            }
        }
    }

    max_rows * max_cols - outside.len()
}

fn calculate_area(points: &[(isize, isize)]) -> usize {
    let length = points.len() - 1;
    let p0 = [points[length]];
    let polygon: Vec<_> = p0.iter().chain(points.iter()).collect();
    let mut internal_area = 0;
    let mut perimeter_area = 0;
    polygon.windows(2).for_each(|window| {
        internal_area += window[0].1 * window[1].0 - window[0].0 * window[1].1;
        perimeter_area += window[0].0.abs_diff(window[1].0) + window[0].1.abs_diff(window[1].1);
    });
    (internal_area as usize) / 2 + perimeter_area / 2 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "R 6 (#70c710)\nD 5 (#0dc571)\nL 2 (#5713f0)\nD 2 (#d2c081)
R 2 (#59c680)\nD 2 (#411b91)\nL 5 (#8ceee2)\nU 2 (#caa173)\nL 1 (#1b58a2)
U 2 (#caa171)\nR 2 (#7807d2)\nU 3 (#a77fa3)\nL 2 (#015232)\nU 2 (#7a21e3)";

    #[test]
    fn test_part_1_sample() {
        let plan = parse_plan(SAMPLE);

        assert_eq!(dig_it(&plan), 62);
    }

    #[test]
    fn test_part_2_sample() {
        let points = parse_points(SAMPLE);

        assert_eq!(calculate_area(&points), 952408144115);
    }
}
