use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq)]
struct Tile {
    x: isize,
    y: isize,
}

impl Tile {
    fn area(&self, other: &Tile) -> isize {
        let rows = self.x.max(other.x) - self.x.min(other.x) + 1;
        let cols = self.y.max(other.y) - self.y.min(other.y) + 1;

        rows * cols
    }
}

#[derive(Debug, PartialEq)]
struct Polygon<'a> {
    tiles: &'a [Tile],
    _calculated: HashMap<(isize, isize), bool>,
}

impl<'a> Polygon<'a> {
    fn is_rectangle_inside(&mut self, tile_a: Tile, tile_b: Tile) -> bool {
        let row_start = tile_a.y.min(tile_b.y);
        let row_end = tile_a.y.max(tile_b.y);
        let col_start = tile_a.x.min(tile_b.x);
        let col_end = tile_a.x.max(tile_b.x);

        for row in row_start..row_end {
            if !self.is_point_inside(Tile {
                x: col_start,
                y: row,
            }) {
                return false;
            }
        }
        for row in row_start..row_end {
            if !self.is_point_inside(Tile { x: col_end, y: row }) {
                return false;
            }
        }

        for col in col_start..col_end {
            if !self.is_point_inside(Tile {
                x: col,
                y: row_start,
            }) {
                return false;
            }
        }

        for col in col_start..col_end {
            if !self.is_point_inside(Tile { x: col, y: row_end }) {
                return false;
            }
        }

        true
    }

    fn is_point_inside(&mut self, point: Tile) -> bool {
        // Based on https://www.xjavascript.com/blog/check-if-point-is-inside-a-polygon/
        if let Some(result) = self._calculated.get(&(point.x, point.y)) {
            return *result;
        }

        let mut inside = false;
        let length = self.tiles.len();
        for index in 0..length {
            let a = &self.tiles[index];
            let b = &self.tiles[(index + 1) % length];

            let ax = a.x as f64;
            let ay = a.y as f64;
            let bx = b.x as f64;
            let by = b.y as f64;
            let px = point.x as f64;
            let py = point.y as f64;

            if self.is_on_segment(&point, a, b) {
                return true;
            }

            if (ay > py + f64::EPSILON) != (by > py + f64::EPSILON) {
                if (by - ay).abs() < f64::EPSILON {
                    continue;
                }

                let x_intersect = ax + (py - ay) / (by - ay) * (bx - ax);

                if px < x_intersect - f64::EPSILON {
                    inside = !inside;
                }
            }
        }
        self._calculated.insert((point.x, point.y), inside);
        inside
    }

    fn is_on_segment(&self, point: &Tile, tile_a: &Tile, tile_b: &Tile) -> bool {
        let cross = (point.x - tile_a.x) * (tile_b.y - tile_a.y)
            - (point.y - tile_a.y) * (tile_b.x - tile_a.x);

        if cross.abs() as f64 > f64::EPSILON {
            return false;
        }

        let min_x = tile_a.x.min(tile_b.x) as f64 - f64::EPSILON;
        let max_x = tile_a.x.max(tile_b.x) as f64 + f64::EPSILON;
        let min_y = tile_a.y.min(tile_b.y) as f64 - f64::EPSILON;
        let max_y = tile_a.y.max(tile_b.y) as f64 + f64::EPSILON;
        let px = point.x as f64;
        let py = point.y as f64;

        min_x <= px && px <= max_x && min_y <= py && py <= max_y
    }
}

fn main() {
    let input = Path::new("2025/day09/src/input.txt");
    let tiles = parse_tiles(&fs::read_to_string(input).unwrap());

    println!("Step 1");
    println!(
        "Largest rectangle area: {:?}",
        largest_rectangle_area(&tiles)
    );

    let areas = get_areas(&tiles);
    println!("Step 2");
    println!("Largest rectangle using only red and green tiles {:?}", largest_contained_rectangle(&areas, &tiles).unwrap());
}

fn parse_tiles(input: &str) -> Vec<Tile> {
    input
        .lines()
        .flat_map(|line| {
            line.split_once(',').map(|(x, y)| Tile {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            })
        })
        .collect()
}

fn get_areas(tiles: &[Tile]) -> Vec<(isize, (&Tile, &Tile))> {
    let mut areas = Vec::new();
    for (index, tile_a) in (0..(tiles.len() - 1)).zip(tiles.iter()) {
        for tile_b in tiles.iter().skip(index + 1) {
            areas.push((tile_a.area(tile_b), (tile_a, tile_b)));
        }
    }

    areas.sort_by_key(|(area, _)| *area);
    areas.reverse();

    areas
}

fn largest_rectangle_area(tiles: &[Tile]) -> isize {
    let (largest_area, _) = get_areas(tiles)[0];
    largest_area
}

fn largest_contained_rectangle(areas: &[(isize, (&Tile, &Tile))], tiles: &[Tile]) -> Option<isize> {
    let (compacted, compacted_x, compacted_y) = compact_tiles(tiles);

    let mut polygon = Polygon { tiles: &compacted, _calculated: HashMap::new() };

    let areas_compacted = areas.iter().map(|(area, (a, b))| {
        let compacted_a = Tile {
            x: *compacted_x.get(&a.x).unwrap(),
            y: *compacted_y.get(&a.y).unwrap(),
        };
        let compacted_b = Tile {
            x: *compacted_x.get(&b.x).unwrap(),
            y: *compacted_y.get(&b.y).unwrap(),
        };
        (area, (a, b, compacted_a, compacted_b))
    });

    for (area, (_, _, compacted_a, compacted_b)) in areas_compacted {
        if polygon.is_rectangle_inside(compacted_a, compacted_b) {
            return Some(*area);
        }
    }
    None
}

fn compact_tiles(tiles: &[Tile]) -> (Vec<Tile>, HashMap<isize, isize>, HashMap<isize, isize>) {
    // Compacting idea from reddit
    let mut unique_x = BTreeSet::new();
    let mut unique_y = BTreeSet::new();

    for tile in tiles {
        unique_x.insert(tile.x);
        unique_y.insert(tile.y);
    }

    let mut compacted_x = HashMap::new();

    for (index, x) in unique_x.iter().enumerate() {
        compacted_x.insert(*x, index as isize);
    }

    let mut compacted_y = HashMap::new();
    for (index, y) in unique_y.iter().enumerate() {
        compacted_y.insert(*y, index as isize);
    }

    let compacted_tiles = tiles
        .iter()
        .map(|tile| Tile {
            x: *compacted_x.get(&tile.x).unwrap(),
            y: *compacted_y.get(&tile.y).unwrap(),
        })
        .collect();

    (compacted_tiles, compacted_x, compacted_y)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3";

    #[test]
    fn test_parse_tiles() {}

    #[test]
    fn test_largest_rectangle_step_1() {
        let tiles = parse_tiles(SAMPLE);
        let result = largest_rectangle_area(&tiles);

        assert_eq!(result, 50);
    }

    #[test]
    fn test_largest_contained_rectangle_step_2() {
        let tiles = parse_tiles(SAMPLE);

        let areas = get_areas(&tiles);
        let result = largest_contained_rectangle(&areas, &tiles).unwrap();

        assert_eq!(result, 24);
    }
}
