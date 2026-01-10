use std::fs;
use std::path::Path;

fn main() {
    let input = Path::new("2023/day10/src/input.txt");
    let pipes = parse_pipes(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Steps from start to farthest point in the loop: {:?}",
        count_steps(&pipes)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!("Tiles enclosed in pipes: {:?}", count_enclosed(&pipes));
    println!("In {:?}", part2.elapsed());
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Vertical,
    Horizontal,
    NorthEastBend,
    NorthWestBend,
    SouthWestBend,
    SouthEastBend,
    Ground,
    Start,
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Vertical => '|',
            Self::Horizontal => '-',
            Self::NorthEastBend => 'L',
            Self::NorthWestBend => 'J',
            Self::SouthWestBend => '7',
            Self::SouthEastBend => 'F',
            Self::Ground => '.',
            Self::Start => 'S',
        };
        write!(f, "{:?}", c)
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => Tile::Vertical,
            '-' => Tile::Horizontal,
            'L' => Tile::NorthEastBend,
            'J' => Tile::NorthWestBend,
            '7' => Tile::SouthWestBend,
            'F' => Tile::SouthEastBend,
            '.' => Tile::Ground,
            'S' => Tile::Start,
            _ => unimplemented!(),
        }
    }
}

impl Tile {
    fn connectable(&self) -> Vec<((isize, isize), Vec<Tile>)> {
        match self {
            Self::Vertical => vec![
                (
                    (-1, 0),
                    vec![Self::Vertical, Self::SouthEastBend, Self::SouthWestBend],
                ),
                (
                    (1, 0),
                    vec![Self::Vertical, Self::NorthEastBend, Self::NorthWestBend],
                ),
            ],
            Self::Horizontal => vec![
                (
                    (0, -1),
                    vec![Self::Horizontal, Self::NorthEastBend, Self::SouthEastBend],
                ),
                (
                    (0, 1),
                    vec![Self::Horizontal, Self::NorthWestBend, Self::SouthWestBend],
                ),
            ],
            Self::NorthEastBend => vec![
                (
                    (-1, 0),
                    vec![Self::Vertical, Self::SouthWestBend, Self::SouthEastBend],
                ),
                (
                    (0, 1),
                    vec![Self::Horizontal, Self::NorthWestBend, Self::SouthWestBend],
                ),
            ],
            Self::NorthWestBend => vec![
                (
                    (-1, 0),
                    vec![Self::Vertical, Self::SouthWestBend, Self::SouthEastBend],
                ),
                (
                    (0, -1),
                    vec![Self::Horizontal, Self::NorthEastBend, Self::SouthEastBend],
                ),
            ],
            Self::SouthEastBend => vec![
                (
                    (1, 0),
                    vec![Self::Vertical, Self::NorthWestBend, Self::NorthEastBend],
                ),
                (
                    (0, 1),
                    vec![Self::Horizontal, Self::SouthWestBend, Self::NorthWestBend],
                ),
            ],
            Self::SouthWestBend => vec![
                (
                    (1, 0),
                    vec![Self::Vertical, Self::NorthWestBend, Self::NorthEastBend],
                ),
                (
                    (0, -1),
                    vec![Self::Horizontal, Self::NorthEastBend, Self::SouthEastBend],
                ),
            ],
            Self::Ground => vec![],
            Self::Start => vec![
                (
                    (-1, 0),
                    vec![Self::Vertical, Self::SouthWestBend, Self::SouthEastBend],
                ),
                (
                    (0, 1),
                    vec![Self::Horizontal, Self::NorthWestBend, Self::SouthWestBend],
                ),
                (
                    (1, 0),
                    vec![Self::Vertical, Self::NorthEastBend, Self::NorthWestBend],
                ),
                (
                    (0, -1),
                    vec![Self::Horizontal, Self::NorthEastBend, Self::SouthEastBend],
                ),
            ],
        }
    }
}

fn parse_pipes(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| line.chars().map(Tile::from).collect())
        .collect()
}

fn bfs_pipes(pipes: &[Vec<Tile>]) -> (usize, Vec<u8>) {
    let (row, col) = pipes
        .iter()
        .enumerate()
        .find_map(|(row_no, row)| {
            row.iter().enumerate().find_map(|(col_no, tile)| {
                if *tile == Tile::Start {
                    Some((row_no, col_no))
                } else {
                    None
                }
            })
        })
        .unwrap();
    let max_row = pipes.len();
    let max_col = pipes.first().unwrap().len();

    let mut is_visited: Vec<u8> = vec![0; max_row * max_col];
    let mut queue = Vec::new();
    queue.push((1, (row, col)));

    let mut max_step = 0;

    while let Some((step, (row, col))) = queue.pop() {
        if step > max_step {
            max_step = step;
        }
        let index = row * max_col + col;
        if is_visited[index] == 1 {
            continue;
        }

        is_visited[index] = 1;

        let tile = &pipes[row][col];
        for ((row_change, col_change), can_connect_with) in tile.connectable() {
            let next_row = row as isize + row_change;
            let next_col = col as isize + col_change;
            if next_row < 0
                || next_col < 0
                || next_row >= (max_row as isize)
                || next_col >= (max_col as isize)
            {
                continue;
            }

            let next_row = next_row as usize;
            let next_col = next_col as usize;

            let next_index = next_row * max_col + next_col;
            if is_visited[next_index] == 1 {
                continue;
            }

            let next_tile = &pipes[next_row][next_col];
            if !can_connect_with.contains(next_tile) {
                continue;
            }

            queue.push((step + 1, (next_row, next_col)));
        }
    }
    (max_step / 2, is_visited)
}

fn count_steps(pipes: &[Vec<Tile>]) -> usize {
    let (max_steps, _) = bfs_pipes(pipes);
    max_steps
}

fn count_enclosed(pipes: &[Vec<Tile>]) -> usize {
    let expanded = expand_pipes(pipes);
    let (_, is_visited) = bfs_pipes(&expanded);

    let max_rows = expanded.len();
    let max_cols = expanded[0].len();

    let mut is_outside: Vec<u8> = vec![0; (max_rows as usize) * (max_cols as usize)];

    for (start_row, start_col) in [(0, 0), (max_rows - 1, 0), (max_rows - 1, max_cols - 1)] {
        let mut stack = vec![(start_row, start_col)];

        while let Some((row, col)) = stack.pop() {
            let index = row * max_cols + col;
            if is_outside[index] == 1 || is_visited[index] == 1 {
                continue;
            }
            is_outside[index] = 1;

            let mut moves = Vec::new();
            if row != 0 {
                moves.push((row - 1, col));
            }
            if row != max_rows - 1 {
                moves.push((row + 1, col));
            }
            if col != 0 {
                moves.push((row, col - 1));
            }
            if col != max_cols - 1 {
                moves.push((row, col + 1));
            }

            for (next_row, next_col) in moves {
                let coords = (next_row, next_col);
                let next_index = next_row * max_cols + next_col;
                if is_visited[index] == 1 || is_outside[next_index] == 1 {
                    continue;
                }
                stack.push(coords);
            }
        }
    }

    pipes
        .iter()
        .enumerate()
        .map(|(row_offset, row)| {
            row.iter()
                .enumerate()
                .filter(|(col_offset, _)| {
                    let row_no = row_offset * 3 + 1;
                    let col_no = col_offset * 3 + 1;
                    let index = row_no * max_cols + col_no;
                    is_visited[index] == 0 && is_outside[index] == 0
                })
                .count()
        })
        .sum()
}

fn expand_pipes(pipes: &[Vec<Tile>]) -> Vec<Vec<Tile>> {
    let mut expanded = vec![vec![Tile::Ground; pipes[0].len() * 3]; pipes.len() * 3];

    for (row_no, row) in pipes.iter().enumerate() {
        for (col_no, tile) in row.iter().enumerate() {
            let to_expand = match tile {
                Tile::Vertical => [[Tile::Ground, Tile::Vertical, Tile::Ground]; 3],
                Tile::Horizontal => [[Tile::Ground; 3], [Tile::Horizontal; 3], [Tile::Ground; 3]],
                Tile::NorthEastBend => [
                    [Tile::Ground, Tile::Vertical, Tile::Ground],
                    [Tile::Ground, Tile::NorthEastBend, Tile::Horizontal],
                    [Tile::Ground; 3],
                ],
                Tile::NorthWestBend => [
                    [Tile::Ground, Tile::Vertical, Tile::Ground],
                    [Tile::Horizontal, Tile::NorthWestBend, Tile::Ground],
                    [Tile::Ground; 3],
                ],
                Tile::SouthWestBend => [
                    [Tile::Ground; 3],
                    [Tile::Horizontal, Tile::SouthWestBend, Tile::Ground],
                    [Tile::Ground, Tile::Vertical, Tile::Ground],
                ],
                Tile::SouthEastBend => [
                    [Tile::Ground; 3],
                    [Tile::Ground, Tile::SouthEastBend, Tile::Horizontal],
                    [Tile::Ground, Tile::Vertical, Tile::Ground],
                ],
                Tile::Ground => [[Tile::Ground; 3]; 3],
                Tile::Start => {
                    let mut expanded_start = [
                        [Tile::Ground, Tile::Ground, Tile::Ground],
                        [Tile::Ground, Tile::Start, Tile::Ground],
                        [Tile::Ground, Tile::Ground, Tile::Ground],
                    ];

                    if row_no > 0
                        && matches!(
                            pipes[row_no - 1][col_no],
                            Tile::SouthEastBend | Tile::SouthWestBend | Tile::Vertical
                        )
                    {
                        expanded_start[0][1] = Tile::Vertical;
                    }

                    if let Some(r) = pipes.get(row_no + 1)
                        && let Some(t) = r.get(col_no)
                        && matches!(
                            t,
                            Tile::Vertical | Tile::NorthEastBend | Tile::NorthWestBend
                        )
                    {
                        expanded_start[2][1] = Tile::Vertical;
                    }

                    if col_no > 0
                        && matches!(
                            pipes[row_no][col_no - 1],
                            Tile::Horizontal | Tile::NorthEastBend | Tile::SouthEastBend
                        )
                    {
                        expanded_start[1][0] = Tile::Horizontal;
                    }

                    if let Some(r) = pipes.get(row_no)
                        && let Some(t) = r.get(col_no + 1)
                        && matches!(
                            t,
                            Tile::Horizontal | Tile::NorthWestBend | Tile::SouthWestBend
                        )
                    {
                        expanded_start[1][2] = Tile::Horizontal;
                    }
                    expanded_start
                }
            };
            let row_offset = row_no * 3;
            let col_offset = col_no * 3;
            for (row_expanded_no, row_expanded) in to_expand.iter().enumerate() {
                for (col_expanded_no, tile) in row_expanded.iter().enumerate() {
                    expanded[row_offset + row_expanded_no][col_offset + col_expanded_no] =
                        tile.to_owned();
                }
            }
        }
    }
    expanded
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
    const SAMPLE2: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
    const SAMPLE3: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
    const SAMPLE4: &str = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";
    const SAMPLE5: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
    const SAMPLE6: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn test_parse_pipes() {
        let pipes = parse_pipes(SAMPLE1);

        assert_eq!(
            pipes,
            [
                Vec::from([
                    Tile::Horizontal,
                    Tile::NorthEastBend,
                    Tile::Vertical,
                    Tile::SouthEastBend,
                    Tile::SouthWestBend
                ]),
                Vec::from([
                    Tile::SouthWestBend,
                    Tile::Start,
                    Tile::Horizontal,
                    Tile::SouthWestBend,
                    Tile::Vertical
                ]),
                Vec::from([
                    Tile::NorthEastBend,
                    Tile::Vertical,
                    Tile::SouthWestBend,
                    Tile::Vertical,
                    Tile::Vertical
                ]),
                Vec::from([
                    Tile::Horizontal,
                    Tile::NorthEastBend,
                    Tile::Horizontal,
                    Tile::NorthWestBend,
                    Tile::Vertical
                ]),
                Vec::from([
                    Tile::NorthEastBend,
                    Tile::Vertical,
                    Tile::Horizontal,
                    Tile::NorthWestBend,
                    Tile::SouthEastBend
                ]),
            ]
        );
    }

    #[test]
    fn test_count_steps() {
        let pipes1 = parse_pipes(SAMPLE1);
        assert_eq!(count_steps(&pipes1), 4);
        let pipes2 = parse_pipes(SAMPLE2);
        assert_eq!(count_steps(&pipes2), 8);
    }

    #[test]
    fn test_count_enclosed_tiles() {
        let pipes1 = parse_pipes(SAMPLE3);
        assert_eq!(count_enclosed(&pipes1), 4);
        let pipes2 = parse_pipes(SAMPLE4);
        assert_eq!(count_enclosed(&pipes2), 4);
        let pipes3 = parse_pipes(SAMPLE5);
        assert_eq!(count_enclosed(&pipes3), 8);
        let pipes4 = parse_pipes(SAMPLE6);
        assert_eq!(count_enclosed(&pipes4), 10);
    }
}
