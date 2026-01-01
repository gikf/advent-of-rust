use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq)]
enum Field {
    Start,
    End,
    Wall,
    Empty,
}

impl From<char> for Field {
    fn from(value: char) -> Self {
        match value {
            '.' => Field::Empty,
            'S' => Field::Start,
            'E' => Field::End,
            '#' => Field::Wall,
            _ => unimplemented!(),
        }
    }
}

type Coords = (usize, usize);
type Track = Vec<Vec<Field>>;

const MOVES: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn main() {
    let input = Path::new("2024/day20/src/input.txt");
    let (track, start, end) = parse_track(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    let best_path = fastest_path(&track, start, end);
    println!("Part 1");
    println!(
        "Two second cheats saving at least 100 picoseconds: {:?}",
        find_short_cheats(&track, &best_path, 100)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    let max_cheat_length = 20;
    let min_gain = 100;
    println!("Part 2");
    println!(
        "Up to 20 seconds cheats, saving at least 100 picoseconds: {:?}",
        find_cheats(&best_path, max_cheat_length, min_gain)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_track(input: &str) -> (Track, Coords, Coords) {
    let track: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.chars().map(Field::from).collect())
        .collect();
    let start = track
        .iter()
        .enumerate()
        .find_map(|(row_no, row)| {
            row.iter()
                .position(|field| *field == Field::Start)
                .map(|col_no| (row_no, col_no))
        })
        .unwrap();
    let end = track
        .iter()
        .enumerate()
        .find_map(|(row_no, row)| {
            row.iter()
                .position(|field| *field == Field::End)
                .map(|col_no| (row_no, col_no))
        })
        .unwrap();
    (track, start, end)
}

fn fastest_path(track: &[Vec<Field>], start: Coords, end: Coords) -> Vec<Coords> {
    let max_row = track.len() as isize;
    let max_col = track[0].len() as isize;
    let mut queue = VecDeque::new();
    queue.push_front(vec![start]);
    let mut visited: HashSet<Coords> = HashSet::new();

    while let Some(so_far) = queue.pop_back() {
        let last_visited = &so_far[so_far.len() - 1];
        if *last_visited == end {
            return so_far;
        }
        if visited.contains(last_visited) {
            continue;
        }

        visited.insert(*last_visited);

        for (next_row, next_col) in moves(last_visited.0, last_visited.1, max_row, max_col) {
            if matches!(track[next_row][next_col], Field::Empty | Field::End)
                && !visited.contains(&(next_row, next_col))
            {
                let mut new_path = so_far.clone();
                new_path.push((next_row, next_col));
                queue.push_back(new_path);
            }
        }
    }

    Vec::new()
}

fn find_short_cheats(track: &[Vec<Field>], path: &[Coords], min_gained: usize) -> usize {
    let mut walls = Vec::new();

    for (row_no, row) in track.iter().enumerate().skip(1).take(track.len() - 2) {
        for (col_no, field) in row.iter().enumerate().skip(1).take(row.len() - 2) {
            if *field == Field::Wall {
                walls.push((row_no, col_no));
            }
        }
    }

    let mut coord_to_step_no = HashMap::new();
    for (step_no, coords) in path.iter().enumerate() {
        coord_to_step_no.insert(coords, step_no);
    }

    let mut cheats = 0_usize;
    for (wall_row, wall_col) in walls.iter() {
        let on_row = [(*wall_row, wall_col - 1), (*wall_row, wall_col + 1)];
        let on_col = [(wall_row - 1, *wall_col), (wall_row + 1, *wall_col)];

        for cheat_candidate in [on_row, on_col].iter() {
            if let Some(one) = coord_to_step_no.get(&cheat_candidate[0])
                && let Some(two) = coord_to_step_no.get(&cheat_candidate[1])
            {
                let gained = one.abs_diff(*two) - 2;
                if gained >= min_gained {
                    cheats += 1;
                }
            }
        }
    }
    cheats
}

fn find_cheats(path: &[Coords], max_cheat_length: usize, min_gained: usize) -> usize {
    let mut cheats = 0_usize;

    for (start_path_step_no, (start_row, start_col)) in
        path[0..(path.len() - min_gained)].iter().enumerate()
    {
        let max_row = start_row + 20;
        let max_col = start_col + 20;
        let offset = start_path_step_no + min_gained.max(1_usize);
        for (end_path_step_no, (end_row, end_col)) in (offset..).zip(path.iter().skip(offset)) {
            if *end_row > max_row || *end_col > max_col {
                continue;
            }
            let shortcut_length = start_row.abs_diff(*end_row) + start_col.abs_diff(*end_col);
            if shortcut_length <= max_cheat_length {
                let distance = end_path_step_no - start_path_step_no;
                let gained = distance - shortcut_length;
                if gained >= min_gained {
                    cheats += 1;
                }
            }
        }
    }
    cheats
}

fn moves(row: usize, col: usize, max_row: isize, max_col: isize) -> Vec<Coords> {
    MOVES
        .iter()
        .filter_map(|(row_change, col_change)| {
            let next_row = row as isize + row_change;
            let next_col = col as isize + col_change;

            if next_row < 0 || next_row >= max_row || next_col < 0 || next_col >= max_col {
                None
            } else {
                Some((next_row as usize, next_col as usize))
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn test_parse_track() {
        let (_, start, end) = parse_track(SAMPLE);

        assert_eq!(start, (3, 1));
        assert_eq!(end, (7, 5));
    }

    #[test]
    fn test_find_short_cheats() {
        let (track, start, end) = parse_track(SAMPLE);

        let best_path = fastest_path(&track, start, end);
        let cheats = find_short_cheats(&track, &best_path, 0);
        assert_eq!(cheats, 44);
    }

    #[test]
    fn test_part_1_sample() {
        let (track, start, end) = parse_track(SAMPLE);

        let best_path = fastest_path(&track, start, end);
        assert_eq!(best_path.len() - 1, 84);
    }

    #[test]
    fn test_part_2_sample() {
        let (track, start, end) = parse_track(SAMPLE);

        let best_path = fastest_path(&track, start, end);
        let cheats = find_cheats(&best_path, 20, 50);
        assert_eq!(cheats, 285);
    }
}
