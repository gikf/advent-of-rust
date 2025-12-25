use std::collections::{HashSet, VecDeque};
use std::fs;
use std::path::Path;

enum Side {
    Prev,
    Cur,
}

type Region = (
    char,
    HashSet<(usize, usize)>,
    HashSet<usize>,
    HashSet<usize>,
);

const MOVES: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn main() {
    let input = Path::new("2024/day12/src/input.txt");
    let plots = parse_plots(&fs::read_to_string(input).unwrap());

    let regions = find_regions(&plots);

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!("Total price for all regions: {:?}", regions_price(&regions));
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Total price with bulk discout for all regions: {:?}",
        regions_discount_price(&regions)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_plots(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_regions(plots: &[Vec<char>]) -> Vec<Region> {
    let mut regions: Vec<Region> = Vec::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    for (row_no, row) in plots.iter().enumerate() {
        for (col_no, region_type) in row.iter().enumerate() {
            if visited.contains(&(row_no, col_no)) {
                continue;
            }

            let (fields, rows, cols) =
                find_region_fields(*region_type, (row_no, col_no), plots, &mut visited);

            regions.push((*region_type, fields, rows, cols));
        }
    }

    regions
}

fn find_region_fields(
    region_type: char,
    (row, col): (usize, usize),
    plots: &[Vec<char>],
    visited: &mut HashSet<(usize, usize)>,
) -> (HashSet<(usize, usize)>, HashSet<usize>, HashSet<usize>) {
    let mut region = HashSet::new();
    let mut rows = HashSet::new();
    let mut cols = HashSet::new();
    let row_limit = plots.len();
    let col_limit = plots[0].len();

    let mut queue = VecDeque::new();
    queue.push_back((row, col));

    while let Some((cur_row, cur_col)) = queue.pop_front() {
        if visited.contains(&(cur_row, cur_col)) {
            continue;
        }
        visited.insert((cur_row, cur_col));
        region.insert((cur_row, cur_col));
        rows.insert(cur_row);
        cols.insert(cur_col);

        for (new_row, new_col) in move_coords(cur_row, cur_col, row_limit, col_limit) {
            if plots[new_row][new_col] == region_type {
                queue.push_back((new_row, new_col));
            }
        }
    }

    (region, rows, cols)
}

fn regions_price(regions: &[Region]) -> usize {
    regions
        .iter()
        .map(|region| region_price(region, region_perimeter))
        .sum()
}

fn regions_discount_price(regions: &[Region]) -> usize {
    regions
        .iter()
        .map(|region| region_price(region, region_sides))
        .sum()
}

fn region_price<PerimeterCalc: Fn(&Region) -> usize>(
    region: &Region,
    func: PerimeterCalc,
) -> usize {
    let (_, fields, _, _) = region;
    let area = fields.len();

    area * func(region)
}

fn region_perimeter(region: &Region) -> usize {
    let mut perimeter = 0;

    for (row, col) in region.1.iter() {
        let neighbours = move_coords(*row, *col, row + 2, col + 2);
        let mut cur_perimeter = 4;
        for (n_row, n_col) in neighbours {
            if region.1.contains(&(n_row, n_col)) {
                cur_perimeter -= 1;
            }
        }
        perimeter += cur_perimeter;
    }

    perimeter
}

fn region_sides(region: &Region) -> usize {
    let (_, fields, rows, cols) = region;

    let min_row = *rows.iter().min().unwrap();
    let max_row = *rows.iter().max().unwrap();

    let min_col = *cols.iter().min().unwrap();
    let max_col = *cols.iter().max().unwrap();

    let col_sides: usize = (min_col..=(max_col + 1))
        .map(|col| {
            if col == min_col {
                border_sides(fields, (min_row..=max_row).zip([col].into_iter().cycle()))
            } else if col == max_col + 1 {
                border_sides(
                    fields,
                    (min_row..=max_row).zip([col - 1].into_iter().cycle()),
                )
            } else {
                internal_sides(
                    fields,
                    (min_row..=max_row)
                        .map(|r| (r, r))
                        .zip([col].into_iter().map(|c| (c - 1, c)).cycle()),
                )
            }
        })
        .sum();

    let row_sides: usize = (min_row..=(max_row + 1))
        .map(|row| {
            if row == min_row {
                border_sides(fields, [row].into_iter().cycle().zip(min_col..=max_col))
            } else if row == max_row + 1 {
                border_sides(fields, [row - 1].into_iter().cycle().zip(min_col..=max_col))
            } else {
                internal_sides(
                    fields,
                    [row]
                        .into_iter()
                        .map(|r| (r - 1, r))
                        .cycle()
                        .zip((min_col..=max_col).map(|c| (c, c))),
                )
            }
        })
        .sum();

    row_sides + col_sides
}

fn internal_sides(
    fields: &HashSet<(usize, usize)>,
    iterator: impl Iterator<Item = ((usize, usize), (usize, usize))>,
) -> usize {
    let mut sides = 0;
    let mut is_side = None;

    for ((row1, row2), (col1, col2)) in iterator {
        match (
            fields.contains(&(row1, col1)),
            fields.contains(&(row2, col2)),
        ) {
            (true, false) => match is_side {
                Some(Side::Prev) => {}
                Some(Side::Cur) => {
                    sides += 1;
                    is_side = Some(Side::Prev);
                }
                None => {
                    is_side = Some(Side::Prev);
                }
            },
            (false, true) => match is_side {
                Some(Side::Prev) => {
                    sides += 1;
                    is_side = Some(Side::Cur);
                }
                Some(Side::Cur) => {}
                None => {
                    is_side = Some(Side::Cur);
                }
            },
            (true, true) => match is_side {
                Some(Side::Prev) | Some(Side::Cur) => {
                    is_side = None;
                    sides += 1;
                }
                None => {}
            },
            (false, false) => match is_side {
                Some(Side::Prev) | Some(Side::Cur) => {
                    is_side = None;
                    sides += 1;
                }
                None => {}
            },
        }
    }
    if is_side.is_some() {
        sides += 1;
    }
    sides
}

fn border_sides(
    fields: &HashSet<(usize, usize)>,
    iterator: impl Iterator<Item = (usize, usize)>,
) -> usize {
    let mut sides = 0;
    let mut is_side = None;
    for (row, col) in iterator {
        if fields.contains(&(row, col)) {
            if is_side.is_none() {
                is_side = Some(Side::Prev);
            }
        } else if is_side.is_some() {
            is_side = None;
            sides += 1;
        }
    }
    if is_side.is_some() {
        sides += 1;
    }
    sides
}

fn move_coords(
    row: usize,
    col: usize,
    row_limit: usize,
    col_limit: usize,
) -> impl Iterator<Item = (usize, usize)> {
    MOVES.iter().filter_map(move |(row_change, col_change)| {
        let new_row = row as isize + row_change;
        if new_row < 0 || new_row >= row_limit as isize {
            return None;
        }
        let new_col = col as isize + col_change;
        if new_col < 0 || new_col >= col_limit as isize {
            return None;
        }
        Some((new_row as usize, new_col as usize))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "AAAA
BBCD
BBCC
EEEC";
    const SAMPLE2: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
    const SAMPLE3: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
    const SAMPLE4: &str = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
    const SAMPLE5: &str = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

    #[test]
    fn test_find_region_fields() {
        let plots = parse_plots(SAMPLE);

        let region_fields_a = find_region_fields('A', (0, 0), &plots, &mut HashSet::new());
        assert_eq!(
            region_fields_a,
            (
                HashSet::from([(0, 0), (0, 1), (0, 2), (0, 3)]),
                HashSet::from([0]),
                HashSet::from([0, 1, 2, 3])
            )
        );

        let region_fields_b = find_region_fields('B', (1, 0), &plots, &mut HashSet::new());
        assert_eq!(
            region_fields_b,
            (
                HashSet::from([(1, 0), (1, 1), (2, 0), (2, 1)]),
                HashSet::from([1, 2]),
                HashSet::from([0, 1,]),
            )
        );

        let region_fields_c = find_region_fields('C', (1, 2), &plots, &mut HashSet::new());
        assert_eq!(
            region_fields_c,
            (
                HashSet::from([(1, 2), (2, 2), (2, 3), (3, 3)]),
                HashSet::from([1, 2, 3]),
                HashSet::from([2, 3])
            )
        );

        let region_fields_d = find_region_fields('D', (1, 3), &plots, &mut HashSet::new());
        assert_eq!(
            region_fields_d,
            (
                HashSet::from([(1, 3)]),
                HashSet::from([1]),
                HashSet::from([3])
            )
        );

        let region_fields_e = find_region_fields('E', (3, 0), &plots, &mut HashSet::new());
        assert_eq!(
            region_fields_e,
            (
                HashSet::from([(3, 0), (3, 1), (3, 2)]),
                HashSet::from([3]),
                HashSet::from([0, 1, 2])
            )
        );
    }

    #[test]
    fn test_find_regions() {
        let plots1 = parse_plots(SAMPLE);
        let plots2 = parse_plots(SAMPLE2);
        let plots3 = parse_plots(SAMPLE3);

        let regions1 = find_regions(&plots1);
        assert_eq!(
            regions1,
            [
                (
                    'A',
                    HashSet::from([(0, 0), (0, 1), (0, 2), (0, 3)]),
                    HashSet::from([0]),
                    HashSet::from([0, 1, 2, 3])
                ),
                (
                    'B',
                    HashSet::from([(1, 0), (1, 1), (2, 0), (2, 1)]),
                    HashSet::from([1, 2]),
                    HashSet::from([0, 1])
                ),
                (
                    'C',
                    HashSet::from([(1, 2), (2, 2), (2, 3), (3, 3)]),
                    HashSet::from([1, 2, 3]),
                    HashSet::from([2, 3])
                ),
                (
                    'D',
                    HashSet::from([(1, 3)]),
                    HashSet::from([1]),
                    HashSet::from([3])
                ),
                (
                    'E',
                    HashSet::from([(3, 0), (3, 1), (3, 2)]),
                    HashSet::from([3]),
                    HashSet::from([0, 1, 2])
                )
            ]
        );

        let regions2 = find_regions(&plots2);

        assert_eq!(regions2.len(), 5);
        assert_eq!(regions2[0].0, 'O');
        assert_eq!(regions2[1].0, 'X');
        assert_eq!(regions2[2].0, 'X');
        assert_eq!(regions2[3].0, 'X');
        assert_eq!(regions2[4].0, 'X');

        let os: HashSet<(usize, usize)> = HashSet::from_iter(
            vec![
                (0, 0),
                (0, 1),
                (0, 2),
                (0, 3),
                (0, 4),
                (1, 0),
                (1, 2),
                (1, 4),
                (2, 0),
                (2, 1),
                (2, 2),
                (2, 3),
                (2, 4),
                (3, 0),
                (3, 2),
                (3, 4),
                (4, 0),
                (4, 1),
                (4, 2),
                (4, 3),
                (4, 4),
            ]
            .iter()
            .map(|v| *v),
        );

        assert_eq!(os.len(), regions2[0].1.len());
        for coord in regions2[0].1.iter() {
            assert_eq!(os.contains(&coord), true);
        }

        let regions3 = find_regions(&plots3);

        assert_eq!(regions3.len(), 11);
        assert_eq!(regions3[0].1.len(), 12);
        assert_eq!(regions3[1].1.len(), 4);
        assert_eq!(regions3[2].1.len(), 14);
        assert_eq!(regions3[3].1.len(), 10);
        assert_eq!(regions3[4].1.len(), 13);
        assert_eq!(regions3[5].1.len(), 11);
        assert_eq!(regions3[6].1.len(), 1);
        assert_eq!(regions3[7].1.len(), 13);
        assert_eq!(regions3[8].1.len(), 14);
        assert_eq!(regions3[9].1.len(), 5);
        assert_eq!(regions3[10].1.len(), 3);
    }

    #[test]
    fn test_region_price() {
        let plots1 = parse_plots(SAMPLE);
        let plots2 = parse_plots(SAMPLE2);
        let plots3 = parse_plots(SAMPLE3);

        let regions1 = find_regions(&plots1);
        let regions2 = find_regions(&plots2);
        let regions3 = find_regions(&plots3);

        assert_eq!(region_price(&regions1[0], region_perimeter), 40);
        assert_eq!(region_price(&regions1[1], region_perimeter), 32);
        assert_eq!(region_price(&regions1[2], region_perimeter), 40);
        assert_eq!(region_price(&regions1[3], region_perimeter), 4);
        assert_eq!(region_price(&regions1[4], region_perimeter), 24);

        assert_eq!(region_price(&regions2[0], region_perimeter), 756);
        assert_eq!(region_price(&regions2[1], region_perimeter), 4);
        assert_eq!(region_price(&regions2[2], region_perimeter), 4);
        assert_eq!(region_price(&regions2[3], region_perimeter), 4);

        assert_eq!(region_price(&regions3[0], region_perimeter), 216);
        assert_eq!(region_price(&regions3[1], region_perimeter), 32);
        assert_eq!(region_price(&regions3[2], region_perimeter), 392);
        assert_eq!(region_price(&regions3[3], region_perimeter), 180);
        assert_eq!(region_price(&regions3[4], region_perimeter), 260);
        assert_eq!(region_price(&regions3[5], region_perimeter), 220);
        assert_eq!(region_price(&regions3[6], region_perimeter), 4);
        assert_eq!(region_price(&regions3[7], region_perimeter), 234);
        assert_eq!(region_price(&regions3[8], region_perimeter), 308);
        assert_eq!(region_price(&regions3[9], region_perimeter), 60);
        assert_eq!(region_price(&regions3[10], region_perimeter), 24);
    }

    #[test]
    fn test_regions_price() {
        let plots1 = parse_plots(SAMPLE);
        let plots2 = parse_plots(SAMPLE2);
        let plots3 = parse_plots(SAMPLE3);

        let regions1 = find_regions(&plots1);
        let regions2 = find_regions(&plots2);
        let regions3 = find_regions(&plots3);

        assert_eq!(regions_price(&regions1), 140);
        assert_eq!(regions_price(&regions2), 772);
        assert_eq!(regions_price(&regions3), 1930);
    }

    #[test]
    fn test_region_perimeter() {
        let plots1 = parse_plots(SAMPLE);
        let plots2 = parse_plots(SAMPLE2);

        let regions1 = find_regions(&plots1);
        let regions2 = find_regions(&plots2);

        assert_eq!(region_perimeter(&regions1[0]), 10);
        assert_eq!(region_perimeter(&regions1[1]), 8);
        assert_eq!(region_perimeter(&regions1[2]), 10);
        assert_eq!(region_perimeter(&regions1[3]), 4);
        assert_eq!(region_perimeter(&regions1[4]), 8);

        assert_eq!(region_perimeter(&regions2[0]), 36);
    }

    #[test]
    fn test_region_sides() {
        let plots1 = parse_plots(SAMPLE);
        let plots2 = parse_plots(SAMPLE2);
        let plots3 = parse_plots(SAMPLE3);
        let plots4 = parse_plots(SAMPLE4);
        let plots5 = parse_plots(SAMPLE5);

        let regions1 = find_regions(&plots1);
        let regions2 = find_regions(&plots2);
        let regions3 = find_regions(&plots3);
        let regions4 = find_regions(&plots4);
        let regions5 = find_regions(&plots5);

        assert_eq!(region_sides(&regions1[0]), 4);
        assert_eq!(region_sides(&regions1[1]), 4);
        assert_eq!(region_sides(&regions1[2]), 8);
        assert_eq!(region_sides(&regions1[3]), 4);
        assert_eq!(region_sides(&regions1[4]), 4);

        assert_eq!(region_sides(&regions2[0]), 20);
        assert_eq!(region_sides(&regions2[1]), 4);
        assert_eq!(region_sides(&regions2[2]), 4);
        assert_eq!(region_sides(&regions2[3]), 4);
        assert_eq!(region_sides(&regions2[4]), 4);

        assert_eq!(region_sides(&regions4[0]), 12);

        assert_eq!(region_sides(&regions5[0]), 12);
        assert_eq!(region_sides(&regions5[1]), 4);
        assert_eq!(region_sides(&regions5[2]), 4);

        assert_eq!(region_sides(&regions3[0]), 10);
        assert_eq!(region_sides(&regions3[1]), 4);
        assert_eq!(region_sides(&regions3[2]), 22);
        assert_eq!(region_sides(&regions3[3]), 12);
        assert_eq!(region_sides(&regions3[4]), 10);
        assert_eq!(region_sides(&regions3[5]), 12);
        assert_eq!(region_sides(&regions3[6]), 4);
        assert_eq!(region_sides(&regions3[7]), 8);
        assert_eq!(region_sides(&regions3[8]), 16);
        assert_eq!(region_sides(&regions3[9]), 6);
        assert_eq!(region_sides(&regions3[10]), 6);
    }

    #[test]
    fn test_regions_discount_price() {
        let plots1 = parse_plots(SAMPLE);
        let plots2 = parse_plots(SAMPLE2);
        let plots3 = parse_plots(SAMPLE3);
        let plots4 = parse_plots(SAMPLE4);
        let plots5 = parse_plots(SAMPLE5);

        let regions1 = find_regions(&plots1);
        let regions2 = find_regions(&plots2);
        let regions3 = find_regions(&plots3);
        let regions4 = find_regions(&plots4);
        let regions5 = find_regions(&plots5);

        assert_eq!(regions_discount_price(&regions1), 80);
        assert_eq!(regions_discount_price(&regions2), 436);
        assert_eq!(regions_discount_price(&regions4), 236);
        assert_eq!(regions_discount_price(&regions5), 368);
        assert_eq!(regions_discount_price(&regions3), 1206);
    }

    #[test]
    fn test_parse_plots() {
        let result = parse_plots(SAMPLE);

        assert_eq!(
            result,
            [
                vec!['A', 'A', 'A', 'A'],
                vec!['B', 'B', 'C', 'D'],
                vec!['B', 'B', 'C', 'C'],
                vec!['E', 'E', 'E', 'C']
            ]
        );
    }

    #[test]
    fn test_part_1_sample() {
        // 1930
    }

    #[test]
    fn test_part_2_sample() {}
}
