use std::fs;
use std::path::Path;

type Coordinates = (usize, usize);

fn main() {
    let input = Path::new("2021/day25/src/input.txt");
    let (size, east_cucumbers, south_cucumbers) =
        parse_seafloor(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Steps to cucumber-lock: {:?}",
        find_cucumber_lock(size, east_cucumbers, south_cucumbers)
    );
    println!("In {:?}", part1.elapsed());
}

fn parse_seafloor(input: &str) -> ((usize, usize), Vec<Coordinates>, Vec<Coordinates>) {
    let mut east_cucumbers = Vec::new();
    let mut south_cucumbers = Vec::new();

    let mut cols = None;
    let mut rows = 0;
    for line in input.lines() {
        if cols.is_none() {
            cols = Some(line.len());
        }

        for (col_no, c) in line.char_indices() {
            match c {
                '>' => {
                    east_cucumbers.push((rows, col_no));
                }
                'v' => {
                    south_cucumbers.push((rows, col_no));
                }
                _ => {}
            }
        }
        rows += 1;
    }

    ((rows, cols.unwrap()), east_cucumbers, south_cucumbers)
}

fn find_cucumber_lock(
    (rows, cols): (usize, usize),
    east_cucumbers: Vec<Coordinates>,
    south_cucumbers: Vec<Coordinates>,
) -> usize {
    let mut east_cucumbers = east_cucumbers;
    let mut south_cucumbers = south_cucumbers;
    let mut east_positions = vec![0; cols * rows];
    for (r, c) in &east_cucumbers {
        east_positions[*r * cols + *c] = 1_u8;
    }

    let mut south_positions = vec![0; cols * rows];
    for (r, c) in &south_cucumbers {
        south_positions[*r * cols + *c] = 1_u8;
    }
    let mut new_east = Vec::with_capacity(east_cucumbers.len());
    let mut new_south = Vec::with_capacity(south_cucumbers.len());

    for step in 1.. {
        let mut next_east_pos = vec![0; east_positions.len()];
        let mut next_south_pos = vec![0; south_positions.len()];
        new_east.clear();
        new_south.clear();

        let mut moved = false;
        for coords in east_cucumbers.drain(..) {
            let new_coords = east_cucumber_move(coords, cols);

            let next_coordinates = if can_move(new_coords, &east_positions, &south_positions, cols)
            {
                moved = true;
                new_coords
            } else {
                coords
            };
            let index = next_coordinates.0 * cols + next_coordinates.1;
            next_east_pos[index] = 1;
            new_east.push(next_coordinates);
        }

        for coords in south_cucumbers.drain(..) {
            let new_coords = south_cucumber_move(coords, rows);

            let next_coordinates = if can_move(new_coords, &next_east_pos, &south_positions, cols) {
                moved = true;
                new_coords
            } else {
                coords
            };
            let index = next_coordinates.0 * cols + next_coordinates.1;
            next_south_pos[index] = 1;
            new_south.push(next_coordinates);
        }

        if !moved {
            return step;
        }

        east_cucumbers.extend(&new_east);
        east_positions = next_east_pos;
        south_positions = next_south_pos;
        south_cucumbers.extend(&new_south);
    }

    unreachable!()
}

fn east_cucumber_move(coordinates: Coordinates, limit: usize) -> (usize, usize) {
    (coordinates.0, (coordinates.1 + 1).rem_euclid(limit))
}

fn south_cucumber_move(coordinates: Coordinates, limit: usize) -> (usize, usize) {
    ((coordinates.0 + 1).rem_euclid(limit), coordinates.1)
}

fn can_move(
    coordinates: Coordinates,
    cucumbers1: &[u8],
    cucumbers2: &[u8],
    col_limit: usize,
) -> bool {
    let index = coordinates.0 * col_limit + coordinates.1;
    cucumbers1[index] == 0 && cucumbers2[index] == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

    #[test]
    fn test_part_1_sample() {
        let (size, east_cucumbers, south_cucumbers) = parse_seafloor(SAMPLE);

        assert_eq!(
            find_cucumber_lock(size, east_cucumbers, south_cucumbers),
            58
        );
    }
}
