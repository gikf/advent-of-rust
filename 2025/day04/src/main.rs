use std::fs;
use std::path::Path;

#[derive(Clone, Debug, PartialEq)]
enum Field {
    Roll,
    Empty,
}

const NEIGHBOUR_OFFSETS: [(i32, i32); 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

fn main() {
    let input = Path::new("2025/day04/src/input.txt");
    let grid = parse_grid(&fs::read_to_string(input).unwrap());

    println!("Step 1");
    println!("Accessible rolls: {}", count_accessible_rolls(&grid));

    println!("Step 2");
    println!("Rolls removed in total: {}", count_removable_rolls(&grid));
}

fn parse_grid(input: &str) -> Vec<Vec<Field>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|field| match field {
                    '.' => Field::Empty,
                    '@' => Field::Roll,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn neighbour_coordinates(
    row: usize,
    col: usize,
    max_row: usize,
    max_col: usize,
) -> Vec<(usize, usize)> {
    NEIGHBOUR_OFFSETS
        .iter()
        .filter_map(|(row_change, col_change)| {
            match (row, col, row_change, col_change, max_row, max_col) {
                (0, _, -1, _, _, _) | (_, 0, _, -1, _, _) => None,
                (coord, _, 1, _, max_coord, _) | (_, coord, _, 1, _, max_coord)
                    if coord + 1 == max_coord =>
                {
                    None
                }
                _ => Some((
                    ((row as i32) + row_change) as usize,
                    ((col as i32) + col_change) as usize,
                )),
            }
        })
        .collect()
}

fn count_accessible_rolls(grid: &[Vec<Field>]) -> usize {
    get_accessible_rolls(grid).iter().len()
}

fn is_roll_accessible(grid: &[Vec<Field>], row_no: usize, col_no: usize) -> bool {
    let neighbours = neighbour_coordinates(row_no, col_no, grid.len(), grid[0].len());
    let roll_neighbours = neighbours
        .iter()
        .filter(|(n_row, n_col)| grid[*n_row][*n_col] == Field::Roll)
        .collect::<Vec<_>>()
        .len();
    roll_neighbours < 4
}

fn get_accessible_rolls(grid: &[Vec<Field>]) -> Vec<(usize, usize)> {
    grid.iter()
        .enumerate()
        .flat_map(|(row_no, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(col_no, field)| {
                    if *field == Field::Empty || !is_roll_accessible(grid, row_no, col_no) {
                        None
                    } else {
                        Some((row_no, col_no))
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn copy_grid(grid: &[Vec<Field>]) -> Vec<Vec<Field>> {
    let mut new_grid = Vec::new();

    for row in grid.iter() {
        let mut new_row = Vec::new();
        for field in row.iter() {
            new_row.push(field.clone());
        }
        new_grid.push(new_row);
    }
    new_grid
}

fn count_removable_rolls(grid: &[Vec<Field>]) -> usize {
    let mut removable_grid = copy_grid(grid);
    let mut removed_rolls = 0;
    loop {
        let removable = get_accessible_rolls(&removable_grid);
        if removable.is_empty() {
            break;
        }

        removed_rolls += removable.len();
        for (row_no, col_no) in removable {
            removable_grid[row_no][col_no] = Field::Empty;
        }
    }
    removed_rolls
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";

    #[test]
    fn test_parse_grid() {
        let input = "..@\n@@@\n@@@";
        let grid = parse_grid(input);
        assert_eq!(
            grid,
            [
                [Field::Empty, Field::Empty, Field::Roll],
                [Field::Roll, Field::Roll, Field::Roll],
                [Field::Roll, Field::Roll, Field::Roll]
            ]
        )
    }

    #[test]
    fn test_step_1_sample() {
        let grid = parse_grid(SAMPLE);
        let accessible_rolls = count_accessible_rolls(&grid);
        assert_eq!(accessible_rolls, 13);
    }

    #[test]
    fn test_step_2_sample() {
        let grid = parse_grid(SAMPLE);
        let removed_rolls = count_removable_rolls(&grid);
        assert_eq!(removed_rolls, 43);
    }
}
