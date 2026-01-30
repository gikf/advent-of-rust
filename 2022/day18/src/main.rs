use std::collections::{HashSet, VecDeque};
use std::fs;
use std::path::Path;

const MOVES: [(isize, isize, isize); 6] = [
    (-1, 0, 0),
    (1, 0, 0),
    (0, -1, 0),
    (0, 1, 0),
    (0, 0, -1),
    (0, 0, 1),
];

fn main() {
    let input = Path::new("2022/day18/src/input.txt");
    let cubes = parse_cubes(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Surface area of droplets: {:?}",
        count_non_connected_sides(&cubes)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!("Exterior surface area: {:?}", exterior_surface(&cubes));
    println!("In {:?}", part2.elapsed());
}

fn parse_cubes(input: &str) -> HashSet<(isize, isize, isize)> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(',').map(|v| v.parse().unwrap());
            (
                split.next().unwrap(),
                split.next().unwrap(),
                split.next().unwrap(),
            )
        })
        .collect()
}

fn count_non_connected_sides(cubes: &HashSet<(isize, isize, isize)>) -> usize {
    let mut total_surface = 0;
    for (x, y, z) in cubes {
        let mut sides = 6;

        for (x_change, y_change, z_change) in &MOVES {
            let next_x = x + x_change;
            let next_y = y + y_change;
            let next_z = z + z_change;

            if cubes.contains(&(next_x, next_y, next_z)) {
                sides -= 1;
            }
        }
        total_surface += sides;
    }
    total_surface
}

fn exterior_surface(cubes: &HashSet<(isize, isize, isize)>) -> usize {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;
    for (x, y, z) in cubes {
        max_x = max_x.max(*x);
        max_y = max_y.max(*y);
        max_z = max_z.max(*z);
    }

    let x_bound = max_x + 1;
    let y_bound = max_y + 1;
    let z_bound = max_z + 1;

    let cubes_set: HashSet<&(isize, isize, isize)> = HashSet::from_iter(cubes.iter());

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((0, 0, 0));
    let mut sides = 0;

    while let Some((x, y, z)) = queue.pop_front() {
        if !visited.insert((x, y, z)) || cubes_set.contains(&(x, y, z)) {
            continue;
        }

        for (x_change, y_change, z_change) in &MOVES {
            let next_x = x + x_change;
            let next_y = y + y_change;
            let next_z = z + z_change;

            if next_x < -1
                || next_x > x_bound
                || next_y < -1
                || next_y > y_bound
                || next_z < -1
                || next_z > z_bound
            {
                continue;
            }

            let key = (next_x, next_y, next_z);
            if visited.contains(&key) {
                continue;
            }
            if cubes_set.contains(&key) {
                sides += 1;
                continue;
            }

            queue.push_back(key);
        }
    }
    sides
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn test_part_1_sample() {
        let cubes1 = parse_cubes("1,1,1\n2,1,1");

        assert_eq!(count_non_connected_sides(&cubes1), 10);

        let cubes2 = parse_cubes(SAMPLE);

        assert_eq!(count_non_connected_sides(&cubes2), 64);
    }

    #[test]
    fn test_part_2_sample() {
        let cubes1 = parse_cubes(SAMPLE);

        assert_eq!(exterior_surface(&cubes1), 58);
    }
}
