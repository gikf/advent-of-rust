use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq)]
enum Instruction {
    Noop,
    Addx(isize),
}

const NTHS: [usize; 6] = [20, 60, 100, 140, 180, 220];

fn main() {
    let input = Path::new("2022/day10/src/input.txt");
    let instructions = parse_instructions(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Sum of six signal strengths: {:?}",
        sum_of_signal_strengths(&instructions)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!("Drawing on the CRT:");
    draw_on_crt(&instructions);
    println!("In {:?}", part2.elapsed());
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            if line == "noop" {
                Instruction::Noop
            } else {
                let (_, value) = line.split_once(" ").unwrap();
                Instruction::Addx(value.parse().unwrap())
            }
        })
        .collect()
}

fn sum_of_signal_strengths(instructions: &[Instruction]) -> isize {
    let cycles = run_instructions(instructions);

    NTHS.iter()
        .map(|cycle| cycles[cycle - 1] * (*cycle as isize))
        .sum()
}

fn run_instructions(instructions: &[Instruction]) -> Vec<isize> {
    let mut x = 1;
    let mut cycles = vec![];

    for instruction in instructions {
        match instruction {
            Instruction::Noop => {
                cycles.push(x);
            }
            Instruction::Addx(value) => {
                cycles.push(x);
                cycles.push(x);
                x += value;
            }
        }
    }
    cycles
}

fn draw_on_crt(instructions: &[Instruction]) {
    let cycles = run_instructions(instructions);

    let mut drawing = vec![];
    cycles.iter().enumerate().for_each(|(cycle_no, value)| {
        if cycle_no % 40 == 0 {
            drawing.push(vec![]);
        }

        let pixel = if ((value - 1)..=(value + 1)).contains(&((cycle_no % 40) as isize)) {
            "#"
        } else {
            "."
        };
        drawing.last_mut().unwrap().push(pixel);
    });

    for row in drawing {
        println!("{:?}", row.join(""));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = "noop
addx 3
addx -5";
    const SAMPLE2: &str = "addx 15\naddx -11\naddx 6
addx -3\naddx 5\naddx -1\naddx -8\naddx 13\naddx 4\nnoop\naddx -1\naddx 5
addx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx -35\naddx 1
addx 24\naddx -19\naddx 1\naddx 16\naddx -11\nnoop\nnoop\naddx 21\naddx -15
noop\nnoop\naddx -3\naddx 9\naddx 1\naddx -3\naddx 8\naddx 1\naddx 5\nnoop
noop\nnoop\nnoop\nnoop\naddx -36\nnoop\naddx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2
addx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\naddx 7\naddx 1\nnoop
addx -13\naddx 13\naddx 7\nnoop\naddx 1\naddx -33\nnoop\nnoop\nnoop\naddx 2
noop\nnoop\nnoop\naddx 8\nnoop\naddx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9
addx 1\naddx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\nnoop\nnoop
addx -13\naddx -19\naddx 1\naddx 3\naddx 26\naddx -30\naddx 12\naddx -1\naddx 3
addx 1\nnoop\nnoop\nnoop\naddx -9\naddx 18\naddx 1\naddx 2\nnoop\nnoop\naddx 9
noop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\naddx 3\nnoop\naddx 15
addx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\naddx 1\nnoop\naddx -10\nnoop
noop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop";

    #[test]
    fn test_parse_instructions() {
        let instructions = parse_instructions(SAMPLE1);

        assert_eq!(
            instructions,
            [
                Instruction::Noop,
                Instruction::Addx(3),
                Instruction::Addx(-5)
            ]
        );
    }

    #[test]
    fn test_part_1_sample() {
        let instructions = parse_instructions(SAMPLE2);

        assert_eq!(sum_of_signal_strengths(&instructions), 13140);
    }

    #[test]
    fn test_part_2_sample() {
        let instructions = parse_instructions(SAMPLE2);

        draw_on_crt(&instructions);
    }
}
