use std::fs;
use std::path::Path;

fn main() {
    let input = Path::new("2021/day07/src/input.txt");
    let positions = parse_positions(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Cheapest cost: {:?}",
        cheapest_position_constant_cost(&positions)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Cheapest cost, with non-constant step rates: {:?}",
        cheapest_position_cost_non_constant_rate(&positions)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_positions(input: &str) -> Vec<usize> {
    input.split(',').map(|num| num.parse().unwrap()).collect()
}

fn cheapest_position_constant_cost(positions: &[usize]) -> usize {
    cheapest_position_with_cost(positions, |position, target| position.abs_diff(target))
}

fn cheapest_position_cost_non_constant_rate(positions: &[usize]) -> usize {
    cheapest_position_with_cost(positions, |position, target| {
        let steps_to_target = position.abs_diff(target);
        steps_to_target * (steps_to_target + 1) / 2
    })
}

fn cheapest_position_with_cost<CostFn: Fn(usize, usize) -> usize>(
    positions: &[usize],
    cost_fn: CostFn,
) -> usize {
    let min = positions.iter().min().unwrap();
    let max = positions.iter().max().unwrap();

    let mut cost = vec![0; max - min + 1];

    for position in positions {
        let position = position - min;
        for (target, value) in cost.iter_mut().enumerate() {
            *value += cost_fn(position, target);
        }
    }
    *cost.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_part_1_sample() {
        let positions = parse_positions(SAMPLE);

        assert_eq!(cheapest_position_constant_cost(&positions), 37);
    }

    #[test]
    fn test_part_2_sample() {
        let positions = parse_positions(SAMPLE);

        assert_eq!(cheapest_position_cost_non_constant_rate(&positions), 168);
    }
}
