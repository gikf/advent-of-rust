use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

type Coordinates = (usize, usize);
type NumberCoordinates = Vec<Coordinates>;
type Numbers = HashSet<(NumberCoordinates, usize)>;
type Schema = Vec<Vec<Field>>;
type Symbols = HashMap<Coordinates, Field>;

#[derive(Clone, Debug, PartialEq)]
enum Field {
    Empty,
    Symbol(char),
    Digit(usize),
}

impl From<char> for Field {
    fn from(value: char) -> Self {
        match value {
            num @ '0'..='9' => Self::Digit(num.to_digit(10).unwrap() as usize),
            '.' => Self::Empty,
            symbol => Self::Symbol(symbol),
        }
    }
}

fn main() {
    let input = Path::new("2023/day03/src/input.txt");
    let (_, symbols, numbers) = parse_schema(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Sum of engine parts: {:?}",
        sum_parts_in_engine(&symbols, &numbers)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Sum of all gear ratios: {:?}",
        sum_of_gear_ratios(&symbols, &numbers)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_schema(input: &str) -> (Schema, Symbols, Numbers) {
    let mut symbols = HashMap::new();
    let schema: Schema = input
        .lines()
        .enumerate()
        .map(|(row_no, line)| {
            line.char_indices()
                .map(|(col_no, c)| {
                    let field = Field::from(c);
                    if matches!(field, Field::Symbol(_)) {
                        symbols.insert((row_no, col_no), field.clone());
                    }
                    field
                })
                .collect()
        })
        .collect();

    let mut numbers = HashSet::new();
    schema
        .iter()
        .chain([vec![Field::Empty; schema[0].len()]].iter())
        .enumerate()
        .for_each(|(row_no, row)| {
            let mut digits: Vec<(Coordinates, usize)> = vec![];

            for (col_no, field) in row.iter().chain([Field::Empty].iter()).enumerate() {
                match field {
                    Field::Empty | Field::Symbol(_) if !digits.is_empty() => {
                        let number: usize = digits
                            .iter()
                            .rev()
                            .enumerate()
                            .map(|(pow, (_, num))| *num * 10_usize.pow(pow as u32))
                            .sum();
                        let coords: Vec<_> =
                            digits.iter().map(|(coords, _)| coords.to_owned()).collect();
                        numbers.insert((coords, number));
                        digits.clear();
                    }
                    Field::Digit(digit) => {
                        digits.push(((row_no, col_no), *digit));
                    }
                    _ => {}
                }
            }
        });

    (schema, symbols, numbers)
}

fn numbers_in_schema(symbols: &Symbols, numbers: &Numbers) -> Vec<(NumberCoordinates, usize)> {
    numbers
        .iter()
        .filter(|(coords, _)| {
            coords.iter().any(|(row, col)| {
                let coords_by = neighbours(*row, *col);
                coords_by
                    .iter()
                    .any(|(r, c)| symbols.contains_key(&(*r, *c)))
            })
        })
        .cloned()
        .collect()
}

fn find_gears(symbols: &Symbols, numbers: &Numbers) -> Vec<Vec<usize>> {
    type SymbolNumbers<'a> = HashMap<Coordinates, Vec<(&'a NumberCoordinates, usize)>>;
    let mut symbol_mul_to_numbers: SymbolNumbers = HashMap::new();
    numbers.iter().for_each(|(coords, number)| {
        coords.iter().for_each(|(row, col)| {
            let coords_by = neighbours(*row, *col);
            for (r, c) in coords_by.iter() {
                if let Some(field) = symbols.get(&(*r, *c))
                    && matches!(field, Field::Symbol('*'))
                {
                    symbol_mul_to_numbers
                        .entry((*r, *c))
                        .and_modify(|nums| {
                            if !nums.contains(&(coords, *number)) {
                                nums.push((coords, *number));
                            }
                        })
                        .or_insert(vec![(coords, *number)]);
                }
            }
        });
    });

    symbol_mul_to_numbers
        .iter()
        .filter_map(|(_, numbers_connected)| {
            if numbers_connected.len() == 2 {
                Some(
                    numbers_connected
                        .iter()
                        .map(|(_, number)| *number)
                        .collect(),
                )
            } else {
                None
            }
        })
        .collect()
}

fn sum_parts_in_engine(symbols: &Symbols, numbers: &Numbers) -> usize {
    numbers_in_schema(symbols, numbers)
        .iter()
        .map(|(_, number)| *number)
        .sum()
}

fn sum_of_gear_ratios(symbols: &Symbols, numbers: &Numbers) -> usize {
    find_gears(symbols, numbers)
        .iter()
        .map(|nums| nums.iter().product::<usize>())
        .sum()
}

const MOVES: [(isize, isize); 8] = [
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

fn neighbours(row: usize, col: usize) -> Vec<Coordinates> {
    let row = row as isize;
    let col = col as isize;

    MOVES
        .iter()
        .filter_map(|(row_change, col_change)| {
            let new_row = row + row_change;
            let new_col = col + col_change;
            if new_row < 0 || new_col < 0 {
                None
            } else {
                Some((new_row as usize, new_col as usize))
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_sum_parts_in_engine() {
        let (_, symbols, numbers) = parse_schema(SAMPLE);

        assert_eq!(sum_parts_in_engine(&symbols, &numbers), 4361);
    }

    #[test]
    fn test_sum_of_gear_ratios() {
        let (_, symbols, numbers) = parse_schema(SAMPLE);
        assert_eq!(sum_of_gear_ratios(&symbols, &numbers), 467835);
    }
}
