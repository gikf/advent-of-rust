use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq)]
enum Operation {
    Sum,
    Product,
}

impl Operation {
    fn calculate(&self, numbers: &[usize]) -> usize {
        match self {
            Operation::Product => numbers.iter().product(),
            Operation::Sum => numbers.iter().sum(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Problem {
    numbers: Vec<usize>,
    operation: Operation,
}

impl Problem {
    fn calculate(&self) -> usize {
        self.operation.calculate(&self.numbers)
    }
}

fn main() {
    let input = Path::new("2025/day06/src/input.txt");
    let contents = &fs::read_to_string(input).unwrap();

    let math_problems = parse_math_problems(contents);
    let cephalopod_math_problems = parse_cephalopod_math_problems(contents);

    println!("Step 1");
    println!("Total: {}", calculate_problems(&math_problems));

    println!("Step 2");
    println!(
        "Cephalopod math total: {}",
        calculate_problems(&cephalopod_math_problems)
    );
}

fn parse_math_problems(input: &str) -> Vec<Problem> {
    let mut lines = input.lines();
    let operations: Vec<Operation> = lines
        .next_back()
        .unwrap()
        .split_ascii_whitespace()
        .map(|op| match op {
            "*" => Operation::Product,
            "+" => Operation::Sum,
            _ => unreachable!(),
        })
        .collect();

    let worksheet = lines
        .map(|line| line.split_ascii_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut problems = Vec::new();
    for ((column, _), operation) in worksheet[0].iter().enumerate().zip(operations) {
        let numbers = worksheet
            .iter()
            .map(|line| line[column].parse().unwrap())
            .collect::<Vec<usize>>();

        problems.push(Problem { numbers, operation })
    }
    problems
}

fn parse_cephalopod_math_problems(input: &str) -> Vec<Problem> {
    let mut lines = input.lines();
    let operations: Vec<Operation> = lines
        .next_back()
        .unwrap()
        .split_ascii_whitespace()
        .rev()
        .map(|op| match op {
            "*" => Operation::Product,
            "+" => Operation::Sum,
            _ => unreachable!(),
        })
        .collect();

    let worksheet = lines
        .map(|line| line.chars().rev().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut problem_numbers: Vec<Vec<usize>> = vec![vec![]];

    for (col_no, _) in worksheet[0].iter().enumerate() {
        let column = worksheet
            .iter()
            .map(|line| line[col_no])
            .filter(|d| *d != ' ')
            .collect::<Vec<_>>();
        if column.is_empty() {
            problem_numbers.push(vec![]);
            continue;
        }

        problem_numbers.last_mut().unwrap().push(
            column
                .into_iter()
                .filter(|md| md != &' ')
                .collect::<String>()
                .parse::<usize>()
                .unwrap(),
        )
    }

    problem_numbers
        .into_iter()
        .zip(operations)
        .map(|(numbers, operation)| Problem { numbers, operation })
        .collect()
}

fn calculate_problems(problems: &[Problem]) -> usize {
    problems.iter().map(|problem| problem.calculate()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

    #[test]
    fn test_calculate_product() {
        assert_eq!(Operation::Product.calculate(&[123, 45, 6]), 33210);
        assert_eq!(Operation::Product.calculate(&[51, 387, 215]), 4243455);
        assert_eq!(Operation::Product.calculate(&[356, 24, 1]), 8544);
        assert_eq!(Operation::Product.calculate(&[175, 581, 32]), 3253600);
    }

    #[test]
    fn test_calculate_sum() {
        assert_eq!(Operation::Sum.calculate(&[328, 64, 98]), 490);
        assert_eq!(Operation::Sum.calculate(&[64, 23, 314]), 401);
        assert_eq!(Operation::Sum.calculate(&[4, 431, 623]), 1058);
        assert_eq!(Operation::Sum.calculate(&[8, 248, 369]), 625);
    }

    #[test]
    fn test_calculate_problems_step_1() {
        let problems = parse_math_problems(SAMPLE);
        assert_eq!(calculate_problems(&problems), 4277556);
    }

    #[test]
    fn test_calculate_problems_step_2() {
        let problems = parse_cephalopod_math_problems(SAMPLE);
        assert_eq!(calculate_problems(&problems), 3263827);
    }
}
