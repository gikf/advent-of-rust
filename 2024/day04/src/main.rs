use std::fs;
use std::path::Path;

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
const SAMX: [char; 4] = ['S', 'A', 'M', 'X'];
const X_MAS: [[[char; 3]; 2]; 4] = [
    [['M', 'A', 'S'], ['M', 'A', 'S']],
    [['M', 'A', 'S'], ['S', 'A', 'M']],
    [['S', 'A', 'M'], ['S', 'A', 'M']],
    [['S', 'A', 'M'], ['M', 'A', 'S']],
];

fn main() {
    let input = Path::new("2024/day04/src/input.txt");
    let contents = &fs::read_to_string(input).unwrap();

    println!("Part 1");
    println!("Number of XMAS: {:?}", count_xmas(contents));

    println!("Part 2");
    println!("Number of X-MAS: {:?}", count_x_mas(contents));
}

fn count_x_mas(input: &str) -> usize {
    let mut count = 0;

    let word_search: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = word_search.len();
    let cols = word_search[0].len();

    for (row_no, row) in word_search.iter().enumerate() {
        for (col_no, character) in row.iter().enumerate() {
            if !['M', 'S'].contains(character) {
                continue;
            }
            let coordinates = x_coordinates(row_no, col_no, rows, cols);
            let candidates: Vec<Vec<char>> = coordinates
                .iter()
                .map(|word| word.iter().map(|(r, c)| word_search[*r][*c]).collect())
                .collect();

            if let [word1, word2] = &candidates[..]
                && X_MAS.iter().any(|expected_variant| {
                    expected_variant
                        .iter()
                        .zip([word1, word2].iter())
                        .all(|(expected, word)| **word == *expected)
                })
            {
                count += 1;
            }
        }
    }

    count
}

fn x_coordinates(
    row_no: usize,
    col_no: usize,
    rows: usize,
    cols: usize,
) -> Vec<Vec<(usize, usize)>> {
    let mut coordinates: Vec<Vec<(usize, usize)>> = Vec::new();

    if row_no + 2 < rows && col_no + 2 < cols {
        let left_top = row_no;
        let left_bottom = row_no + 2;
        coordinates.push(
            (0..3usize)
                .map(|offset| (left_top + offset, col_no + offset))
                .collect(),
        );
        coordinates.push(
            (0..3usize)
                .map(|offset| (left_bottom - offset, col_no + offset))
                .collect(),
        );
    }

    coordinates
}

fn count_xmas(input: &str) -> usize {
    let mut count = 0;

    let word_search: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = word_search.len();
    let cols = word_search[0].len();

    for (row_no, row) in word_search.iter().enumerate() {
        for (col_no, character) in row.iter().enumerate() {
            if !['X', 'S'].contains(character) {
                continue;
            }
            let candidates = related_coordinates(row_no, col_no, rows, cols);

            for candidate in candidates {
                let word: Vec<char> = candidate.iter().map(|(r, c)| word_search[*r][*c]).collect();
                if word == XMAS || word == SAMX {
                    count += 1;
                }
            }
        }
    }
    count
}

fn related_coordinates(
    row_no: usize,
    col_no: usize,
    rows: usize,
    cols: usize,
) -> Vec<Vec<(usize, usize)>> {
    let mut coordinates = Vec::new();

    let forward_cols = col_no + 3 < cols;
    let backward_cols = col_no >= 3;
    let forward_rows = row_no + 3 < rows;
    if forward_cols {
        coordinates.push((col_no..(col_no + 4)).map(|col| (row_no, col)).collect());
    }
    if forward_rows {
        coordinates.push((row_no..(row_no + 4)).map(|row| (row, col_no)).collect());

        if forward_cols {
            coordinates.push(
                (0..4usize)
                    .map(|offset| (row_no + offset, col_no + offset))
                    .collect(),
            );
        }
        if backward_cols {
            coordinates.push(
                (0..4usize)
                    .map(|offset| (row_no + offset, col_no - offset))
                    .collect(),
            );
        }
    }

    coordinates
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX";

    #[test]
    fn test_related_coordinates() {
        assert_eq!(
            related_coordinates(0, 1, 10, 10),
            [
                [(0, 1), (0, 2), (0, 3), (0, 4)],
                [(0, 1), (1, 1), (2, 1), (3, 1)],
                [(0, 1), (1, 2), (2, 3), (3, 4)],
            ]
        );
        assert_eq!(
            related_coordinates(6, 1, 10, 10),
            [
                [(6, 1), (6, 2), (6, 3), (6, 4)],
                [(6, 1), (7, 1), (8, 1), (9, 1)],
                [(6, 1), (7, 2), (8, 3), (9, 4)],
            ]
        );
    }

    #[test]
    fn test_part_1_sample() {
        let count = count_xmas(SAMPLE);
        assert_eq!(count, 18);
    }

    #[test]
    fn test_part_2_sample() {
        let count = count_x_mas(SAMPLE);
        assert_eq!(count, 9);
    }
}
