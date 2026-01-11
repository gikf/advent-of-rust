use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq)]
enum Space {
    Empty,
    Galaxy,
}

impl From<char> for Space {
    fn from(value: char) -> Self {
        match value {
            '.' => Space::Empty,
            '#' => Space::Galaxy,
            _ => unimplemented!(),
        }
    }
}

fn main() {
    let input = Path::new("2023/day11/src/input.txt");
    let galaxies = parse_input(&fs::read_to_string(input).unwrap(), 2);

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Sum of shortest paths between galaxies: {:?}",
        sum_of_lengths_between_galaxies(&galaxies)
    );
    println!("In {:?}", part1.elapsed());

    let galaxies_part2 = parse_input(&fs::read_to_string(input).unwrap(), 1_000_000);
    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Sum of shortest paths between \"one million\" expanded galaxies: {:?}",
        sum_of_lengths_between_galaxies(&galaxies_part2)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_input(input: &str, expansion: usize) -> Vec<(usize, usize)> {
    let mut galaxies = Vec::new();
    let star_map: Vec<Vec<Space>> = input
        .lines()
        .enumerate()
        .map(|(row_no, row)| {
            row.char_indices()
                .map(|(col_no, field)| {
                    let space = Space::from(field);
                    if matches!(space, Space::Galaxy) {
                        galaxies.push((row_no, col_no));
                    }
                    space
                })
                .collect()
        })
        .collect();

    let mut row_expansion = 0;
    for (row_no, row) in star_map.iter().enumerate() {
        if !row.contains(&Space::Galaxy) {
            let min_row_to_expand = row_no + row_expansion;
            for galaxy in galaxies.iter_mut() {
                if galaxy.0 > min_row_to_expand {
                    galaxy.0 += expansion - 1;
                }
            }
            row_expansion += expansion - 1;
        }
    }

    let mut col_expansion = 0;
    for col_no in 0..star_map[0].len() {
        if (0..star_map.len())
            .map(|row_no| &star_map[row_no][col_no])
            .all(|space| matches!(space, Space::Empty))
        {
            let min_col_to_epxand = col_no + col_expansion;
            for galaxy in galaxies.iter_mut() {
                if galaxy.1 > min_col_to_epxand {
                    galaxy.1 += expansion - 1;
                }
            }
            col_expansion += expansion - 1;
        }
    }
    galaxies
}

fn taxicab_distance(galaxy1: (usize, usize), galaxy2: (usize, usize)) -> usize {
    galaxy1.0.abs_diff(galaxy2.0) + galaxy1.1.abs_diff(galaxy2.1)
}

fn sum_of_lengths_between_galaxies(galaxies: &[(usize, usize)]) -> usize {
    let mut sum = 0;
    for (index, galaxy1) in galaxies.iter().enumerate() {
        for galaxy2 in galaxies.iter().skip(index + 1) {
            sum += taxicab_distance(*galaxy1, *galaxy2);
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    const SAMPLE: &str = "...#......\n.......#..
#.........\n..........\n......#...\n.#........
.........#\n..........\n.......#..\n#...#.....";

    #[test]
    fn test_parse_galaxies() {
        let galaxies = parse_input(SAMPLE, 2);

        assert_eq!(
            HashSet::from_iter(galaxies.into_iter()),
            HashSet::from([
                (0, 4),
                (1, 9),
                (2, 0),
                (5, 8),
                (6, 1),
                (7, 12),
                (10, 9),
                (11, 0),
                (11, 5),
            ])
        );

        let galaxies2 = parse_input(SAMPLE, 10);

        assert_eq!(
            HashSet::from_iter(galaxies2.into_iter()),
            HashSet::from([
                (0, 12),
                (1, 25),
                (2, 0),
                (13, 24),
                (14, 1),
                (15, 36),
                (26, 25),
                (27, 0),
                (27, 13),
            ])
        )
    }

    #[test]
    fn test_shortest_path() {
        assert_eq!(taxicab_distance((0, 4), (10, 9)), 15);
        assert_eq!(taxicab_distance((2, 0), (7, 12)), 17);
        assert_eq!(taxicab_distance((11, 0), (11, 5)), 5);
        assert_eq!(taxicab_distance((6, 1), (11, 5)), 9);
    }

    #[test]
    fn test_shortest_path2() {
        assert_eq!(taxicab_distance((0, 13), (28, 27)), 42);
        assert_eq!(taxicab_distance((2, 0), (16, 39)), 53);
        assert_eq!(taxicab_distance((29, 0), (29, 14)), 14);
        assert_eq!(taxicab_distance((15, 1), (29, 14)), 27);
    }

    #[test]
    fn test_part_1_sample() {
        let galaxies = parse_input(SAMPLE, 2);

        assert_eq!(sum_of_lengths_between_galaxies(&galaxies), 374);
    }

    #[test]
    fn test_part_2_sample() {
        let galaxies1 = parse_input(SAMPLE, 10);
        assert_eq!(sum_of_lengths_between_galaxies(&galaxies1), 1030);
        let galaxies2 = parse_input(SAMPLE, 100);
        assert_eq!(sum_of_lengths_between_galaxies(&galaxies2), 8410);
    }
}
