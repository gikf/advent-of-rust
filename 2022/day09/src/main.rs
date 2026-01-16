use std::collections::HashSet;
use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Right,
    Left,
    Down,
    UpRight,
    DownRight,
    UpLeft,
    DownLeft,
}

impl Direction {
    fn move_by(&self, (row, col): (isize, isize)) -> (isize, isize) {
        match self {
            Self::Up => (row - 1, col),
            Self::Right => (row, col + 1),
            Self::Down => (row + 1, col),
            Self::Left => (row, col - 1),
            Self::UpRight => (row - 1, col + 1),
            Self::UpLeft => (row - 1, col - 1),
            Self::DownRight => (row + 1, col + 1),
            Self::DownLeft => (row + 1, col - 1),
        }
    }

    fn new(a: (isize, isize), b: (isize, isize)) -> Option<Self> {
        let row_diff = a.0.abs_diff(b.0);
        let col_diff = a.1.abs_diff(b.1);
        if row_diff == 2 && col_diff == 2 {
            match (a.0 > b.0, a.1 > b.1) {
                (true, true) => Some(Self::DownRight),
                (true, false) => Some(Self::DownLeft),
                (false, true) => Some(Self::UpRight),
                (false, false) => Some(Self::UpLeft),
            }
        } else if row_diff == 2 {
            if a.0 < b.0 {
                Some(Self::Up)
            } else {
                Some(Self::Down)
            }
        } else if col_diff == 2 {
            if a.1 > b.1 {
                Some(Self::Right)
            } else {
                Some(Self::Left)
            }
        } else {
            None
        }
    }
}

impl std::str::FromStr for Direction {
    type Err = std::io::Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "U" => Ok(Self::Up),
            "R" => Ok(Self::Right),
            "L" => Ok(Self::Left),
            "D" => Ok(Self::Down),
            _ => Err(std::io::Error::other("Unknown direction")),
        }
    }
}

fn main() {
    let input = Path::new("2022/day09/src/input.txt");
    let motions = parse_motions(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!("Positions visited by tail: {:?}", motion_rope(&motions));
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    let knots = 10;
    println!("Part 2");
    println!(
        "Positions visited by tail in 10 knot rope: {:?}",
        motion_rope_with_n_knots(&motions, knots)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_motions(input: &str) -> Vec<(Direction, isize)> {
    input
        .lines()
        .map(|line| {
            let (direction, value) = line.split_once(" ").unwrap();

            (direction.parse().unwrap(), value.parse().unwrap())
        })
        .collect()
}

fn motion_rope(motions: &[(Direction, isize)]) -> usize {
    let mut visited_by_tail = HashSet::new();
    visited_by_tail.insert((0, 0));
    let mut head = (0, 0);
    let mut tail = (0, 0);

    for (direction, length) in motions {
        for _ in 0..*length {
            head = direction.move_by(head);
            let dist = distance(head, tail);
            if (0..=1).contains(&dist) || (dist == 2 && head.0 != tail.0 && head.1 != tail.1) {
                continue;
            }
            match direction {
                Direction::Up => {
                    if head.1 != tail.1 {
                        tail.1 = head.1;
                    }
                }
                Direction::Right => {
                    if head.0 != tail.0 {
                        tail.0 = head.0;
                    }
                }
                Direction::Left => {
                    if head.0 != tail.0 {
                        tail.0 = head.0;
                    }
                }
                Direction::Down => {
                    if head.1 != tail.1 {
                        tail.1 = head.1;
                    }
                }
                _ => {}
            }
            tail = direction.move_by(tail);
            visited_by_tail.insert(tail);
        }
    }
    visited_by_tail.len()
}

fn motion_rope_with_n_knots(motions: &[(Direction, isize)], knots: usize) -> usize {
    let mut visited_by_tail = HashSet::from([(0, 0)]);

    let mut rope = vec![(0, 0); knots];

    for (direction, length) in motions {
        for _ in 0..*length {
            rope[0] = direction.move_by(rope[0]);
            for index in 1..rope.len() {
                let prev = rope[index - 1];
                let current = rope.get_mut(index).unwrap();

                let dist = distance(prev, *current);
                if (0..=1).contains(&dist)
                    || (dist == 2 && prev.0 != current.0 && prev.1 != current.1)
                {
                    continue;
                }

                let direction_to_follow_prev = Direction::new(prev, *current);
                match direction_to_follow_prev {
                    Some(Direction::Up) => {
                        if prev.1 != current.1 {
                            current.1 = prev.1;
                        }
                    }
                    Some(Direction::Right) => {
                        if prev.0 != current.0 {
                            current.0 = prev.0;
                        }
                    }
                    Some(Direction::Left) => {
                        if prev.0 != current.0 {
                            current.0 = prev.0;
                        }
                    }
                    Some(Direction::Down) => {
                        if prev.1 != current.1 {
                            current.1 = prev.1;
                        }
                    }
                    None => {
                        continue;
                    }
                    _ => {}
                }

                if let Some(direction) = direction_to_follow_prev {
                    *current = direction.move_by(*current);
                }
            }
            visited_by_tail.insert(*rope.last().unwrap());
        }
    }
    visited_by_tail.len()
}

fn distance(a: (isize, isize), b: (isize, isize)) -> isize {
    (a.1.abs_diff(b.1) + a.0.abs_diff(b.0)) as isize
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
    const SAMPLE2: &str = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";

    #[test]
    fn test_parse_motions() {
        let motions = parse_motions(SAMPLE);

        assert_eq!(
            motions,
            [
                (Direction::Right, 4),
                (Direction::Up, 4),
                (Direction::Left, 3),
                (Direction::Down, 1),
                (Direction::Right, 4),
                (Direction::Down, 1),
                (Direction::Left, 5),
                (Direction::Right, 2),
            ]
        );
    }

    #[test]
    fn test_part_1_sample() {
        let motions = parse_motions(SAMPLE);

        assert_eq!(motion_rope(&motions), 13);
    }

    #[test]
    fn test_part_2_sample() {
        let motions = parse_motions(SAMPLE);

        assert_eq!(motion_rope_with_n_knots(&motions, 2), 13);
        assert_eq!(motion_rope_with_n_knots(&motions, 10), 1);

        let motions2 = parse_motions(SAMPLE2);
        assert_eq!(motion_rope_with_n_knots(&motions2, 10), 36);
    }
}
