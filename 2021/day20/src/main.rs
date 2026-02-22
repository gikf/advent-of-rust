use std::collections::HashSet;
use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq)]
enum Pixel {
    Light,
    Dark,
}

impl From<char> for Pixel {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Light,
            '.' => Self::Dark,
            _ => unimplemented!(),
        }
    }
}

const PART1_ENHANCE: usize = 2;
const PART2_ENHANCE: usize = 50;

fn main() {
    let input = Path::new("2021/day20/src/input.txt");
    let (algorithm, lighted_pixels) = parse_input(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Number of lit pixels after {:?} enhances: {:?}",
        PART1_ENHANCE,
        enhance_image(&lighted_pixels, &algorithm, PART1_ENHANCE)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Number of lit pixels after {:?} enhances: {:?}",
        PART2_ENHANCE,
        enhance_image(&lighted_pixels, &algorithm, PART2_ENHANCE)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_input(input: &str) -> (Vec<Pixel>, Vec<(isize, isize)>) {
    let mut lines = input.lines();
    let algorithm: Vec<_> = lines.next().unwrap().chars().map(Pixel::from).collect();
    lines.next();

    let mut lighted_pixels = Vec::new();
    lines.enumerate().for_each(|(row_no, line)| {
        line.char_indices().for_each(|(col_no, ch)| {
            if Pixel::from(ch) == Pixel::Light {
                lighted_pixels.push((row_no as isize, col_no as isize));
            }
        })
    });

    (algorithm, lighted_pixels)
}

const NEIGHBOURS: [(isize, isize); 9] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 0),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn enhance_image(
    lighted_pixels: &[(isize, isize)],
    algorithm: &[Pixel],
    enhance_count: usize,
) -> usize {
    let mut image = HashSet::from_iter(lighted_pixels.iter().copied());
    let ((mut min_row, mut max_row), (mut min_col, mut max_col)) = image_ranges(&image);

    let mut outside_value = 0;

    for _ in 0..enhance_count {
        let mut next_image = HashSet::with_capacity(image.len() + (max_col * max_row) as usize);

        for row in (min_row - 1)..=(max_row + 1) {
            for col in (min_col - 1)..=(max_col + 1) {
                let binary: Vec<_> = NEIGHBOURS
                    .iter()
                    .map(|(row_change, col_change)| {
                        let new_row = row + row_change;
                        let new_col = col + col_change;

                        if new_row < min_row
                            || new_col < min_col
                            || new_row > max_row
                            || new_col > max_col
                        {
                            outside_value
                        } else if image.contains(&(new_row, new_col)) {
                            1
                        } else {
                            0
                        }
                    })
                    .collect();
                let decimal = binary_to_decimal(&binary);

                if algorithm[decimal] == Pixel::Light {
                    next_image.insert((row, col));
                }
            }
        }
        let outside_grid = vec![outside_value; 9];
        outside_value = if matches!(algorithm[binary_to_decimal(&outside_grid)], Pixel::Light) {
            1
        } else {
            0
        };

        image = next_image;
        min_row -= 1;
        max_row += 1;
        min_col -= 1;
        max_col += 1;
    }
    image.len()
}

fn binary_to_decimal(binary: &[usize]) -> usize {
    binary
        .iter()
        .rev()
        .enumerate()
        .map(|(offset, bit)| bit * 2_usize.pow(offset as u32))
        .sum::<usize>()
}

fn image_ranges(lighted_pixels: &HashSet<(isize, isize)>) -> ((isize, isize), (isize, isize)) {
    let mut min_row = isize::MAX;
    let mut max_row = isize::MIN;
    let mut min_col = isize::MAX;
    let mut max_col = isize::MIN;

    for (row, col) in lighted_pixels {
        min_row = min_row.min(*row);
        max_row = max_row.max(*row);
        min_col = min_col.min(*col);
        max_col = max_col.max(*col);
    }

    ((min_row, max_row), (min_col, max_col))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

    #[test]
    fn test_part_1_sample() {
        let (algorithm, lighted_pixels) = parse_input(SAMPLE);

        assert_eq!(
            enhance_image(&lighted_pixels, &algorithm, PART1_ENHANCE),
            35
        );
    }

    #[test]
    fn test_part_2_sample() {
        let (algorithm, lighted_pixels) = parse_input(SAMPLE);

        assert_eq!(
            enhance_image(&lighted_pixels, &algorithm, PART2_ENHANCE),
            3351
        );
    }
}
