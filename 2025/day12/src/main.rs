use std::collections::HashSet;
use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq)]
struct Size {
    rows: usize,
    cols: usize,
}

type Present = (usize, Shape);
type Shape = Vec<Vec<char>>;
type Tree = (Size, Vec<usize>);

fn main() {
    let input = Path::new("2025/day12/src/input.txt");
    let (_, trees) = parse_input(&fs::read_to_string(input).unwrap());

    println!("Step 1");
    println!(
        "Regions that can fit required presents: {:?}",
        count_trees_with_fitting_presents(&trees)
    );

    println!("Step 2");
    println!("Calculating...");
}

fn count_trees_with_fitting_presents(trees: &[Tree]) -> usize {
    trees.iter().filter(|tree| can_fit(tree)).count()
}

fn can_fit(tree: &Tree) -> bool {
    let (grid_size, required_presents) = tree;

    let free_space = grid_size.rows * grid_size.cols;

    let minimum_space_approximation: usize = required_presents
        .iter()
        .map(|required| {
            let pairs = required.div_euclid(2);
            let left = required - pairs * 2;
            pairs * 16 + left * 9
        })
        .sum();

    minimum_space_approximation < free_space
}

fn parse_input(input: &str) -> (Vec<Present>, Vec<Tree>) {
    let mut lines = input.lines();

    let mut presents = Vec::new();
    let mut trees = Vec::new();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            continue;
        } else if line.chars().nth(1).unwrap() == ':' {
            let id: usize = line.trim_end_matches(':').parse().unwrap();

            let present: Vec<Vec<char>> = (0..3)
                .map(|_| lines.next().unwrap().chars().collect())
                .collect();
            presents.push((id, present));
        } else if !line.is_empty() {
            let parts = line.split_ascii_whitespace();
            let mut size = Vec::new();
            for values in parts.clone().take(1) {
                let (cols, rows) = values.trim_end_matches(':').split_once('x').unwrap();
                size.push(cols.parse::<usize>().unwrap());
                size.push(rows.parse::<usize>().unwrap());
            }
            let presents: Vec<usize> = parts
                .skip(1)
                .map(|val| val.parse::<usize>().unwrap())
                .collect();

            trees.push((
                Size {
                    rows: size[1],
                    cols: size[0],
                },
                presents,
            ));
        }
    }
    (presents, trees)
}

fn shape_variants(shape: Shape) -> Vec<Shape> {
    let mut unique = HashSet::new();
    let mut variants = Vec::new();
    unique.insert(shape.clone());
    variants.push(shape.clone());
    let mut cur_shape = shape;
    for _ in 0..3 {
        let rotated = rotate(&cur_shape);
        if !unique.contains(&rotated) {
            variants.push(rotated.clone());
            unique.insert(rotated.clone());
        }
        cur_shape = rotated;
    }
    let mut flipped = flip(&cur_shape);
    if !unique.contains(&flipped) {
        variants.push(flipped.clone());
        unique.insert(flipped.clone());
    }
    for _ in 0..3 {
        let rotated = rotate(&flipped);
        if !unique.contains(&rotated) {
            variants.push(rotated.clone());
            unique.insert(rotated.clone());
        }
        flipped = rotated;
    }
    variants
}

fn flip(shape: &Shape) -> Shape {
    let mut flipped: Shape = (0..shape.len())
        .map(|_row| (0..shape[0].len()).map(|_col| '.').collect())
        .collect();

    let last_row_index = shape.len() - 1;
    for (row_no, row) in shape.iter().enumerate() {
        for (col_no, col) in row.iter().enumerate() {
            flipped[last_row_index - row_no][col_no] = *col;
        }
    }

    flipped
}

fn rotate(shape: &Shape) -> Shape {
    let mut rotated: Shape = (0..shape.len())
        .map(|_row| (0..shape[0].len()).map(|_col| '.').collect())
        .collect();
    let last_row_index = shape.len() - 1;
    for (row_no, row) in shape.iter().enumerate() {
        for (col_no, col) in row.iter().enumerate() {
            rotated[col_no][last_row_index - row_no] = *col;
        }
    }

    rotated
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "0:\n###\n##.\n##.\n
1:\n###\n##.\n.##\n
2:\n.##\n###\n##.\n
3:\n##.\n###\n##.\n
4:\n###\n#..\n###\n
5:\n###\n.#.\n###\n
4x4: 0 0 0 0 2 0\n12x5: 1 0 1 0 2 2\n12x5: 1 0 1 0 3 2";

    #[test]
    fn test_count_trees_with_fitting_presents() {
        let (_, trees) = parse_input(SAMPLE);

        let trees_with_fitting_presents = count_trees_with_fitting_presents(&trees);
        assert_eq!(trees_with_fitting_presents, 2);
    }

    #[test]
    fn test_parse_input() {
        let (presents, trees) = parse_input(SAMPLE);

        assert_eq!(
            presents,
            Vec::from([
                (
                    0,
                    Vec::from([
                        Vec::from(['#', '#', '#']),
                        Vec::from(['#', '#', '.']),
                        Vec::from(['#', '#', '.'])
                    ])
                ),
                (
                    1,
                    Vec::from([
                        Vec::from(['#', '#', '#']),
                        Vec::from(['#', '#', '.']),
                        Vec::from(['.', '#', '#']),
                    ])
                ),
                (
                    2,
                    Vec::from([
                        Vec::from(['.', '#', '#']),
                        Vec::from(['#', '#', '#']),
                        Vec::from(['#', '#', '.']),
                    ])
                ),
                (
                    3,
                    Vec::from([
                        Vec::from(['#', '#', '.']),
                        Vec::from(['#', '#', '#']),
                        Vec::from(['#', '#', '.']),
                    ])
                ),
                (
                    4,
                    Vec::from([
                        Vec::from(['#', '#', '#']),
                        Vec::from(['#', '.', '.']),
                        Vec::from(['#', '#', '#']),
                    ])
                ),
                (
                    5,
                    Vec::from([
                        Vec::from(['#', '#', '#']),
                        Vec::from(['.', '#', '.']),
                        Vec::from(['#', '#', '#']),
                    ])
                ),
            ])
        );

        assert_eq!(
            trees,
            Vec::from([
                (Size { rows: 4, cols: 4 }, Vec::from([0, 0, 0, 0, 2, 0])),
                (Size { rows: 5, cols: 12 }, Vec::from([1, 0, 1, 0, 2, 2])),
                (Size { rows: 5, cols: 12 }, Vec::from([1, 0, 1, 0, 3, 2])),
            ])
        )
    }

    #[test]
    fn test_shape_variants() {
        let (shapes, _) = parse_input(SAMPLE);

        let variants: Vec<Vec<Shape>> = shapes
            .iter()
            .map(|(_id, shape)| shape_variants(shape.to_vec()))
            .collect();

        assert_eq!(
            variants[0],
            Vec::from([
                Vec::from([
                    Vec::from(['#', '#', '#']),
                    Vec::from(['#', '#', '.']),
                    Vec::from(['#', '#', '.'])
                ]),
                Vec::from([
                    Vec::from(['#', '#', '#']),
                    Vec::from(['#', '#', '#']),
                    Vec::from(['.', '.', '#'])
                ]),
                Vec::from([
                    Vec::from(['.', '#', '#']),
                    Vec::from(['.', '#', '#']),
                    Vec::from(['#', '#', '#'])
                ]),
                Vec::from([
                    Vec::from(['#', '.', '.']),
                    Vec::from(['#', '#', '#']),
                    Vec::from(['#', '#', '#'])
                ]),
                Vec::from([
                    Vec::from(['#', '#', '#']),
                    Vec::from(['#', '#', '#']),
                    Vec::from(['#', '.', '.'])
                ]),
                Vec::from([
                    Vec::from(['#', '#', '#']),
                    Vec::from(['.', '#', '#']),
                    Vec::from(['.', '#', '#'])
                ]),
                Vec::from([
                    Vec::from(['.', '.', '#']),
                    Vec::from(['#', '#', '#']),
                    Vec::from(['#', '#', '#'])
                ]),
                Vec::from([
                    Vec::from(['#', '#', '.']),
                    Vec::from(['#', '#', '.']),
                    Vec::from(['#', '#', '#'])
                ]),
            ])
        );
        assert_eq!(
            variants[1],
            Vec::from([
                Vec::from([
                    Vec::from(['#', '#', '#']),
                    Vec::from(['#', '#', '.']),
                    Vec::from(['.', '#', '#']),
                ]),
                Vec::from([
                    Vec::from(['.', '#', '#']),
                    Vec::from(['#', '#', '#']),
                    Vec::from(['#', '.', '#']),
                ]),
                Vec::from([
                    Vec::from(['#', '#', '.']),
                    Vec::from(['.', '#', '#']),
                    Vec::from(['#', '#', '#']),
                ]),
                Vec::from([
                    Vec::from(['#', '.', '#']),
                    Vec::from(['#', '#', '#']),
                    Vec::from(['#', '#', '.']),
                ]),
                Vec::from([
                    Vec::from(['#', '#', '.']),
                    Vec::from(['#', '#', '#']),
                    Vec::from(['#', '.', '#']),
                ]),
                Vec::from([
                    Vec::from(['#', '#', '#']),
                    Vec::from(['.', '#', '#']),
                    Vec::from(['#', '#', '.']),
                ]),
                Vec::from([
                    Vec::from(['#', '.', '#']),
                    Vec::from(['#', '#', '#']),
                    Vec::from(['.', '#', '#']),
                ]),
                Vec::from([
                    Vec::from(['.', '#', '#']),
                    Vec::from(['#', '#', '.']),
                    Vec::from(['#', '#', '#']),
                ]),
            ])
        );
        assert_eq!(
            variants[2],
            Vec::from([
                Vec::from([
                    Vec::from(['.', '#', '#']),
                    Vec::from(['#', '#', '#']),
                    Vec::from(['#', '#', '.']),
                ]),
                Vec::from([
                    Vec::from(['#', '#', '.']),
                    Vec::from(['#', '#', '#']),
                    Vec::from(['.', '#', '#']),
                ]),
            ])
        );

        assert_eq!(
            variants[3],
            Vec::from([
                Vec::from([
                    Vec::from(['#', '#', '.']),
                    Vec::from(['#', '#', '#']),
                    Vec::from(['#', '#', '.']),
                ]),
                Vec::from([
                    Vec::from(['#', '#', '#']),
                    Vec::from(['#', '#', '#']),
                    Vec::from(['.', '#', '.']),
                ]),
                Vec::from([
                    Vec::from(['.', '#', '#']),
                    Vec::from(['#', '#', '#']),
                    Vec::from(['.', '#', '#']),
                ]),
                Vec::from([
                    Vec::from(['.', '#', '.']),
                    Vec::from(['#', '#', '#']),
                    Vec::from(['#', '#', '#']),
                ]),
            ])
        );

        assert_eq!(
            variants[4],
            Vec::from([
                Vec::from([
                    Vec::from(['#', '#', '#']),
                    Vec::from(['#', '.', '.']),
                    Vec::from(['#', '#', '#']),
                ]),
                Vec::from([
                    Vec::from(['#', '#', '#']),
                    Vec::from(['#', '.', '#']),
                    Vec::from(['#', '.', '#']),
                ]),
                Vec::from([
                    Vec::from(['#', '#', '#']),
                    Vec::from(['.', '.', '#']),
                    Vec::from(['#', '#', '#']),
                ]),
                Vec::from([
                    Vec::from(['#', '.', '#']),
                    Vec::from(['#', '.', '#']),
                    Vec::from(['#', '#', '#']),
                ]),
            ])
        );
        assert_eq!(
            variants[5],
            Vec::from([
                Vec::from([
                    Vec::from(['#', '#', '#']),
                    Vec::from(['.', '#', '.']),
                    Vec::from(['#', '#', '#']),
                ]),
                Vec::from([
                    Vec::from(['#', '.', '#']),
                    Vec::from(['#', '#', '#']),
                    Vec::from(['#', '.', '#']),
                ]),
            ])
        )
    }
}
