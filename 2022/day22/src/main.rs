use std::fs;
use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Field {
    Empty,
    Wall,
    Wrap,
}

impl From<char> for Field {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '#' => Self::Wall,
            ' ' => Self::Wrap,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Turn {
    R,
    L,
}

#[derive(Debug, PartialEq)]
enum Move {
    Forward(usize),
    Turn(Turn),
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn turn(&self, t: &Turn) -> Self {
        match (self, t) {
            (Self::Up, Turn::L) => Self::Left,
            (Self::Up, Turn::R) => Self::Right,
            (Self::Down, Turn::L) => Self::Right,
            (Self::Down, Turn::R) => Self::Left,
            (Self::Right, Turn::L) => Self::Up,
            (Self::Right, Turn::R) => Self::Down,
            (Self::Left, Turn::L) => Self::Down,
            (Self::Left, Turn::R) => Self::Up,
        }
    }

    fn change(&self) -> (isize, isize) {
        match self {
            Self::Up => (-1, 0),
            Self::Down => (1, 0),
            Self::Right => (0, 1),
            Self::Left => (0, -1),
        }
    }
}

fn main() {
    let input = Path::new("2022/day22/src/input.txt");
    let contents = fs::read_to_string(input).unwrap();
    let (map, moves) = parse_notes(&contents);

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Final password: {:?}",
        follow_notes(&map, &moves, wrapping_part1)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Final password after folding map into a cube: {:?}",
        follow_notes(&map, &moves, wrapping_part2)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_notes(input: &str) -> (Vec<(usize, Vec<Field>)>, Vec<Move>) {
    let mut lines = input.lines();

    let mut map = Vec::new();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let row: Vec<_> = line.chars().map(Field::from).collect();
        let offset = row.iter().take_while(|f| matches!(f, Field::Wrap)).count();

        map.push((offset, row));
    }

    let chars = lines.next().unwrap().chars();
    let mut moves = Vec::new();
    let mut cur_path: Vec<usize> = Vec::new();

    for ch in chars.chain([' ']) {
        if ch.is_numeric() {
            cur_path.push(ch.to_digit(10).unwrap() as usize);
            continue;
        }
        if !cur_path.is_empty() {
            let num: usize = cur_path
                .iter()
                .rev()
                .enumerate()
                .map(|(pow, digit)| *digit * 10_usize.pow(pow as u32))
                .sum();
            moves.push(Move::Forward(num));

            cur_path.clear();
        }

        match ch {
            'L' => {
                moves.push(Move::Turn(Turn::L));
            }
            'R' => {
                moves.push(Move::Turn(Turn::R));
            }
            _ => {}
        }
    }

    (map, moves)
}

fn follow_notes<
    WrapFunction: Fn(&[(usize, Vec<Field>)], usize, usize, Direction) -> (usize, usize, Direction),
>(
    map: &[(usize, Vec<Field>)],
    moves: &[Move],
    wrapping: WrapFunction,
) -> usize {
    let mut pos = (0, map[0].0);
    let mut direction = Direction::Right;

    for m in moves {
        match m {
            Move::Forward(steps) => {
                for _ in 0..*steps {
                    let (row, col) = pos;
                    let (next_row, next_col, next_direction) = wrapping(map, row, col, direction);

                    let next_field = &map[next_row].1[next_col];

                    match next_field {
                        Field::Wall => {
                            break;
                        }
                        Field::Empty => {
                            pos = (next_row, next_col);
                            direction = next_direction;
                        }
                        _ => unreachable!(),
                    }
                }
            }
            Move::Turn(turn) => {
                direction = direction.turn(turn);
            }
        }
    }

    (pos.0 + 1) * 1_000 + (pos.1 + 1) * 4 + direction_to_score(&direction)
}

fn wrapping_part1(
    map: &[(usize, Vec<Field>)],
    row: usize,
    col: usize,
    direction: Direction,
) -> (usize, usize, Direction) {
    let max_row = map.len();

    let (row_change, col_change) = direction.change();
    let mut next_row = (row as isize + row_change).rem_euclid(max_row as isize) as usize;
    let r = &map[next_row];
    let mut max_col = r.1.len();
    let mut next_col = (col as isize + col_change) as usize;

    if max_col < next_col && matches!(direction, Direction::Up | Direction::Down) {
        while max_col < next_col {
            next_row = (next_row as isize + row_change).rem_euclid(max_row as isize) as usize;
            max_col = map[next_row].1.len();
        }
    }

    next_col = next_col.rem_euclid(max_col);
    let mut next_field = &map[next_row].1[next_col];

    if matches!(next_field, Field::Wrap) {
        let mut wrapping_pos = (next_row, next_col);

        loop {
            let (row, col) = wrapping_pos;
            let next_row = (row as isize + row_change).rem_euclid(max_row as isize) as usize;
            let r = &map[next_row];
            let max_col = r.1.len();
            let col_value = col as isize + col_change;
            if matches!(direction, Direction::Up | Direction::Down) && col_value > max_col as isize
            {
                wrapping_pos.0 = next_row;
                continue;
            }
            let next_col = col_value.rem_euclid(max_col as isize) as usize;
            next_field = &map[next_row].1[next_col];
            wrapping_pos = (next_row, next_col);
            if !matches!(next_field, Field::Wrap) {
                break;
            }
        }
        next_row = wrapping_pos.0;
        next_col = wrapping_pos.1;
    }

    (next_row as usize, next_col as usize, direction)
}

#[allow(unused)]
fn wrapping_part2_sample(
    map: &[(usize, Vec<Field>)],
    row: usize,
    col: usize,
    direction: Direction,
) -> (usize, usize, Direction) {
    //    1
    //  234
    //    56
    let size = (map[0].1.len() / 3) as isize;
    let max_row = map.len() as isize;
    let (row_change, col_change) = direction.change();
    let mut next_row = row as isize + row_change;
    let mut next_col = col as isize + col_change;
    let mut next_direction = direction;
    if next_row < 0 {
        if ((2 * size)..(3 * size)).contains(&next_col) {
            next_col = 3 * size - next_col - 1;
            next_row = size;
            next_direction = Direction::Down;
        }
    } else if next_row >= max_row {
        if ((2 * size)..(3 * size)).contains(&next_col) {
            next_col = 3 * size - 1 - next_col;
            next_row = 2 * size - 1;
            next_direction = Direction::Up;
        } else if ((3 * size)..(4 * size)).contains(&next_col) {
            next_row = 4 * size - next_col - 1;
            next_col = 0;
            next_direction = Direction::Right;
        }
    }

    let r = &map[next_row as usize];
    let max_col = r.1.len() as isize;

    if next_col < 0 {
        if (size..(2 * size)).contains(&next_row) {
            next_col = (2 * size - next_row - 1) + 3 * size;
            next_row = 3 * size - 1;
            next_direction = Direction::Up;
        }
    } else if next_col >= max_col {
        if (0..size).contains(&next_row) {
            next_col = 4 * size - 1;
            next_row = size - next_row;
            next_direction = Direction::Left;
        } else if (size..(2 * size)).contains(&next_row) {
            match next_direction {
                Direction::Right => {
                    next_col = (2 * size - next_row - 1) + 3 * size;
                    next_row = 2 * size;
                    next_direction = Direction::Down;
                }
                Direction::Up => {
                    next_row = size + (4 * size - next_col - 1);
                    next_col = 3 * size - 1;
                    next_direction = Direction::Left;
                }
                _ => {}
            }
        } else if ((2 * size)..(3 * size)).contains(&next_row) {
            next_col = 3 * size - 1;
            next_row = 3 * size - next_row - 1;
            next_direction = Direction::Left;
        }
    }

    let next_field = &map[next_row as usize].1[next_col as usize];

    if matches!(next_field, Field::Wrap) {
        if (0..size).contains(&next_col) {
            match next_direction {
                Direction::Up => {
                    next_row = 0;
                    next_direction = Direction::Down;
                }
                Direction::Down => {
                    next_row = 3 * size - 1;
                    next_direction = Direction::Up
                }
                _ => {}
            }
            next_col = 3 * size - 1 - next_col;
        } else if ((size)..(2 * size)).contains(&next_col) {
            match next_direction {
                Direction::Up => {
                    next_row = 2 * size - next_col;
                    next_col = 2 * size;
                    next_direction = Direction::Right;
                }
                Direction::Down => {
                    next_row = 2 * size - next_col - 1 + 2 * size;
                    next_col = 2 * size;
                    next_direction = Direction::Right;
                }
                Direction::Left => {
                    if (0..size).contains(&next_row) {
                        next_col = next_row;
                        next_row = size;
                        next_direction = Direction::Down;
                    } else {
                        next_col = 2 * size - (3 * size - next_row - 1) - 1;
                        next_row = 2 * size - 1;
                        next_direction = Direction::Up;
                    }
                }
                _ => {}
            }
        }
    }
    (next_row as usize, next_col as usize, next_direction)
}

fn wrapping_part2(
    map: &[(usize, Vec<Field>)],
    row: usize,
    col: usize,
    direction: Direction,
) -> (usize, usize, Direction) {
    //   12
    //   3
    //  54
    //  6
    let max_row = map.len() as isize;
    let size = (map[0].1.len() / 3) as isize;
    let (row_change, col_change) = direction.change();
    let mut next_row = row as isize + row_change;
    let mut next_col = col as isize + col_change;
    let mut next_direction = direction;
    if next_row < 0 {
        if ((size)..(2 * size)).contains(&next_col) {
            // 1 U -> 6 L
            next_row = 4 * size - (2 * size - 1 - next_col) - 1;
            next_col = 0;
            next_direction = Direction::Right;
        } else if ((2 * size)..(3 * size)).contains(&next_col) {
            // 2 U -> 6 D
            next_col -= 2 * size;
            next_row = 4 * size - 1;
            next_direction = Direction::Up;
        }
    } else if next_row >= max_row && (0..size).contains(&next_col) {
        // 6 D -> 2 U
        next_col += 2 * size;
        next_row = 0;
        next_direction = Direction::Down;
    }

    let r = &map[next_row as usize];
    let max_col = r.1.len() as isize;

    if next_col < 0 {
        if ((2 * size)..(3 * size)).contains(&next_row) {
            // 5 L -> 1 L
            next_col = size;
            next_row = 3 * size - next_row - 1;
            next_direction = Direction::Right;
        } else if ((3 * size)..(4 * size)).contains(&next_row) {
            // 6 L -> 1 U
            next_col = next_row - 3 * size + size;
            next_row = 0;
            next_direction = Direction::Down;
        }
    } else if next_col >= max_col {
        if (0..size).contains(&next_row) {
            // 2 R -> 4 R
            next_row = 3 * size - 1 - next_row;
            next_col = 2 * size - 1;
            next_direction = Direction::Left;
        } else if (size..(2 * size)).contains(&next_row) {
            match next_direction {
                Direction::Right => {
                    // 3 R -> 2 D
                    next_col = 2 * size + (next_row - size);
                    next_row = size - 1;
                    next_direction = Direction::Up;
                }
                Direction::Down => {
                    // 2 D -> 3 R
                    next_row = next_col - size;
                    next_col = 2 * size - 1;
                    next_direction = Direction::Left;
                }
                _ => {}
            }
        } else if ((2 * size)..(3 * size)).contains(&next_row) {
            // 4 R -> 2 R
            next_row = 3 * size - next_row - 1;
            next_col = 3 * size - 1;
            next_direction = Direction::Left;
        } else if ((3 * size)..(4 * size)).contains(&next_row) {
            match next_direction {
                Direction::Right => {
                    // 6 R -> 4 D
                    next_col = (next_row - 3 * size) + size;
                    next_row = 3 * size - 1;
                    next_direction = Direction::Up;
                }
                Direction::Down => {
                    // 4 D -> 6 R
                    next_row = 3 * size + next_col - size;
                    next_col = size - 1;
                    next_direction = Direction::Left;
                }
                _ => {}
            }
        }
    }

    let next_field = &map[next_row as usize].1[next_col as usize];

    if matches!(next_field, Field::Wrap) {
        if (0..size).contains(&next_row) {
            // 1 L -> 5 L
            next_col = 0;
            next_row = 3 * size - 1 - next_row;
            next_direction = Direction::Right;
        } else if ((size)..(2 * size)).contains(&next_row) {
            match next_direction {
                Direction::Up => {
                    // 5 U -> 3 L
                    next_row = size + next_col;
                    next_col = size;
                    next_direction = Direction::Right;
                }
                Direction::Left => {
                    // 3 L -> 5 U
                    next_col = next_row - size;
                    next_row = 2 * size;
                    next_direction = Direction::Down;
                }
                _ => {}
            }
        }
    }
    (next_row as usize, next_col as usize, next_direction)
}

fn direction_to_score(direction: &Direction) -> usize {
    match direction {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";
    const SAMPLE2: &str = "   ......
   ......
   ......
   ...
   ...
   ...
......
......
......
...
...
...

 ";

    #[test]
    fn test_parse_notes() {
        let (map, path) = parse_notes(SAMPLE);

        let offsets: Vec<_> = map.iter().map(|(o, _)| *o).collect();
        let lines_lengths: Vec<_> = map.iter().map(|(_, l)| l.len()).collect();

        assert_eq!(offsets, [8, 8, 8, 8, 0, 0, 0, 0, 8, 8, 8, 8]);
        assert_eq!(
            lines_lengths,
            [12, 12, 12, 12, 12, 12, 12, 12, 16, 16, 16, 16]
        );

        assert_eq!(
            path,
            [
                Move::Forward(10),
                Move::Turn(Turn::R),
                Move::Forward(5),
                Move::Turn(Turn::L),
                Move::Forward(5),
                Move::Turn(Turn::R),
                Move::Forward(10),
                Move::Turn(Turn::L),
                Move::Forward(4),
                Move::Turn(Turn::R),
                Move::Forward(5),
                Move::Turn(Turn::L),
                Move::Forward(5),
            ]
        )
    }

    #[test]
    fn test_part_1_sample() {
        let (map, moves) = parse_notes(SAMPLE);

        assert_eq!(follow_notes(&map, &moves, wrapping_part1), 6032);
    }

    #[test]
    fn test_part_2_1l_5l() {
        let (map, _) = parse_notes(SAMPLE2);

        assert_eq!(
            follow_notes(
                &map,
                &[Move::Turn(Turn::L), Move::Turn(Turn::L), Move::Forward(1),],
                wrapping_part2
            ),
            (8 + 1) * 1000 + (0 + 1) * 4 + 0
        );

        assert_eq!(
            follow_notes(
                &map,
                &[
                    Move::Turn(Turn::R),
                    Move::Forward(2),
                    Move::Turn(Turn::R),
                    Move::Forward(1),
                ],
                wrapping_part2
            ),
            (6 + 1) * 1000 + (0 + 1) * 4 + 0
        );

        assert_eq!(
            follow_notes(
                &map,
                &[
                    Move::Turn(Turn::L),
                    Move::Turn(Turn::L),
                    Move::Forward(1),
                    Move::Turn(Turn::L),
                    Move::Turn(Turn::L),
                    Move::Forward(1),
                ],
                wrapping_part2
            ),
            (0 + 1) * 1000 + (3 + 1) * 4 + 0
        );

        assert_eq!(
            follow_notes(
                &map,
                &[
                    Move::Turn(Turn::R),
                    Move::Forward(2),
                    Move::Turn(Turn::R),
                    Move::Forward(1),
                    Move::Turn(Turn::L),
                    Move::Turn(Turn::L),
                    Move::Forward(1),
                    Move::Turn(Turn::L),
                    Move::Forward(2),
                ],
                wrapping_part2
            ),
            (0 + 1) * 1000 + (3 + 1) * 4 + 3
        );
    }

    #[test]
    fn test_part_2_1u_6l() {
        let (map, _) = parse_notes(SAMPLE2);

        assert_eq!(
            follow_notes(
                &map,
                &[Move::Turn(Turn::L), Move::Forward(1),],
                wrapping_part2
            ),
            (9 + 1) * 1000 + (0 + 1) * 4 + 0
        );

        assert_eq!(
            follow_notes(
                &map,
                &[Move::Forward(2), Move::Turn(Turn::L), Move::Forward(1),],
                wrapping_part2
            ),
            (11 + 1) * 1000 + (0 + 1) * 4 + 0
        );

        assert_eq!(
            follow_notes(
                &map,
                &[
                    Move::Turn(Turn::L),
                    Move::Forward(1),
                    Move::Turn(Turn::L),
                    Move::Turn(Turn::L),
                    Move::Forward(1),
                ],
                wrapping_part2
            ),
            (0 + 1) * 1000 + (3 + 1) * 4 + 1
        );

        assert_eq!(
            follow_notes(
                &map,
                &[
                    Move::Forward(2),
                    Move::Turn(Turn::L),
                    Move::Forward(1),
                    Move::Turn(Turn::L),
                    Move::Turn(Turn::L),
                    Move::Forward(1),
                    Move::Turn(Turn::R),
                    Move::Forward(2),
                ],
                wrapping_part2
            ),
            (0 + 1) * 1000 + (3 + 1) * 4 + 2
        );
    }

    #[test]
    fn test_part_2_2r_4r() {
        let (map, _) = parse_notes(SAMPLE2);

        assert_eq!(
            follow_notes(&map, &[Move::Forward(6),], wrapping_part2),
            (8 + 1) * 1000 + (5 + 1) * 4 + 2
        );

        assert_eq!(
            follow_notes(
                &map,
                &[
                    Move::Turn(Turn::R),
                    Move::Forward(2),
                    Move::Turn(Turn::L),
                    Move::Forward(6),
                ],
                wrapping_part2
            ),
            (6 + 1) * 1000 + (5 + 1) * 4 + 2
        );

        assert_eq!(
            follow_notes(
                &map,
                &[
                    Move::Forward(6),
                    Move::Turn(Turn::L),
                    Move::Turn(Turn::L),
                    Move::Forward(6),
                ],
                wrapping_part2
            ),
            (0 + 1) * 1000 + (3 + 1) * 4 + 2
        );

        assert_eq!(
            follow_notes(
                &map,
                &[
                    Move::Turn(Turn::R),
                    Move::Forward(2),
                    Move::Turn(Turn::L),
                    Move::Forward(6),
                    Move::Turn(Turn::L),
                    Move::Turn(Turn::L),
                    Move::Forward(6),
                    Move::Turn(Turn::R),
                    Move::Forward(2),
                ],
                wrapping_part2
            ),
            (0 + 1) * 1000 + (3 + 1) * 4 + 3
        );
    }

    #[test]
    fn test_part_2_2u_6d() {
        let (map, _) = parse_notes(SAMPLE2);

        assert_eq!(
            follow_notes(
                &map,
                &[Move::Forward(3), Move::Turn(Turn::L), Move::Forward(1),],
                wrapping_part2
            ),
            (11 + 1) * 1000 + (0 + 1) * 4 + 3
        );

        assert_eq!(
            follow_notes(
                &map,
                &[Move::Forward(5), Move::Turn(Turn::L), Move::Forward(1),],
                wrapping_part2
            ),
            (11 + 1) * 1000 + (2 + 1) * 4 + 3
        );

        assert_eq!(
            follow_notes(
                &map,
                &[
                    Move::Forward(3),
                    Move::Turn(Turn::L),
                    Move::Forward(1),
                    Move::Turn(Turn::L),
                    Move::Turn(Turn::L),
                    Move::Forward(1),
                    Move::Turn(Turn::R),
                    Move::Forward(3)
                ],
                wrapping_part2
            ),
            (0 + 1) * 1000 + (3 + 1) * 4 + 2
        );

        assert_eq!(
            follow_notes(
                &map,
                &[
                    Move::Forward(5),
                    Move::Turn(Turn::L),
                    Move::Forward(1),
                    Move::Turn(Turn::L),
                    Move::Turn(Turn::L),
                    Move::Forward(1),
                    Move::Turn(Turn::R),
                    Move::Forward(5)
                ],
                wrapping_part2
            ),
            (0 + 1) * 1000 + (3 + 1) * 4 + 2
        );
    }

    #[test]
    fn test_part_2_2d_3r() {
        let (map, _) = parse_notes(SAMPLE2);

        assert_eq!(
            follow_notes(
                &map,
                &[Move::Forward(3), Move::Turn(Turn::R), Move::Forward(3)],
                wrapping_part2
            ),
            (3 + 1) * 1000 + (5 + 1) * 4 + 2
        );
        assert_eq!(
            follow_notes(
                &map,
                &[Move::Forward(5), Move::Turn(Turn::R), Move::Forward(3)],
                wrapping_part2
            ),
            (5 + 1) * 1000 + (5 + 1) * 4 + 2
        );
        assert_eq!(
            follow_notes(
                &map,
                &[
                    Move::Forward(3),
                    Move::Turn(Turn::R),
                    Move::Forward(3),
                    Move::Turn(Turn::L),
                    Move::Turn(Turn::L),
                    Move::Forward(3),
                    Move::Turn(Turn::L),
                    Move::Forward(3),
                ],
                wrapping_part2
            ),
            (0 + 1) * 1000 + (3 + 1) * 4 + 2
        );
        assert_eq!(
            follow_notes(
                &map,
                &[
                    Move::Forward(5),
                    Move::Turn(Turn::R),
                    Move::Forward(3),
                    Move::Turn(Turn::L),
                    Move::Turn(Turn::L),
                    Move::Forward(3),
                    Move::Turn(Turn::L),
                    Move::Forward(5),
                ],
                wrapping_part2
            ),
            (0 + 1) * 1000 + (3 + 1) * 4 + 2
        );
    }

    #[test]
    fn test_part_2_3l_5u() {
        let (map, _) = parse_notes(SAMPLE2);

        assert_eq!(
            follow_notes(
                &map,
                &[
                    Move::Turn(Turn::R),
                    Move::Forward(3),
                    Move::Turn(Turn::R),
                    Move::Forward(1)
                ],
                wrapping_part2
            ),
            (6 + 1) * 1000 + (0 + 1) * 4 + 1
        );
        assert_eq!(
            follow_notes(
                &map,
                &[
                    Move::Turn(Turn::R),
                    Move::Forward(5),
                    Move::Turn(Turn::R),
                    Move::Forward(1)
                ],
                wrapping_part2
            ),
            (6 + 1) * 1000 + (2 + 1) * 4 + 1
        );
        assert_eq!(
            follow_notes(
                &map,
                &[
                    Move::Turn(Turn::R),
                    Move::Forward(3),
                    Move::Turn(Turn::R),
                    Move::Forward(1),
                    Move::Turn(Turn::L),
                    Move::Turn(Turn::L),
                    Move::Forward(1),
                    Move::Turn(Turn::L),
                    Move::Forward(3)
                ],
                wrapping_part2
            ),
            (0 + 1) * 1000 + (3 + 1) * 4 + 3
        );
        assert_eq!(
            follow_notes(
                &map,
                &[
                    Move::Turn(Turn::R),
                    Move::Forward(5),
                    Move::Turn(Turn::R),
                    Move::Forward(1),
                    Move::Turn(Turn::L),
                    Move::Turn(Turn::L),
                    Move::Forward(1),
                    Move::Turn(Turn::L),
                    Move::Forward(5)
                ],
                wrapping_part2
            ),
            (0 + 1) * 1000 + (3 + 1) * 4 + 3
        );
    }

    #[test]
    fn test_part_2_4d_6r() {
        let (map, _) = parse_notes(SAMPLE2);

        assert_eq!(
            follow_notes(
                &map,
                &[Move::Turn(Turn::R), Move::Forward(9)],
                wrapping_part2
            ),
            (9 + 1) * 1000 + (2 + 1) * 4 + 2
        );
        assert_eq!(
            follow_notes(
                &map,
                &[Move::Forward(2), Move::Turn(Turn::R), Move::Forward(9)],
                wrapping_part2
            ),
            (11 + 1) * 1000 + (2 + 1) * 4 + 2
        );
        assert_eq!(
            follow_notes(
                &map,
                &[
                    Move::Turn(Turn::R),
                    Move::Forward(10),
                    Move::Turn(Turn::L),
                    Move::Turn(Turn::L),
                    Move::Forward(10)
                ],
                wrapping_part2
            ),
            (0 + 1) * 1000 + (3 + 1) * 4 + 3
        );
        assert_eq!(
            follow_notes(
                &map,
                &[
                    Move::Forward(2),
                    Move::Turn(Turn::R),
                    Move::Forward(10),
                    Move::Turn(Turn::L),
                    Move::Turn(Turn::L),
                    Move::Forward(10),
                    Move::Turn(Turn::L),
                    Move::Forward(2)
                ],
                wrapping_part2
            ),
            (0 + 1) * 1000 + (3 + 1) * 4 + 2
        );
    }

    #[test]
    fn test_part_2_sample() {
        let (map, moves) = parse_notes(SAMPLE);

        assert_eq!(follow_notes(&map, &moves, wrapping_part2_sample), 5031);
    }
}
