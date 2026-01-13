use std::fs;
use std::path::Path;

#[derive(Clone, Debug, PartialEq)]
enum Island {
    Ash,
    Rocks,
}

impl From<char> for Island {
    fn from(value: char) -> Self {
        match value {
            '.' => Island::Ash,
            '#' => Island::Rocks,
            _ => unimplemented!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Reflection {
    Horizontal(usize),
    Vertical(usize),
    None,
}

impl Reflection {
    const HORIZONTAL_MUL: usize = 100;

    fn score(&self) -> usize {
        match self {
            Reflection::Horizontal(value) => *value * Reflection::HORIZONTAL_MUL,
            Reflection::Vertical(value) => *value,
            Reflection::None => 0,
        }
    }
}

fn main() {
    let input = Path::new("2023/day13/src/input.txt");
    let patterns = parse_patterns(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!("Summarized patterns: {:?}", summarize_patterns(&patterns));
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    let mut patterns = patterns;
    println!("Part 2");
    println!(
        "Summarized patterns after cleaning smudges: {:?}",
        summarize_patterns_after_cleaning(&mut patterns)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_patterns(input: &str) -> Vec<Vec<Vec<Island>>> {
    let mut patterns = Vec::new();
    let mut cur_pattern = Vec::new();
    input.lines().chain([""]).for_each(|line| {
        if line.is_empty() {
            patterns.push(cur_pattern.to_owned());
            cur_pattern = Vec::new();
            return;
        }
        let pattern_line = line.chars().map(Island::from).collect();
        cur_pattern.push(pattern_line);
    });

    patterns
}

fn find_reflection(pattern: &[Vec<Island>], skip: Option<Reflection>) -> Reflection {
    let rows = pattern.len();
    for row_no in 1..rows {
        let length_to_match = (rows - row_no).min(row_no);

        let top_start = row_no - length_to_match;
        let top_end = row_no;
        let bottom_start = top_end;
        let bottom_end = bottom_start + length_to_match;

        if pattern[top_start..top_end]
            .iter()
            .zip(pattern[bottom_start..bottom_end].iter().rev())
            .all(|(a, b)| a == b)
        {
            let reflection = Reflection::Horizontal(row_no);
            if skip.as_ref().is_none() || skip.as_ref().is_some_and(|r| *r != reflection) {
                return Reflection::Horizontal(row_no);
            }
        }
    }

    let cols = pattern[0].len();
    for col_no in 1..cols {
        let length_to_match = (cols - col_no).min(col_no);

        let left_start = col_no - length_to_match;
        let left_end = col_no;
        let right_start = left_end;
        let right_end = right_start + length_to_match;

        if (left_start..left_end)
            .map(|col| pattern.iter().map(move |row| &row[col]))
            .zip(
                (right_start..right_end)
                    .rev()
                    .map(|col| pattern.iter().map(move |row| &row[col])),
            )
            .all(|(a, b)| a.zip(b).all(|(l, r)| l == r))
        {
            let reflection = Reflection::Vertical(col_no);
            if skip.as_ref().is_none() || skip.as_ref().is_some_and(|r| *r != reflection) {
                return Reflection::Vertical(col_no);
            }
        }
    }
    Reflection::None
}

fn clean_smudges(pattern: &mut Vec<Vec<Island>>) -> Reflection {
    let prev_reflection = find_reflection(pattern, None);

    for (row_no, top) in pattern[..(pattern.len() - 1)].iter().enumerate() {
        for (bottom_no, bottom) in pattern.iter().enumerate().skip(row_no) {
            let mut differing_columns: Vec<_> = top
                .iter()
                .enumerate()
                .zip(bottom.iter())
                .filter(|((_, t), b)| t != b)
                .collect();
            if differing_columns.len() == 1 {
                let ((col_no, a), b) = differing_columns.pop().unwrap();
                let mut non_smudged = pattern.clone();
                non_smudged[row_no][col_no] = b.clone();
                let new_reflection = find_reflection(&non_smudged, Some(prev_reflection));
                if new_reflection != prev_reflection && !matches!(new_reflection, Reflection::None)
                {
                    return new_reflection;
                }

                non_smudged[row_no][col_no] = a.clone();
                non_smudged[bottom_no][col_no] = a.clone();
                let new_reflection = find_reflection(&non_smudged, Some(prev_reflection));

                if new_reflection != prev_reflection && !matches!(new_reflection, Reflection::None)
                {
                    return new_reflection;
                }
            }
        }
    }

    for (col_no, _) in pattern[0][..(pattern[0].len() - 1)].iter().enumerate() {
        for (second_col, _) in pattern[0].iter().enumerate().skip(col_no) {
            let left_column = pattern.iter().map(|row| row[col_no].clone());
            let right_column = pattern.iter().map(|row| row[second_col].clone());

            let mut differing_rows: Vec<_> = left_column
                .enumerate()
                .zip(right_column)
                .filter(|((_, l), r)| l != r)
                .collect();

            if differing_rows.len() == 1 {
                let ((row_no, l), r) = differing_rows.pop().unwrap();

                let mut non_smudged = pattern.to_owned();
                non_smudged[row_no][col_no] = r.clone();

                let new_reflection = find_reflection(&non_smudged, Some(prev_reflection));

                if new_reflection != prev_reflection && !matches!(new_reflection, Reflection::None)
                {
                    return new_reflection;
                }

                non_smudged[row_no][col_no] = l.clone();
                non_smudged[row_no][second_col] = l.clone();

                let new_reflection = find_reflection(&non_smudged, Some(prev_reflection));

                if new_reflection != prev_reflection && !matches!(new_reflection, Reflection::None)
                {
                    return new_reflection;
                }
            }
        }
    }
    Reflection::None
}

fn summarize_patterns_after_cleaning(patterns: &mut [Vec<Vec<Island>>]) -> usize {
    patterns
        .iter_mut()
        .map(|pattern| clean_smudges(pattern).score())
        .sum()
}

fn summarize_patterns(patterns: &[Vec<Vec<Island>>]) -> usize {
    patterns
        .iter()
        .map(|pattern| find_reflection(pattern, None).score())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_find_reflection() {
        let patterns = parse_patterns(SAMPLE);
        assert_eq!(find_reflection(&patterns[0], None), Reflection::Vertical(5));
        assert_eq!(
            find_reflection(&patterns[1], None),
            Reflection::Horizontal(4)
        );
    }

    #[test]
    fn test_clean_smudges() {
        let mut patterns = parse_patterns(SAMPLE);

        assert_eq!(clean_smudges(&mut patterns[0]), Reflection::Horizontal(3));
        assert_eq!(clean_smudges(&mut patterns[1]), Reflection::Horizontal(1));
    }

    #[test]
    fn test_part_1_sample() {
        let patterns = parse_patterns(SAMPLE);

        assert_eq!(summarize_patterns(&patterns), 405);
    }

    #[test]
    fn test_part_2_sample() {
        let mut patterns = parse_patterns(SAMPLE);

        assert_eq!(summarize_patterns_after_cleaning(&mut patterns), 400);
    }
}
