use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq)]
enum Field {
    Wall,
    Empty,
    Start,
    Exit,
}

#[derive(Debug, PartialEq)]
struct Coords {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq)]
struct Maze {
    start: Coords,
    exit: Coords,
    rows: usize,
    cols: usize,
    map: Vec<Vec<Field>>,
}

#[derive(Debug, PartialEq)]
struct Reindeer {
    x: usize,
    y: usize,
    direction: Direction,
    score: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rotate_clockwise(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    fn rotate_counterclockwise(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::West => Self::South,
            Self::South => Self::East,
            Self::East => Self::North,
        }
    }

    fn forward(&self, row: usize, col: usize) -> (usize, usize) {
        match self {
            Self::North => (row - 1, col),
            Self::East => (row, col + 1),
            Self::South => (row + 1, col),
            Self::West => (row, col - 1),
        }
    }

    fn rotate_to_face(
        &self,
        row: usize,
        col: usize,
        target_row: usize,
        target_col: usize,
    ) -> (Self, usize) {
        let right = self.rotate_clockwise();
        let left = self.rotate_counterclockwise();
        if right.forward(row, col) == (target_row, target_col) {
            (right, 1)
        } else if left.forward(row, col) == (target_row, target_col) {
            (left, 1)
        } else {
            let (after_left, left_count) = left.rotate_to_face(row, col, target_row, target_col);
            let (_, right_count) = right.rotate_to_face(row, col, target_row, target_col);

            (after_left, left_count.min(right_count))
        }
    }
}

const MOVES: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

impl From<char> for Field {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            'E' => Self::Exit,
            '#' => Self::Wall,
            'S' => Self::Start,
            _ => unimplemented!(),
        }
    }
}

impl From<Direction> for u8 {
    fn from(val: Direction) -> Self {
        match val {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
        }
    }
}

fn main() {
    let input = Path::new("2024/day16/src/input.txt");
    let maze = parse_maze(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    let (score, _) = find_path(&maze);
    println!("Lowest possible score: {:?}", score);
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    let (_, paths) = find_path(&maze);
    let unique_tiles = unique_tiles_from_paths(maze.start.y, maze.start.x, paths);
    println!("Unique tiles in best paths: {:?}", unique_tiles);
    println!("In {:?}", part2.elapsed());
}

fn parse_maze(input: &str) -> Maze {
    let map: Vec<Vec<Field>> = input
        .lines()
        .map(|line| line.chars().map(Field::from).collect())
        .collect();

    let start = map
        .iter()
        .enumerate()
        .find_map(|(row_no, row)| {
            row.iter().enumerate().find_map(|(col_no, col)| {
                if *col == Field::Start {
                    Some(Coords {
                        y: row_no,
                        x: col_no,
                    })
                } else {
                    None
                }
            })
        })
        .unwrap();
    let exit = map
        .iter()
        .enumerate()
        .find_map(|(row_no, row)| {
            row.iter().enumerate().find_map(|(col_no, col)| {
                if *col == Field::Exit {
                    Some(Coords {
                        y: row_no,
                        x: col_no,
                    })
                } else {
                    None
                }
            })
        })
        .unwrap();

    Maze {
        start,
        exit,
        rows: map.len(),
        cols: map[0].len(),
        map,
    }
}

fn find_path(maze: &Maze) -> (usize, Vec<Vec<u8>>) {
    let mut scores: Vec<Vec<Option<usize>>> = vec![vec![None; maze.map[0].len()]; maze.map.len()];
    let mut best_paths: Vec<Vec<u8>> = vec![];

    let reindeer = Reindeer {
        x: maze.start.x,
        y: maze.start.y,
        direction: Direction::East,
        score: 0,
    };
    let mut queue = Vec::with_capacity(1000);

    queue.push((reindeer, vec![]));

    while let Some((reindeer, path)) = queue.pop() {
        let is_junction = move_coords(reindeer.y, reindeer.x)
            .filter(|(r, c)| maze.map[*r][*c] == Field::Empty)
            .count()
            >= 3;
        let score_is_1000_diff =
            scores[reindeer.y][reindeer.x].is_some_and(|val| val + 1000 == reindeer.score);
        let score_prev_is_better =
            scores[reindeer.y][reindeer.x].is_some_and(|val| val < reindeer.score);

        if !(is_junction && score_is_1000_diff) && score_prev_is_better {
            continue;
        }

        if maze.map[reindeer.y][reindeer.x] == Field::Exit {
            let score_is_equal =
                scores[reindeer.y][reindeer.x].is_some_and(|val| val == reindeer.score);
            if score_is_equal {
                best_paths.push(path.clone());
            } else if !is_junction || !score_is_1000_diff {
                best_paths = vec![];
            }
        }

        scores[reindeer.y][reindeer.x] = Some(reindeer.score);

        if maze.map[reindeer.y][reindeer.x] == Field::Exit {
            continue;
        }

        for (row, col) in move_coords(reindeer.y, reindeer.x) {
            let field = &maze.map[row][col];
            if !matches!(*field, Field::Empty | Field::Exit) {
                continue;
            }

            if reindeer.direction.forward(reindeer.y, reindeer.x) == (row, col) {
                let next_reindeer = Reindeer {
                    x: col,
                    y: row,
                    score: reindeer.score + 1,
                    direction: reindeer.direction,
                };
                let mut new_path = path.clone();
                new_path.push(reindeer.direction.into());

                queue.push((next_reindeer, new_path));
            } else {
                let (new_direction, rotations) = reindeer
                    .direction
                    .rotate_to_face(reindeer.y, reindeer.x, row, col);
                let next_reindeer = Reindeer {
                    x: col,
                    y: row,
                    score: reindeer.score + 1 + rotations * 1000,
                    direction: new_direction,
                };
                let mut new_path = path.clone();
                new_path.push(new_direction.into());

                queue.push((next_reindeer, new_path));
            }
        }
    }
    (
        scores[maze.exit.y][maze.exit.x].unwrap() as usize,
        best_paths,
    )
}

fn unique_tiles_from_paths(start_row: usize, start_col: usize, paths: Vec<Vec<u8>>) -> usize {
    let mut tiles = HashSet::new();
    tiles.insert((start_row, start_col));

    for path in paths {
        let mut row = start_row;
        let mut col = start_col;

        for step in path {
            (row, col) = match step {
                0 => (row - 1, col),
                1 => (row, col + 1),
                2 => (row + 1, col),
                3 => (row, col - 1),
                _ => unreachable!(),
            };
            tiles.insert((row, col));
        }
    }

    tiles.len()
}

fn move_coords(row: usize, col: usize) -> impl Iterator<Item = (usize, usize)> {
    MOVES.iter().map(move |(row_change, col_change)| {
        (
            (row as isize + row_change) as usize,
            (col as isize + col_change) as usize,
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
    const SAMPLE2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn test_part_1_sample() {
        let maze1 = parse_maze(SAMPLE);
        let (path_score1, _) = find_path(&maze1);
        assert_eq!(path_score1, 7036);

        let maze2 = parse_maze(SAMPLE2);
        let (path_score2, _) = find_path(&maze2);
        assert_eq!(path_score2, 11048);
    }

    #[test]
    fn test_part_2_sample() {
        let maze1 = parse_maze(SAMPLE);
        let (_, paths1) = find_path(&maze1);
        let unique = unique_tiles_from_paths(maze1.start.y, maze1.start.x, paths1);
        assert_eq!(unique, 45);

        let maze2 = parse_maze(SAMPLE2);
        let (_, paths2) = find_path(&maze2);
        let unique = unique_tiles_from_paths(maze2.start.y, maze1.start.x, paths2);
        assert_eq!(unique, 64);
    }
}
