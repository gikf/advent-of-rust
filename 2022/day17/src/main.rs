use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq)]
enum Jet {
    Left,
    Right,
}

impl From<char> for Jet {
    fn from(value: char) -> Self {
        match value {
            '<' => Self::Left,
            '>' => Self::Right,
            _ => unimplemented!(),
        }
    }
}

const PART1_ROCKS: usize = 2022;
const PART2_ROCKS: usize = 1_000_000_000_000;

fn main() {
    let input = Path::new("2022/day17/src/input.txt");
    let jets = parse_pattern(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Height of tower after 2022 rocks stopped falling: {:?}",
        fall_rocks(&jets, PART1_ROCKS)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Height of tower after 1 000 000 000 000 rocks stopped falling: {:?}",
        fall_rocks(&jets, PART2_ROCKS)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_pattern(input: &str) -> Vec<Jet> {
    input.chars().map(Jet::from).collect()
}

fn fall_rocks(jets: &[Jet], count: usize) -> usize {
    let chamber_width = 7;
    let rocks: [Vec<Vec<u8>>; 5] = [
        vec![vec![1; 4]],
        vec![vec![0, 1, 0], vec![1; 3], vec![0, 1, 0]],
        vec![vec![1; 3], vec![0, 0, 1], vec![0, 0, 1]],
        vec![vec![1]; 4],
        vec![vec![1; 2]; 2],
    ];

    let mut cycles = HashMap::new();
    let mut chamber: Vec<Vec<u8>> = vec![vec![1; chamber_width]];
    let mut pattern_pointer = 0;
    let mut height = 0;
    let mut rock_index = 0;

    while rock_index < count {
        let next_rock = &rocks[rock_index % 5];
        let cur_height = tower_height(&chamber);
        while chamber.len() - cur_height <= 4 + next_rock.len() {
            chamber.push(vec![0; chamber_width]);
        }

        let mut left_top = (cur_height + 4, 2);

        loop {
            let jet = &jets[pattern_pointer];
            pattern_pointer = (pattern_pointer + 1) % jets.len();

            match jet {
                Jet::Left => {
                    if left_top.1 != 0 && try_fit(&chamber, next_rock, left_top.0, left_top.1 - 1) {
                        left_top.1 -= 1;
                    }
                }

                Jet::Right => {
                    if left_top.1 + next_rock[0].len() < chamber_width
                        && try_fit(&chamber, next_rock, left_top.0, left_top.1 + 1)
                    {
                        left_top.1 += 1;
                    }
                }
            }

            if !try_fit(&chamber, next_rock, left_top.0 - 1, left_top.1) {
                for (rock_row_no, row) in next_rock.iter().enumerate() {
                    for (rock_col_no, c) in row.iter().enumerate() {
                        if *c != 1 {
                            continue;
                        }
                        let chamber_row = rock_row_no + left_top.0;
                        let chamber_col = rock_col_no + left_top.1;
                        chamber[chamber_row][chamber_col] = 1;
                    }
                }

                if chamber[left_top.0]
                    .iter()
                    .zip(chamber[left_top.0 - 1].iter())
                    .all(|(c1, c2)| *c1 == 1 || *c2 == 1)
                {
                    chamber = chamber[(left_top.0 - 1)..].to_vec();
                    height += left_top.0 - 1;
                }

                let cur_height = (0..chamber.len())
                    .rev()
                    .find(|index| chamber[*index].contains(&1))
                    .unwrap();

                let key = (
                    chamber[..=cur_height].to_vec(),
                    (rock_index % rocks.len()),
                    pattern_pointer,
                );
                if let Some((prev_rock_index, prev_height)) = cycles.get(&key) {
                    let cycle_length = rock_index - prev_rock_index;
                    if cycle_length < (count - rock_index) {
                        let cycle_height = height + cur_height - prev_height;
                        let cycles_to_skip = (count - rock_index) / cycle_length;

                        rock_index += cycles_to_skip * cycle_length;
                        height += cycles_to_skip * cycle_height;
                    }
                } else {
                    cycles.insert(key, (rock_index, cur_height + height));
                }

                break;
            }

            left_top.0 -= 1;
        }
        rock_index += 1;
    }

    tower_height(&chamber) + height
}

fn tower_height(chamber: &[Vec<u8>]) -> usize {
    (0..chamber.len())
        .rev()
        .find(|index| chamber[*index].contains(&1))
        .unwrap()
}

fn try_fit(chamber: &[Vec<u8>], rock: &[Vec<u8>], row_no: usize, col_no: usize) -> bool {
    for (rock_row_no, row) in rock.iter().enumerate() {
        for (rock_col_no, c) in row.iter().enumerate() {
            let cur_row = row_no + rock_row_no;
            let cur_col = col_no + rock_col_no;
            if chamber[cur_row][cur_col] == 1 && *c == 1 {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_part_1_sample() {
        let pattern = parse_pattern(SAMPLE);

        assert_eq!(fall_rocks(&pattern, PART1_ROCKS), 3068);
    }

    #[test]
    fn test_part_2_sample() {
        let pattern = parse_pattern(SAMPLE);

        assert_eq!(fall_rocks(&pattern, PART2_ROCKS), 1514285714288);
    }
}
