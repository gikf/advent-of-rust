use std::collections::HashSet;
use std::fs;
use std::path::Path;

#[derive(Clone, Copy, PartialEq)]
enum Field {
    Wall,
    Box,
    Robot,
    Empty,
}

#[derive(Clone, Copy, PartialEq)]
enum FieldScaled {
    Wall,
    BoxLeft,
    BoxRight,
    Robot,
    Empty,
}

#[derive(Debug, PartialEq)]
enum Move {
    Up,
    Left,
    Down,
    Right,
}

impl std::fmt::Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Self::Wall => '#',
            Self::Box => 'O',
            Self::Robot => '@',
            Self::Empty => '.',
        };
        write!(f, "{}", res)
    }
}

impl std::fmt::Debug for FieldScaled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Self::Wall => '#',
            Self::BoxLeft => '[',
            Self::BoxRight => ']',
            Self::Robot => '@',
            Self::Empty => '.',
        };
        write!(f, "{}", res)
    }
}

impl Move {
    fn move_by(&self) -> (isize, isize) {
        match self {
            Move::Up => (-1, 0),
            Move::Left => (0, -1),
            Move::Down => (1, 0),
            Move::Right => (0, 1),
        }
    }
}

fn main() {
    let input = Path::new("2024/day15/src/input.txt");
    let (mut map, moves) = parse_input(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    move_robot(&mut map, &moves);
    println!("Part 1");
    println!(
        "Sum of all boxes' GPS coordinates, after all moves: {:?}",
        sum_gps_coordinates(&map, box_checker)
    );
    println!("In {:?}", part1.elapsed());

    let (mut map, moves) = parse_input_scaled(&fs::read_to_string(input).unwrap());
    let part2 = std::time::Instant::now();
    move_robot_scaled(&mut map, &moves);
    println!("Part 2");
    println!(
        "Sum of all boxes' GPS coordinates, with scaling, after all moves: {:?}",
        sum_gps_coordinates(&map, scaled_box_checker)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_input(input: &str) -> (Vec<Vec<Field>>, Vec<Move>) {
    let mut lines = input.lines();
    let mut map = Vec::new();

    for line in lines.by_ref().take_while(|line| !line.is_empty()) {
        map.push(
            line.chars()
                .map(|c| match c {
                    '@' => Field::Robot,
                    'O' => Field::Box,
                    '.' => Field::Empty,
                    '#' => Field::Wall,
                    _ => unreachable!(),
                })
                .collect(),
        )
    }
    let mut moves = Vec::new();

    for line in lines {
        line.chars().for_each(|m| {
            moves.push(match m {
                '<' => Move::Left,
                'v' => Move::Down,
                '>' => Move::Right,
                '^' => Move::Up,
                _ => unreachable!(),
            })
        });
    }

    (map, moves)
}

fn parse_input_scaled(input: &str) -> (Vec<Vec<FieldScaled>>, Vec<Move>) {
    let mut lines = input.lines();
    let mut map = Vec::new();

    for line in lines.by_ref().take_while(|line|!line.is_empty()) {
        map.push(
            line.chars()
                .flat_map(|c| match c {
                    '@' => [FieldScaled::Robot, FieldScaled::Empty],
                    'O' => [FieldScaled::BoxLeft, FieldScaled::BoxRight],
                    '.' => [FieldScaled::Empty, FieldScaled::Empty],
                    '#' => [FieldScaled::Wall, FieldScaled::Wall],
                    _ => unreachable!(),
                })
                .collect(),
        )
    }
    let mut moves = Vec::new();

    for line in lines {
        line.chars().for_each(|m| {
            moves.push(match m {
                '<' => Move::Left,
                'v' => Move::Down,
                '>' => Move::Right,
                '^' => Move::Up,
                _ => unreachable!(),
            })
        });
    }

    (map, moves)
}

fn move_robot(map: &mut [Vec<Field>], moves: &[Move]) {
    let mut robot = map
        .iter()
        .enumerate()
        .find_map(|(row_no, row)| {
            row.iter()
                .enumerate()
                .find_map(|(col_no, field)| {
                    if *field == Field::Robot {
                        Some(col_no)
                    } else {
                        None
                    }
                })
                .map(|col_no| (row_no, col_no))
        })
        .unwrap();

    for cur_move in moves {
        let (row_change, col_change) = cur_move.move_by();

        let mut to_move = Vec::new();
        for offset in 1.. {
            let next_row = (offset * row_change + robot.0 as isize) as usize;
            let next_col = (offset * col_change + robot.1 as isize) as usize;

            let next_field = &map[next_row][next_col];
            match next_field {
                Field::Box => {
                    to_move.push((next_row, next_col));
                }
                Field::Empty => {
                    to_move.push((next_row, next_col));
                    break;
                }
                Field::Wall | Field::Robot => {
                    break;
                }
            }
        }

        if to_move.is_empty() {
            continue;
        }
        let (last_row, last_col) = to_move[to_move.len() - 1];
        if map[last_row][last_col] != Field::Empty {
            continue;
        }

        map[robot.0][robot.1] = Field::Empty;
        robot = (
            (row_change + robot.0 as isize) as usize,
            (col_change + robot.1 as isize) as usize,
        );

        for (row, col) in to_move.iter().rev().skip(1) {
            let target_row = (row_change + *row as isize) as usize;
            let target_col = (col_change + *col as isize) as usize;

            let field = map[*row][*col];
            if field == Field::Empty && map[target_row][target_col] != Field::Empty {
                continue;
            }
            map[target_row][target_col] = field;
            map[*row][*col] = Field::Empty;
        }
    }
}

fn move_robot_scaled(map: &mut [Vec<FieldScaled>], moves: &[Move]) {
    let mut robot = map
        .iter()
        .enumerate()
        .find_map(|(row_no, row)| {
            row.iter()
                .enumerate()
                .find_map(|(col_no, field)| {
                    if *field == FieldScaled::Robot {
                        Some(col_no)
                    } else {
                        None
                    }
                })
                .map(|col_no| (row_no, col_no))
        })
        .unwrap();

    for cur_move in moves {
        let (row_change, col_change) = cur_move.move_by();
        match cur_move {
            Move::Down | Move::Up => {
                let mut to_move = vec![HashSet::from([(robot.0, robot.1)])];

                'outer: loop {
                    let last_level = to_move.pop().unwrap();
                    let mut next_level_coords = HashSet::new();
                    let mut next_level_fields = vec![];
                    for (row, col) in &last_level {
                        if map[*row][*col] == FieldScaled::Empty {
                            continue;
                        }
                        let next_row = (row_change + *row as isize) as usize;
                        let next_col = (col_change + *col as isize) as usize;

                        let next_field = &map[next_row][next_col];

                        match next_field {
                            FieldScaled::Empty => {
                                let _ = &next_level_coords.insert((next_row, next_col));
                                let _ = &next_level_fields.push(FieldScaled::Empty);
                            }
                            FieldScaled::BoxLeft => {
                                let _ = &next_level_coords.insert((next_row, next_col));
                                let _ = &next_level_coords.insert((next_row, next_col + 1));
                                let _ = &next_level_fields.push(FieldScaled::BoxLeft);
                                let _ = &next_level_fields.push(FieldScaled::BoxRight);
                            }
                            FieldScaled::BoxRight => {
                                let _ = &next_level_coords.insert((next_row, next_col - 1));
                                let _ = &next_level_coords.insert((next_row, next_col));
                                let _ = &next_level_fields.push(FieldScaled::BoxLeft);
                                let _ = &next_level_fields.push(FieldScaled::BoxRight);
                            }
                            FieldScaled::Robot => {}
                            FieldScaled::Wall => {
                                let _ = &next_level_coords.insert((next_row, next_col));
                                let _ = &next_level_fields.push(FieldScaled::Wall);
                            }
                        }
                    }
                    to_move.push(last_level);
                    if next_level_fields
                        .iter()
                        .any(|f| matches!(*f, FieldScaled::Wall | FieldScaled::Robot))
                        || next_level_fields.iter().all(|f| *f == FieldScaled::Empty)
                    {
                        to_move.push(next_level_coords);
                        break 'outer;
                    }
                    to_move.push(next_level_coords);
                }

                if to_move[to_move.len() - 1]
                    .iter()
                    .any(|(row, col)| map[*row][*col] != FieldScaled::Empty)
                {
                    continue;
                }

                for level in to_move.iter().rev().skip(1) {
                    for (row, col) in level {
                        let target_row = (row_change + *row as isize) as usize;
                        let target_col = (col_change + *col as isize) as usize;

                        let field = map[*row][*col];
                        if field == FieldScaled::Empty
                            && map[target_row][target_col] != FieldScaled::Empty
                        {
                            continue;
                        }
                        map[target_row][target_col] = field;
                        map[*row][*col] = FieldScaled::Empty;
                    }
                }
                robot = (
                    (robot.0 as isize + row_change) as usize,
                    (robot.1 as isize + col_change) as usize,
                );

            }
            Move::Left | Move::Right => {
                let mut to_push = Vec::new();
                for offset in 1.. {
                    let next_row = (offset * row_change + robot.0 as isize) as usize;
                    let next_col = (offset * col_change + robot.1 as isize) as usize;

                    let next_field = &map[next_row][next_col];
                    match next_field {
                        FieldScaled::BoxLeft => {
                            to_push.push((next_row, next_col));
                        }
                        FieldScaled::BoxRight => {
                            to_push.push((next_row, next_col));
                        }
                        FieldScaled::Empty => {
                            to_push.push((next_row, next_col));
                            break;
                        }
                        FieldScaled::Wall | FieldScaled::Robot => {
                            break;
                        }
                    }
                }

                if to_push.is_empty() {
                    continue;
                }
                let (last_row, last_col) = to_push[to_push.len() - 1];
                if map[last_row][last_col] != FieldScaled::Empty {
                    continue;
                }
                map[robot.0][robot.1] = FieldScaled::Empty;
                robot = (
                    (row_change + robot.0 as isize) as usize,
                    (col_change + robot.1 as isize) as usize,
                );
                let mut prev = (robot.0, robot.1, FieldScaled::Robot);

                for (next_row, next_col) in to_push {
                    if prev.2 == FieldScaled::Empty {
                        break;
                    }
                    let on_field = map[next_row][next_col];
                    map[next_row][next_col] = prev.2;

                    prev = (next_row, next_col, on_field);
                }
            }
        }
    }
}

fn sum_gps_coordinates<T, BoxChecker: Fn(&T) -> bool>(map: &[Vec<T>], is_box_func: BoxChecker) -> usize {
    map.iter()
        .enumerate()
        .map(|(row_no, row)| {
            row.iter()
                .enumerate()
                .map(|(col_no, field)| {
                    if is_box_func(field) {
                        100 * row_no + col_no
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

fn scaled_box_checker(field: &FieldScaled) -> bool {
    *field == FieldScaled::BoxLeft
}

fn box_checker(field: &Field) -> bool {
    *field == Field::Box
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
    const SAMPLE2: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
    const SAMPLE_ENDING: &str = "########
#....OO#
##.....#
#.....O#
#.#O@..#
#...O..#
#...O..#
########";
    const SAMPLE2_ENDING: &str = "##########
#.O.O.OOO#
#........#
#OO......#
#OO@.....#
#O#.....O#
#O.....OO#
#O.....OO#
#OO....OO#
##########";
    const SAMPLE3: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";
    const SAMPLE2_ENDING2: &str = "####################
##[].......[].[][]##
##[]...........[].##
##[]........[][][]##
##[]......[]....[]##
##..##......[]....##
##..[]............##
##..@......[].[][]##
##......[][]..[]..##
####################";

    #[test]
    fn test_sum_gps_coordinates() {
        let (map1, _) = parse_input(SAMPLE_ENDING);
        let (map2, _) = parse_input(SAMPLE2_ENDING);

        assert_eq!(sum_gps_coordinates(&map1, box_checker), 2028);
        assert_eq!(sum_gps_coordinates(&map2, box_checker), 10092);
    }

    #[test]
    fn test_part_1_sample() {
        let (mut map1, moves1) = parse_input(SAMPLE);
        let (mut map2, moves2) = parse_input(SAMPLE2);

        move_robot(&mut map1, &moves1);
        assert_eq!(sum_gps_coordinates(&map1, box_checker), 2028);

        move_robot(&mut map2, &moves2);
        assert_eq!(sum_gps_coordinates(&map2, box_checker), 10092);
    }

    #[test]
    fn test_part_2_sample() {
        let (mut map, moves) = parse_input_scaled(SAMPLE2);
        move_robot_scaled(&mut map, &moves);

        assert_eq!(sum_gps_coordinates(&map, scaled_box_checker), 9021);
    }
}
