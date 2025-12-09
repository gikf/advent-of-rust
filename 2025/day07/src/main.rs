use std::collections::HashSet;
use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq)]
struct Splitter {
    active: bool,
}

#[derive(Debug, PartialEq)]
enum Field {
    Beam,
    Empty,
    Entrance,
    Splitter(Splitter),
}

impl TryFrom<char> for Field {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Field::Empty),
            'S' => Ok(Field::Entrance),
            '^' => Ok(Field::Splitter(Splitter { active: false })),
            '|' => Ok(Field::Beam),
            _ => Err("No such field"),
        }
    }
}

fn main() {
    let input = Path::new("2025/day07/src/input.txt");
    let contents = &fs::read_to_string(input).unwrap();
    let mut diagram = parse_diagram(contents);

    let diagram = beam_taychon(&mut diagram);

    println!("Step 1");
    println!(
        "Number of times beam is split: {}",
        count_active_splitters(diagram)
    );

    println!("Step 2");
    println!("Different timelines: {}", count_timelines(diagram));
}

fn parse_diagram(input: &str) -> Vec<Vec<Field>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Field::try_from(c).unwrap_or(Field::Empty))
                .collect()
        })
        .collect()
}

fn beam_taychon(diagram: &mut [Vec<Field>]) -> &[Vec<Field>] {
    let entry_column = diagram[0]
        .iter()
        .position(|field| *field == Field::Entrance)
        .unwrap();

    diagram[1][entry_column] = Field::Beam;

    let mut beams_to_add: HashSet<(usize, usize)> = HashSet::new();
    beams_to_add.insert((1, entry_column));

    for (row_no, row) in (2_usize..diagram.len()).zip(diagram[2..].iter_mut()) {
        for (col_no, field) in row.iter_mut().enumerate() {
            let beam_above = beams_to_add.contains(&(row_no - 1, col_no));
            match (beam_above, field) {
                (true, Field::Empty) => {
                    beams_to_add.insert((row_no, col_no));
                }
                (true, Field::Splitter(splitter)) => {
                    splitter.active = true;
                    beams_to_add.insert((row_no, col_no - 1));
                    beams_to_add.insert((row_no, col_no + 1));
                }
                _ => {}
            }
        }
    }

    for (beam_row, beam_col) in beams_to_add.drain() {
        diagram[beam_row][beam_col] = Field::Beam;
    }

    diagram
}

fn count_active_splitters(diagram: &[Vec<Field>]) -> usize {
    diagram
        .iter()
        .map(|row| {
            row.iter()
                .filter(|col| {
                    if let Field::Splitter(splitter) = col {
                        splitter.active
                    } else {
                        false
                    }
                })
                .count()
        })
        .sum()
}

fn count_timelines(diagram: &[Vec<Field>]) -> usize {
    let mut timelines: Vec<Vec<usize>> = (0..diagram.len())
        .map(|_| (0..diagram[0].len()).map(|_| 0).collect())
        .collect();

    let last_row = diagram.len() - 1;

    timelines[last_row] = timelines[last_row].iter().map(|_| 1).collect();

    for (row_no, row) in (0..last_row).rev().zip(diagram[..last_row].iter().rev()) {
        for (col_no, field) in row.iter().enumerate() {
            match field {
                Field::Beam | Field::Entrance => {
                    timelines[row_no][col_no] = timelines[row_no + 1][col_no];
                }
                Field::Splitter(splitter) => {
                    if splitter.active {
                        timelines[row_no][col_no + 1] = timelines[row_no + 1][col_no + 1];

                        timelines[row_no][col_no] =
                            timelines[row_no][col_no - 1] + timelines[row_no][col_no + 1];
                    }
                }
                Field::Empty => {}
            }
        }
    }

    *timelines[0].iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = ".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............\n....^.^...^....\n...............\n...^.^...^.^...\n...............\n..^...^.....^..\n...............\n.^.^.^.^.^...^.\n...............";

    #[test]
    fn test_count_active_splitters_step_1() {
        let mut diagram = parse_diagram(SAMPLE);
        let diagram = beam_taychon(&mut diagram);

        assert_eq!(count_active_splitters(diagram), 21);
    }

    #[test]
    fn test_count_timelines_step_2() {
        let mut diagram = parse_diagram(SAMPLE);
        let diagram = beam_taychon(&mut diagram);

        assert_eq!(count_timelines(diagram), 40);
    }

    #[test]
    fn test_field_parse() {
        assert_eq!(Field::try_from('|'), Ok(Field::Beam));
        assert_eq!(Field::try_from('.'), Ok(Field::Empty));
        assert_eq!(Field::try_from('S'), Ok(Field::Entrance));
        assert_eq!(
            Field::try_from('^'),
            Ok(Field::Splitter(Splitter { active: false }))
        );

        assert_eq!(Field::try_from('x'), Err("No such field"));
    }
}
