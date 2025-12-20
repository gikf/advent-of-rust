use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq)]
enum Op {
    Add,
    Mul,
    Concatenation,
}

impl Op {
    fn do_it(&self, num1: usize, num2: usize) -> usize {
        match self {
            Op::Add => num1 + num2,
            Op::Mul => num1 * num2,
            Op::Concatenation => {
                let mut multiplier = 10;
                while multiplier <= num2 {
                    multiplier *= 10;
                }

                num1 * multiplier + num2
            }
        }
    }
}

fn main() {
    let input = Path::new("2024/day07/src/input.txt");
    let equations = parse_equations(&fs::read_to_string(input).unwrap());

    let part1_ops = [Op::Add, Op::Mul];
    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Total calibration result: {:?}",
        calibration_result(&equations, &part1_ops)
    );
    println!("In {:?}", part1.elapsed());

    let part2_ops = [Op::Add, Op::Mul, Op::Concatenation];
    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Calibration with concatenation: {:?}",
        calibration_result(&equations, &part2_ops)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_equations(input: &str) -> Vec<(usize, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_ascii_whitespace();
            let result: usize = parts.next().unwrap().trim_end_matches(':').parse().unwrap();
            let nums: Vec<usize> = parts.map(|num| num.parse().unwrap()).collect();
            (result, nums)
        })
        .collect()
}

fn calibration_result(equations: &[(usize, Vec<usize>)], ops: &[Op]) -> usize {
    equations
        .iter()
        .filter_map(|equation| {
            if is_equation_possible(equation, ops) {
                Some(equation.0)
            } else {
                None
            }
        })
        .sum()
}

fn is_equation_possible(equation: &(usize, Vec<usize>), ops: &[Op]) -> bool {
    let (target, nums) = &equation;

    let mut iter = nums.iter();
    let mut results = vec![*iter.next().unwrap()];

    for num in iter {
        let mut next_results = Vec::new();

        for so_far in &results {
            for op in ops.iter() {
                let current_value = op.do_it(*so_far, *num);
                if current_value > *target {
                    continue;
                }
                next_results.push(current_value);
            }
        }
        results = next_results;
    }

    results.contains(target)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "190: 10 19\n3267: 81 40 27\n83: 17 5\n156: 15 6\n7290: 6 8 6 15\n161011: 16 10 13\n192: 17 8 14\n21037: 9 7 18 13\n292: 11 6 16 20";

    #[test]
    fn test_parse_input() {
        let parsed = parse_equations(SAMPLE);

        assert_eq!(
            parsed,
            Vec::from([
                (190, Vec::from([10, 19])),
                (3267, Vec::from([81, 40, 27])),
                (83, Vec::from([17, 5])),
                (156, Vec::from([15, 6])),
                (7290, Vec::from([6, 8, 6, 15])),
                (161011, Vec::from([16, 10, 13])),
                (192, Vec::from([17, 8, 14])),
                (21037, Vec::from([9, 7, 18, 13])),
                (292, Vec::from([11, 6, 16, 20])),
            ])
        )
    }

    #[test]
    fn test_is_equation_possible() {
        let ops = [Op::Add, Op::Mul];
        assert_eq!(
            is_equation_possible(&(190, Vec::from([19, 10])), &ops),
            true
        );
        assert_eq!(
            is_equation_possible(&(3267, Vec::from([81, 40, 27])), &ops),
            true
        );
        assert_eq!(is_equation_possible(&(83, Vec::from([17, 5])), &ops), false);
        assert_eq!(
            is_equation_possible(&(156, Vec::from([15, 6])), &ops),
            false
        );
        assert_eq!(
            is_equation_possible(&(7290, Vec::from([6, 8, 6, 15])), &ops),
            false
        );
        assert_eq!(
            is_equation_possible(&(161011, Vec::from([16, 10, 13])), &ops),
            false
        );
        assert_eq!(
            is_equation_possible(&(192, Vec::from([17, 8, 14])), &ops),
            false
        );
        assert_eq!(
            is_equation_possible(&(21037, Vec::from([9, 7, 18, 13])), &ops),
            false
        );
        assert_eq!(
            is_equation_possible(&(292, Vec::from([11, 6, 16, 20])), &ops),
            true
        );
    }

    #[test]
    fn test_is_equation_possible_with_conc() {
        let ops = [Op::Add, Op::Mul, Op::Concatenation];
        assert_eq!(is_equation_possible(&(156, Vec::from([15, 6])), &ops), true);
        assert_eq!(
            is_equation_possible(&(7290, Vec::from([6, 8, 6, 15])), &ops),
            true
        );
        assert_eq!(
            is_equation_possible(&(192, Vec::from([17, 8, 14])), &ops),
            true
        );
    }

    #[test]
    fn test_part_1_sample() {
        let equations = parse_equations(SAMPLE);
        let ops = [Op::Add, Op::Mul];

        let result = calibration_result(&equations, &ops);

        assert_eq!(result, 3749);
    }

    #[test]
    fn test_part_2_sample() {
        let equations = parse_equations(SAMPLE);
        let ops = [Op::Add, Op::Mul, Op::Concatenation];

        let result = calibration_result(&equations, &ops);

        assert_eq!(result, 11387);
    }
}
