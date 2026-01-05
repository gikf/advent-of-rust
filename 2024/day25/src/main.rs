use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq)]
enum Field {
    Filled,
    Empty,
}

impl From<char> for Field {
    fn from(value: char) -> Self {
        match value {
            '#' => Field::Filled,
            '.' => Field::Empty,
            _ => unimplemented!(),
        }
    }
}

fn main() {
    let input = Path::new("2024/day25/src/input.txt");
    let (keys, locks) = parse_input(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Unique pairs, without overlapping pins: {:?}",
        fitting_combinations(&keys, &locks)
    );
    println!("In {:?}", part1.elapsed());
}

fn parse_input(input: &str) -> (Vec<[u8; 5]>, Vec<[u8; 5]>) {
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    let mut current_schema: Vec<Vec<Field>> = Vec::new();
    for line in input.lines().chain([""].into_iter()) {
        if line.is_empty() {
            let is_lock = current_schema[0].iter().all(|f| *f == Field::Filled);
            let mut pins: [u8; 5] = [0; 5];
            for col_no in 0..5 {
                pins[col_no] = current_schema[if is_lock { 1..=6 } else { 0..=5 }]
                    .iter()
                    .filter(|row| row[col_no] == Field::Filled)
                    .count() as u8;
            }
            if is_lock {
                locks.push(pins);
            } else {
                keys.push(pins);
            }
            current_schema.clear();
            continue;
        }
        current_schema.push(line.chars().map(Field::from).collect());
    }
    (keys, locks)
}

fn fitting_combinations(keys: &[[u8; 5]], locks: &[[u8; 5]]) -> usize {
    let free_space_for_pins = 5;
    keys.iter()
        .map(|key| {
            locks
                .iter()
                .filter(|lock| {
                    key.iter()
                        .zip(lock.iter())
                        .all(|(pins_a, pins_b)| pins_a + pins_b <= free_space_for_pins)
                })
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "#####\n.####\n.####\n.####\n.#.#.\n.#...\n.....

#####\n##.##\n.#.##\n...##\n...#.\n...#.\n.....

.....\n#....\n#....\n#...#\n#.#.#\n#.###\n#####

.....\n.....\n#.#..\n###..\n###.#\n###.#\n#####

.....\n.....\n.....\n#....\n#.#..\n#.#.#\n#####";

    #[test]
    fn test_parse_input() {
        let (keys, locks) = parse_input(SAMPLE);

        assert_eq!(keys.len(), 3);
        assert_eq!(locks.len(), 2);

        assert_eq!(
            keys,
            Vec::from([[5, 0, 2, 1, 3], [4, 3, 4, 0, 2], [3, 0, 2, 0, 1]])
        );
        assert_eq!(locks, Vec::from([[0, 5, 3, 4, 3], [1, 2, 0, 5, 3]]));
    }

    #[test]
    fn test_fitting_combinations() {
        let (keys, locks) = parse_input(SAMPLE);

        assert_eq!(fitting_combinations(&keys, &locks), 3);
    }
}
