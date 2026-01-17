use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs;
use std::path::Path;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn move_by(&self, count: isize) -> (isize, isize) {
        match self {
            Self::Up => (-count, 0),
            Self::Right => (0, count),
            Self::Down => (count, 0),
            Self::Left => (0, -count),
        }
    }

    fn as_num(&self) -> usize {
        match self {
            Self::Up => 0,
            Self::Right => 1,
            Self::Down => 2,
            Self::Left => 3,
        }
    }
}

fn main() {
    let input = Path::new("2023/day17/src/input.txt");
    let map = parse_map(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    let min_steps = 1;
    let max_steps = 3;
    println!("Part 1");
    println!(
        "Least heat loss: {:?}",
        least_heat_loss(&map, min_steps, max_steps)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    let min_steps = 4;
    let max_steps = 10;
    println!("Part 2");
    println!(
        "Least heat loss for ultra crucible: {:?}",
        least_heat_loss(&map, min_steps, max_steps)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_map(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn least_heat_loss(map: &[Vec<usize>], min_steps: usize, max_steps: usize) -> usize {
    // Based on https://cutonbuminband.github.io/AOC/qmd/2023.html#day-17-clumsy-crucible

    let mut costs: Vec<Vec<((usize, usize), Direction)>> = vec![vec![]; 1_500];
    let mut queue = BinaryHeap::from([Reverse(0)]);
    costs[0].push(((0, 0), Direction::Right));
    let rows = map.len();
    let cols = map[0].len();

    let mut seen = vec![[0, 0, 0, 0]; rows * cols];

    while let Some(min_cost) = queue.pop() {
        let cost = min_cost.0;
        if cost > costs.len() - 1 || costs[cost].is_empty() {
            continue;
        }

        if cost + 100 > costs.len() {
            costs.resize(costs.len() + 100, vec![]);
        }
        let costs_to_process: Vec<_> = costs.get_mut(cost).unwrap().drain(..).collect();
        for ((row, col), direction) in costs_to_process {
            if row == rows - 1 && col == cols - 1 {
                return cost;
            }

            let seen_index = row * cols + col;
            if seen[seen_index][direction.as_num()] == 1 {
                continue;
            }

            seen[seen_index][direction.as_num()] = 1;

            let directions = match direction {
                Direction::Up | Direction::Down => [Direction::Left, Direction::Right],
                Direction::Left | Direction::Right => [Direction::Up, Direction::Down],
            };

            for next_direction in directions {
                let mut next_cost = cost;
                for straight_steps in 1..=max_steps {
                    let (row_change, col_change) = direction.move_by(straight_steps as isize);
                    let next_row = (row as isize) + row_change;
                    let next_col = (col as isize) + col_change;
                    if next_row < 0
                        || next_col < 0
                        || next_row >= (rows as isize)
                        || next_col >= (cols as isize)
                    {
                        break;
                    }

                    let next_row = next_row as usize;
                    let next_col = next_col as usize;
                    next_cost += map[next_row][next_col];
                    let next_seen_index = next_row * cols + next_col;
                    if seen[next_seen_index][next_direction.as_num()] == 1 {
                        continue;
                    }
                    if straight_steps >= min_steps {
                        costs[next_cost].push(((next_row, next_col), next_direction));
                        queue.push(Reverse(next_cost));
                    }
                }
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
    const SAMPLE2: &str = "111111111111
999999999991
999999999991
999999999991
999999999991";

    #[test]
    fn test_part_1_sample() {
        let map = parse_map(SAMPLE);

        assert_eq!(least_heat_loss(&map, 1, 3), 102);
    }

    #[test]
    fn test_part_2_sample() {
        let map = parse_map(SAMPLE);
        let map2 = parse_map(SAMPLE2);

        assert_eq!(least_heat_loss(&map, 4, 10), 94);
        assert_eq!(least_heat_loss(&map2, 4, 10), 71);
    }
}
