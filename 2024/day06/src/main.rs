use std::collections::HashSet;
use std::fs;
use std::path::Path;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn to_int(&self) -> usize {
        match self {
            Self::Up => 0,
            Self::Right => 1,
            Self::Down => 2,
            Self::Left => 3,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Guard {
    direction: Direction,
    row: usize,
    col: usize,
}

#[derive(Clone, Debug, PartialEq)]
enum Field {
    Empty,
    Obstruction,
}

impl Guard {
    fn rotate(&mut self) {
        match self.direction {
            Direction::Up => self.direction = Direction::Right,
            Direction::Right => self.direction = Direction::Down,
            Direction::Down => self.direction = Direction::Left,
            Direction::Left => self.direction = Direction::Up,
        }
    }

    fn move_(&mut self, map: &[Vec<Field>]) -> Option<bool> {
        let rows = map.len();
        let cols = map[0].len();
        if let Some((next_row, next_col)) = self.next_field(rows, cols) {
            match map[next_row][next_col] {
                Field::Empty => {
                    self.row = next_row;
                    self.col = next_col;
                    return Some(false);
                }
                Field::Obstruction => {
                    self.rotate();
                    return Some(true);
                }
            }
        }
        None
    }

    fn next_field(&self, rows: usize, cols: usize) -> Option<(usize, usize)> {
        match self.direction {
            Direction::Up => {
                if self.row == 0 {
                    None
                } else {
                    Some((self.row - 1, self.col))
                }
            }
            Direction::Right => {
                if self.col == cols - 1 {
                    None
                } else {
                    Some((self.row, self.col + 1))
                }
            }
            Direction::Down => {
                if self.row == rows - 1 {
                    None
                } else {
                    Some((self.row + 1, self.col))
                }
            }
            Direction::Left => {
                if self.col == 0 {
                    None
                } else {
                    Some((self.row, self.col - 1))
                }
            }
        }
    }
}

fn main() {
    let input = Path::new("2024/day06/src/input.txt");
    let (guard, map) = parse_map(&fs::read_to_string(input).unwrap());

    let guard_starting_position = (guard.row, guard.col, guard.direction.clone());

    let part2 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Number of distinct positions visited: {:?}",
        observe_guard(guard, &map)
    );
    println!("In {:?}", part2.elapsed());

    let guard = Guard {
        row: guard_starting_position.0,
        col: guard_starting_position.1,
        direction: guard_starting_position.2,
    };

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Number of loops, with one added obstraction: {:?}",
        count_loops(guard, map)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_map(input: &str) -> (Guard, Vec<Vec<Field>>) {
    let mut guard = None;
    let map = input
        .lines()
        .enumerate()
        .map(|(row_no, line)| {
            line.char_indices()
                .map(|(col_no, c)| match c {
                    '.' => Field::Empty,
                    '#' => Field::Obstruction,
                    '^' => {
                        guard = Some(Guard {
                            row: row_no,
                            col: col_no,
                            direction: Direction::Up,
                        });
                        Field::Empty
                    }
                    '>' => {
                        guard = Some(Guard {
                            row: row_no,
                            col: col_no,
                            direction: Direction::Right,
                        });
                        Field::Empty
                    }
                    '<' => {
                        guard = Some(Guard {
                            row: row_no,
                            col: col_no,
                            direction: Direction::Left,
                        });
                        Field::Empty
                    }
                    'v' => {
                        guard = Some(Guard {
                            row: row_no,
                            col: col_no,
                            direction: Direction::Down,
                        });
                        Field::Empty
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    (guard.unwrap(), map)
}

fn count_loops(mut guard: Guard, map: Vec<Vec<Field>>) -> usize {
    let mut visited = HashSet::new();
    let mut unique_points = HashSet::new();
    let mut map = map;

    visited.insert((guard.row, guard.col));

    while guard.move_(&map).is_some() {
        if let Some(position) = guard.next_field(map.len(), map[0].len())
            && map[position.0][position.1] == Field::Empty
            && !visited.contains(&position)
            && will_loop(guard.clone(), &mut map, position)
        {
            unique_points.insert(position);
        }

        visited.insert((guard.row, guard.col));
    }
    unique_points.len()
}

fn will_loop(mut guard: Guard, map: &mut [Vec<Field>], obstruction: (usize, usize)) -> bool {
    let (obstruction_row, obstruction_col) = obstruction;
    let original = map[obstruction_row][obstruction_col].clone();
    map[obstruction_row][obstruction_col] = Field::Obstruction;

    let mut visited = HashSet::new();
    visited.insert((guard.row, guard.col, guard.direction.to_int()));

    guard.rotate();

    while guard.move_(map).is_some() {
        let visited_point = (guard.row, guard.col, guard.direction.to_int());
        if visited.contains(&visited_point) {
            map[obstruction_row][obstruction_col] = original;
            return true;
        }
        visited.insert(visited_point);
    }
    map[obstruction_row][obstruction_col] = original;
    false
}

fn observe_guard(mut guard: Guard, map: &[Vec<Field>]) -> usize {
    let mut visited = HashSet::new();
    let mut counts: Vec<Vec<usize>> = map.iter().map(|r| r.iter().map(|_| 0).collect()).collect();
    let mut rotations = HashSet::new();

    visited.insert((guard.row, guard.col));
    counts[guard.row][guard.col] += 1;

    while let Some(rotated) = guard.move_(map) {
        visited.insert((guard.row, guard.col));
        if rotated {
            rotations.insert((guard.row, guard.col));
        }
    }
    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "....#.....\n.........#\n..........\n..#.......
.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...";

    #[test]
    fn test_part_1_sample() {
        let (guard, map) = parse_map(SAMPLE);
        let result = observe_guard(guard, &map);

        assert_eq!(result, 41);
    }

    #[test]
    fn test_part_2_sample() {
        let (guard, map) = parse_map(SAMPLE);
        let result = count_loops(guard, map);

        assert_eq!(result, 6);
    }
}
