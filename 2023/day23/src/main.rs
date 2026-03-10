use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs;
use std::path::Path;

enum Field {
    Forest,
    Path,
    SlopeUp,
    SlopeRight,
    SlopeDown,
    SlopeLeft,
}

impl From<char> for Field {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Forest,
            '.' => Self::Path,
            '^' => Self::SlopeUp,
            '>' => Self::SlopeRight,
            'v' => Self::SlopeDown,
            '<' => Self::SlopeLeft,
            _ => unimplemented!(),
        }
    }
}

fn main() {
    let input = Path::new("2023/day23/src/input.txt");
    let map = parse_map(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!("Longest hike: {:?}", longest_hike(&map, moves_part1));
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!("Longest dry hike: {:?}", longest_dry_hike(&map));
    println!("In {:?}", part2.elapsed());
}

fn parse_map(input: &str) -> Vec<Vec<Field>> {
    let mut map: Vec<Vec<Field>> = input
        .lines()
        .map(|line| line.chars().map(Field::from).collect())
        .collect();
    map[0][1] = Field::SlopeDown;
    map
}

fn simplify_map(map: &[Vec<Field>]) -> HashMap<u16, HashMap<u16, usize>> {
    let rows = map.len();
    let cols = map[0].len();
    let start = (0, 1);
    let end = (map.len() - 1, map[0].len() - 2);

    let mut queue = VecDeque::new();
    queue.push_front((1, start, (0, 1)));
    let mut seen = HashSet::new();

    let mut crossroads = vec![start, end];

    while let Some((distance, from, (row, col))) = queue.pop_front() {
        if !seen.insert((row, col)) {
            continue;
        }

        let field = &map[row][col];

        let next_moves: Vec<_> = moves_part2(field)
            .iter()
            .filter_map(|(row_change, col_change)| {
                let next_row = row as isize + row_change;
                let next_col = col as isize + col_change;
                if !(0..rows as isize).contains(&next_row)
                    || !(0..cols as isize).contains(&next_col)
                {
                    None
                } else {
                    let next_row = next_row as usize;
                    let next_col = next_col as usize;
                    let next_field = &map[next_row][next_col];

                    if matches!(next_field, Field::Forest) {
                        None
                    } else {
                        Some((next_row, next_col))
                    }
                }
            })
            .collect();

        if next_moves.len() > 2 {
            crossroads.push((row, col));
            for next_move in next_moves {
                queue.push_back((1, (row, col), next_move));
            }
        } else {
            for next_move in next_moves {
                queue.push_back((distance + 1, from, (next_move.0, next_move.1)));
            }
        }
    }

    let mut point_to_num = HashMap::new();
    let mut num_to_point = HashMap::new();
    point_to_num.insert(start, 0_u16);
    point_to_num.insert(end, 1);
    num_to_point.insert(0, start);
    num_to_point.insert(1, end);

    for (num, point) in (2..).zip(&crossroads) {
        if *point == start || *point == end {
            continue;
        }
        point_to_num.insert(*point, num);
        num_to_point.insert(num, *point);
    }

    let mut distances = HashMap::new();
    for point in &crossroads {
        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((0, *point));
        while let Some((steps, (row, col))) = queue.pop_front() {
            if !seen.insert((row, col)) {
                continue;
            }

            if *point != (row, col) && crossroads.contains(&(row, col)) {
                let point_num = point_to_num.get(point).unwrap();
                let other_num = point_to_num.get(&(row, col)).unwrap();
                distances
                    .entry(*point_num)
                    .and_modify(|points: &mut HashMap<u16, usize>| {
                        points.insert(*other_num, steps);
                    })
                    .or_insert(HashMap::from([(*other_num, steps)]));
                distances
                    .entry(*other_num)
                    .and_modify(|points: &mut HashMap<u16, usize>| {
                        points.insert(*point_num, steps);
                    })
                    .or_insert(HashMap::from([(*point_num, steps)]));
                continue;
            }
            let field = &map[row][col];
            let next_moves = moves_part2(field);
            let next_moves = next_moves.iter().filter_map(|(row_change, col_change)| {
                let next_row = row as isize + row_change;
                let next_col = col as isize + col_change;
                if !(0..rows as isize).contains(&next_row)
                    || !(0..cols as isize).contains(&next_col)
                {
                    None
                } else {
                    let next_row = next_row as usize;
                    let next_col = next_col as usize;
                    let next_field = &map[next_row][next_col];

                    if matches!(next_field, Field::Forest) {
                        None
                    } else {
                        Some((next_row, next_col))
                    }
                }
            });
            for next_move in next_moves {
                queue.push_back((steps + 1, next_move));
            }
        }
    }
    distances
}

fn longest_hike<MovesFunc: Fn(&Field) -> Vec<(isize, isize)>>(
    map: &[Vec<Field>],
    next_moves: MovesFunc,
) -> usize {
    let start = (0, 1);
    let end = (map.len() - 1, map[0].len() - 2);

    let rows = map.len();
    let cols = map[0].len();

    let mut queue = BinaryHeap::new();
    queue.push((0, start, vec![0; rows * cols]));

    let mut most_steps = vec![vec![-1_isize; cols]; rows];

    while let Some((steps, (row, col), mut visits)) = queue.pop() {
        let index = row * cols + col;
        if visits[index] == 1 {
            continue;
        }
        visits[index] = 1;

        if most_steps[row][col] >= steps {
            continue;
        }
        most_steps[row][col] = most_steps[row][col].max(steps);

        if (row, col) == end {
            continue;
        }

        let field = &map[row][col];

        for (row_change, col_change) in next_moves(field) {
            let next_row = row as isize + row_change;
            let next_col = (col as isize + col_change) as usize;

            if next_row < 0 || next_row >= rows as isize {
                continue;
            }

            let next_row = next_row as usize;
            let next_field = &map[next_row][next_col];
            if matches!(next_field, Field::Forest) {
                continue;
            }

            queue.push((steps + 1, (next_row, next_col), visits.clone()));
        }
    }

    most_steps[end.0][end.1] as usize
}

fn longest_dry_hike(map: &[Vec<Field>]) -> usize {
    let simplified = simplify_map(map);
    let start = 0;
    let end = 1;
    let mut queue = Vec::new();
    queue.push((
        0,
        start,
        1,
        vec![0_u8; *simplified.keys().max().unwrap() as usize + 1],
    ));
    let mut result = 0;

    while let Some((distance, point_num, steps_so_far, visits)) = queue.pop() {
        if point_num == end {
            if result < distance {
                result = distance;
            }
            continue;
        }

        if let Some(targets) = simplified.get(&point_num) {
            for (next_point_num, steps) in targets {
                if visits[*next_point_num as usize] != 0 {
                    continue;
                }
                let mut next_path = visits.clone();
                next_path[*next_point_num as usize] = steps_so_far;
                queue.push((
                    distance + steps,
                    *next_point_num,
                    steps_so_far + 1,
                    next_path,
                ));
            }
        }
    }

    result
}

fn moves_part1(field: &Field) -> Vec<(isize, isize)> {
    match field {
        Field::Forest => vec![],
        Field::Path => vec![(-1, 0), (1, 0), (0, -1), (0, 1)],
        Field::SlopeUp => vec![(-1, 0)],
        Field::SlopeRight => vec![(0, 1)],
        Field::SlopeDown => vec![(1, 0)],
        Field::SlopeLeft => vec![(0, -1)],
    }
}

fn moves_part2(field: &Field) -> Vec<(isize, isize)> {
    match field {
        Field::Forest => vec![],
        _ => vec![(-1, 0), (1, 0), (0, -1), (0, 1)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn test_part_1_sample() {
        let map = parse_map(SAMPLE);

        assert_eq!(longest_hike(&map, moves_part1), 94);
    }

    #[test]
    fn test_part_2_sample() {
        let map = parse_map(SAMPLE);

        assert_eq!(longest_dry_hike(&map), 154);
    }
}
