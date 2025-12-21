use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

type Frequencies = HashMap<char, Vec<(isize, isize)>>;

fn main() {
    let input = Path::new("2024/day08/src/input.txt");
    let (size, frequencies) = parse_map(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Unique antinode locations: {:?}",
        unique_antinodes(size, &frequencies)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Unique antinodes, incliding harmonics: {:?}",
        unique_any_antinodes(size, &frequencies)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_map(input: &str) -> ((isize, isize), Frequencies) {
    let mut frequencies: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    let map: Vec<usize> = input
        .lines()
        .enumerate()
        .map(|(row_no, line)| {
            line.char_indices()
                .map(|(col_no, field)| match field {
                    '.' | '#' => {}
                    frequency => {
                        frequencies
                            .entry(frequency)
                            .and_modify(|points| points.push((row_no as isize, col_no as isize)))
                            .or_insert(vec![(row_no as isize, col_no as isize)]);
                    }
                })
                .count()
        })
        .collect();
    ((map.len() as isize, map[0] as isize), frequencies)
}

fn unique_antinodes((rows, cols): (isize, isize), frequencies: &Frequencies) -> usize {
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

    for antennas in frequencies.values() {
        if antennas.len() <= 1 {
            continue;
        }

        for (index, a) in antennas.iter().enumerate() {
            for b in antennas[(index + 1)..].iter() {
                let row_diff = a.0 - b.0;
                let col_diff = a.1 - b.1;
                let points = [a, b];

                for (row, col) in points.iter() {
                    for (row_change, col_change) in [(row_diff, col_diff), (-row_diff, -col_diff)] {
                        let candidate = (row - row_change, col - col_change);
                        if !points.contains(&&candidate) && is_on_map(candidate, rows, cols) {
                            antinodes.insert(candidate);
                        }
                    }
                }
            }
        }
    }

    antinodes.len()
}

fn unique_any_antinodes((rows, cols): (isize, isize), frequencies: &Frequencies) -> usize {
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

    for antennas in frequencies.values() {
        if antennas.len() <= 1 {
            continue;
        }

        for (index, a) in antennas.iter().enumerate() {
            for b in antennas[(index + 1)..].iter() {
                antinodes.insert(*a);
                antinodes.insert(*b);

                let row_diff = a.0 - b.0;
                let col_diff = a.1 - b.1;

                for (row_change, col_change) in [(row_diff, col_diff), (-row_diff, -col_diff)] {
                    let (mut row, mut col) = *a;
                    loop {
                        let candidate = (row - row_change, col - col_change);
                        if !is_on_map(candidate, rows, cols) {
                            break;
                        }
                        antinodes.insert(candidate);

                        (row, col) = candidate;
                    }
                }
            }
        }
    }

    antinodes.len()
}

fn is_on_map((row, col): (isize, isize), rows: isize, cols: isize) -> bool {
    row >= 0 && row < rows && col >= 0 && col < cols
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    const SAMPLE2: &str = "T....#....
...T......
.T....#...
.........#
..#.......
..........
...#......
..........
....#.....
..........";

    #[test]
    fn test_parse_map() {
        let (_, frequencies) = parse_map(SAMPLE);

        assert_eq!(
            frequencies,
            HashMap::from([
                ('0', Vec::from([(1, 8), (2, 5), (3, 7), (4, 4),])),
                ('A', Vec::from([(5, 6), (8, 8), (9, 9),]))
            ])
        );
    }

    #[test]
    fn test_part_1_sample() {
        let (size, frequencies) = parse_map(SAMPLE);

        let result = unique_antinodes(size, &frequencies);
        assert_eq!(result, 14);
    }

    #[test]
    fn test_unique_any_antinodes() {
        let (size, frequencies) = parse_map(SAMPLE2);

        let result = unique_any_antinodes(size, &frequencies);
        assert_eq!(result, 9);
    }

    #[test]
    fn test_part_2_sample() {
        let (size, frequencies) = parse_map(SAMPLE);

        let result = unique_any_antinodes(size, &frequencies);
        assert_eq!(result, 34);
    }
}
