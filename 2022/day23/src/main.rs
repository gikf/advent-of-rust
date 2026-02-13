use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

const ROUNDS_PART1: usize = 10;

#[derive(Debug, PartialEq)]
enum Neighbours {
    N,
    NE,
    NW,
    S,
    SE,
    SW,
    E,
    W,
}

impl Neighbours {
    fn changes(&self) -> (i16, i16) {
        match self {
            Neighbours::N => (-1, 0),
            Neighbours::NE => (-1, 1),
            Neighbours::NW => (-1, -1),
            Neighbours::S => (1, 0),
            Neighbours::SE => (1, 1),
            Neighbours::SW => (1, -1),
            Neighbours::E => (0, 1),
            Neighbours::W => (0, -1),
        }
    }

    fn changes_for(&self, row: i16, col: i16) -> (i16, i16) {
        match self {
            Neighbours::N => (row - 1, col),
            Neighbours::NE => (row - 1, col + 1),
            Neighbours::NW => (row - 1, col - 1),
            Neighbours::S => (row + 1, col),
            Neighbours::SE => (row + 1, col + 1),
            Neighbours::SW => (row + 1, col - 1),
            Neighbours::E => (row, col + 1),
            Neighbours::W => (row, col - 1),
        }
    }
}

fn main() {
    let input = Path::new("2022/day23/src/input.txt");
    let elves = parse_elves(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Empty ground tiles after 10 rounds: {:?}",
        rounds(&elves, ROUNDS_PART1)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!("First round without moves: {:?}", stop_motion(&elves));
    println!("In {:?}", part2.elapsed());
}

fn parse_elves(input: &str) -> HashSet<(i16, i16)> {
    let mut elves = HashSet::new();
    input.lines().enumerate().for_each(|(row_no, line)| {
        line.char_indices().for_each(|(col_no, c)| {
            if c == '#' {
                elves.insert((row_no as i16, col_no as i16));
            }
        })
    });
    elves
}

fn count_empty(elves: &HashSet<(i16, i16)>) -> usize {
    let mut min_row = i16::MAX;
    let mut max_row = i16::MIN;
    let mut min_col = i16::MAX;
    let mut max_col = i16::MIN;

    for (row, col) in elves {
        min_row = min_row.min(*row);
        max_row = max_row.max(*row);
        min_col = min_col.min(*col);
        max_col = max_col.max(*col);
    }

    let mut count = 0;
    for row in min_row..=max_row {
        for col in min_col..=max_col {
            if !elves.contains(&(row, col)) {
                count += 1;
            }
        }
    }
    count
}

const ELF_CONSIDERATIONS: [([Neighbours; 3], Neighbours); 4] = [
    (
        [Neighbours::N, Neighbours::NE, Neighbours::NW],
        Neighbours::N,
    ),
    (
        [Neighbours::S, Neighbours::SE, Neighbours::SW],
        Neighbours::S,
    ),
    (
        [Neighbours::W, Neighbours::NW, Neighbours::SW],
        Neighbours::W,
    ),
    (
        [Neighbours::E, Neighbours::NE, Neighbours::SE],
        Neighbours::E,
    ),
];

fn rounds(elves: &HashSet<(i16, i16)>, n: usize) -> usize {
    let mut elves = elves.clone();
    let elves_count = elves.len();
    let mut propositions: HashMap<(i16, i16), Vec<(i16, i16)>> =
        HashMap::with_capacity(elves_count);

    for round in 0..n {
        let mut next_elves = HashSet::with_capacity(elves_count);

        let offset = round % ELF_CONSIDERATIONS.len();
        let considerations: Vec<_> = ELF_CONSIDERATIONS[offset..]
            .iter()
            .chain(ELF_CONSIDERATIONS[..offset].iter())
            .collect();

        for (row, col) in elves.iter() {
            let neighbours = present_neighbours(*row, *col, &elves);

            if neighbours.is_empty() {
                next_elves.insert((*row, *col));
            } else {
                let mut proposed = false;
                for (pos, next_move) in &considerations {
                    if pos.iter().all(|p| !neighbours.contains(&p)) {
                        let (row_change, col_change) = next_move.changes();
                        propositions
                            .entry((*row + row_change, *col + col_change))
                            .and_modify(|es| es.push((*row, *col)))
                            .or_insert(vec![(*row, *col)]);
                        proposed = true;
                        break;
                    }
                }

                if !proposed {
                    next_elves.insert((*row, *col));
                }
            }
        }

        for (to, elves_from) in propositions.drain() {
            if elves_from.len() == 1 {
                next_elves.insert(to);
            } else {
                next_elves.extend(elves_from);
            }
        }

        elves = next_elves
    }

    count_empty(&elves)
}

fn stop_motion(elves: &HashSet<(i16, i16)>) -> usize {
    let mut elves = elves.clone();
    let elves_count = elves.len();
    let mut propositions: HashMap<(i16, i16), Vec<(i16, i16)>> =
        HashMap::with_capacity(elves_count);

    for round in 0.. {
        let offset = round % ELF_CONSIDERATIONS.len();
        let considerations: Vec<_> = ELF_CONSIDERATIONS[offset..]
            .iter()
            .chain(ELF_CONSIDERATIONS[..offset].iter())
            .collect();

        for (row, col) in elves.iter() {
            let neighbours = present_neighbours(*row, *col, &elves);

            if !neighbours.is_empty() {
                for (positions, next_move) in &considerations {
                    if positions
                        .iter()
                        .all(|position| !neighbours.contains(&position))
                    {
                        let (next_row, next_col) = next_move.changes_for(*row, *col);
                        propositions
                            .entry((next_row, next_col))
                            .and_modify(|other_elves| other_elves.push((*row, *col)))
                            .or_insert(vec![(*row, *col)]);
                        break;
                    }
                }
            }
        }

        if propositions.is_empty() {
            return round + 1;
        }
        for (to, elves_from) in propositions.drain() {
            if elves_from.len() == 1 {
                elves.remove(&(elves_from[0]));
                elves.insert(to);
            }
        }
    }
    unreachable!()
}

fn present_neighbours(row: i16, col: i16, elves: &HashSet<(i16, i16)>) -> Vec<&Neighbours> {
    [
        Neighbours::E,
        Neighbours::NE,
        Neighbours::N,
        Neighbours::NW,
        Neighbours::S,
        Neighbours::SE,
        Neighbours::SW,
        Neighbours::W,
    ]
    .iter()
    .filter(|n| {
        let (row_change, col_change) = n.changes();
        elves.contains(&(row + row_change, col + col_change))
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";
    const SAMPLE_RESULT: &str = ".......#......
...........#..
..#.#..#......
......#.......
...#.....#..#.
.#......##....
.....##.......
..#........#..
....#.#..#....
..............
....#..#..#...
..............";

    #[test]
    fn test_count_empty() {
        let result = parse_elves(SAMPLE_RESULT);

        assert_eq!(count_empty(&result), 110);
    }

    #[test]
    fn test_part_1_sample() {
        let elves = parse_elves(SAMPLE);

        assert_eq!(rounds(&elves, ROUNDS_PART1), 110);
    }

    #[test]
    fn test_part_2_sample() {
        let elves = parse_elves(SAMPLE);

        assert_eq!(stop_motion(&elves), 20);
    }
}
