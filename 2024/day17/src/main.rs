use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

#[derive(Debug, PartialEq)]
enum Operand {
    Literal(i8),
    Combo(i8),
}

#[derive(Debug, PartialEq)]
struct Computer {
    a: isize,
    b: isize,
    c: isize,
    instruction: usize,
    out: Vec<isize>,
}

impl Computer {
    fn run(&mut self, program: &[(Instruction, Operand)]) -> Option<String> {
        while self.instruction < program.len() {
            if self.process_instruction(&program[self.instruction]) {
                self.instruction += 1;
            }
        }

        if !self.out.is_empty() {
            Some(
                self.out
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            )
        } else {
            None
        }
    }

    fn process_instruction(&mut self, (instruction, operand): &(Instruction, Operand)) -> bool {
        match instruction {
            Instruction::Adv => {
                self.a /= 2isize.pow(self.get_value(operand) as u32);
            }
            Instruction::Bxl => {
                self.b ^= self.get_value(operand);
            }
            Instruction::Bst => {
                self.b = self.get_value(operand).rem_euclid(8);
            }
            Instruction::Jnz => {
                if self.a != 0 {
                    self.instruction = (self.get_value(operand) / 2) as usize;
                    return false;
                }
            }
            Instruction::Bxc => {
                self.b ^= self.c;
            }
            Instruction::Out => {
                self.out.push(self.get_value(operand).rem_euclid(8));
            }
            Instruction::Bdv => {
                self.b = self.a / 2isize.pow(self.get_value(operand) as u32);
            }
            Instruction::Cdv => {
                self.c = self.a / 2isize.pow(self.get_value(operand) as u32);
            }
        }
        true
    }

    fn get_value(&self, operand: &Operand) -> isize {
        match operand {
            Operand::Literal(value) => *value as isize,
            Operand::Combo(ref_to) => match ref_to {
                val @ 0..=3 => *val as isize,
                4 => self.a,
                5 => self.b,
                6 => self.c,
                _ => unimplemented!(),
            },
        }
    }
}

fn main() {
    let input = Path::new("2024/day17/src/input.txt");
    let read_input = fs::read_to_string(input).unwrap();
    let (mut computer, program, raw_program) = parse_input(&read_input);

    let part1 = std::time::Instant::now();
    println!("Part 1");
    let result = computer.run(&program).unwrap();
    println!("Result: {:?}", result);
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();

    println!("Part 2");
    let a = find_a(&raw_program);
    computer.a = a;
    computer.b = 0;
    computer.c = 0;
    computer.out = vec![];
    computer.instruction = 0;
    let result = computer.run(&program);
    println!(
        "Number to initialize A, to make program output itself: {:?}",
        a
    );
    println!("Result: {:?}", result.unwrap());
    println!("In {:?}", part2.elapsed());
}

fn parse_input(input: &str) -> (Computer, Vec<(Instruction, Operand)>, Vec<i8>) {
    let mut lines = input.lines();
    let registers: Vec<isize> = (0..3)
        .map(|_| {
            lines
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .last()
                .unwrap()
                .parse::<isize>()
                .unwrap()
        })
        .collect();

    let _ = lines.next();

    let raw_program = lines.next().unwrap()[("Program: ".len())..]
        .split(',')
        .map(|v| v.parse().unwrap())
        .collect::<Vec<_>>();

    let program: Vec<(Instruction, Operand)> = raw_program
        .clone()
        .chunks_exact(2)
        .map(|instruction| {
            if let &[opcode, operand] = instruction {
                match opcode {
                    0 => (Instruction::Adv, Operand::Combo(operand)),
                    1 => (Instruction::Bxl, Operand::Literal(operand)),
                    2 => (Instruction::Bst, Operand::Combo(operand)),
                    3 => (Instruction::Jnz, Operand::Literal(operand)),
                    4 => (Instruction::Bxc, Operand::Combo(operand)),
                    5 => (Instruction::Out, Operand::Combo(operand)),
                    6 => (Instruction::Bdv, Operand::Combo(operand)),
                    7 => (Instruction::Cdv, Operand::Combo(operand)),
                    _ => unimplemented!(),
                }
            } else {
                panic!()
            }
        })
        .collect();

    (
        Computer {
            a: registers[0],
            b: registers[1],
            c: registers[2],
            instruction: 0,
            out: vec![],
        },
        program,
        raw_program,
    )
}

fn result_from_a(a: isize) -> isize {
    let a1 = a;
    let b1 = a1 % 8;
    let b2 = b1 ^ 6;
    let c1 = a1 / 2_isize.pow(b2 as u32);
    let b3 = b2 ^ c1;
    b3 ^ 7
}

fn results_from_range(a: isize) -> impl Iterator<Item = (isize, isize)> {
    (a..(a + 8)).map(|num| (num, result_from_a(num)))
}

fn find_a(raw_program: &[i8]) -> isize {
    let mut queue: Vec<(usize, isize)> = vec![(0, 0)];

    while let Some((level, num)) = queue.pop() {
        if level == raw_program.len() {
            return num;
        }
        for (for_num, result) in results_from_range(num * 8) {
            if result % 8 == raw_program[raw_program.len() - 1 - level] as isize {
                queue.push((level + 1, for_num));
            }
        }
    }
    0
}

#[allow(unused)]
fn find_sample_a(raw_program: &[i8]) -> isize {
    for a in 8_isize.. {
        if (1..)
            .zip(raw_program.iter())
            .all(|(pow, result)| (a / (8_isize.pow(pow as u32))) % 8 == *result as isize)
        {
            return a;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
    const SAMPLE2: &str = "Register A: 0
Register B: 0
Register C: 9

Program: 2,6";
    const SAMPLE3: &str = "Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4";
    const SAMPLE4: &str = "Register A: 2024
Register B: 0
Register C: 9

Program: 0,1,5,4,3,0";
    const SAMPLE5: &str = "Register A: 0
Register B: 29
Register C: 0

Program: 1,7";
    const SAMPLE6: &str = "Register A: 0
Register B: 2024
Register C: 43690

Program: 4,0";
    const SAMPLE7: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn test_parse_input() {
        let (computer, _, _) = parse_input(SAMPLE);

        assert_eq!(
            computer,
            Computer {
                a: 729,
                b: 0,
                c: 0,
                instruction: 0,
                out: vec![],
            }
        );
    }

    #[test]
    fn test_computer1() {
        let (mut computer, program, _) = parse_input(SAMPLE2);
        let _ = computer.run(&program);

        assert_eq!(computer.b, 1);
    }

    #[test]
    fn test_computer2() {
        let (mut computer, program, _) = parse_input(SAMPLE3);
        let _ = computer.run(&program);

        assert_eq!(computer.out, [0, 1, 2]);
    }

    #[test]
    fn test_computer3() {
        let (mut computer, program, _) = parse_input(SAMPLE4);
        let res = computer.run(&program);

        assert_eq!(computer.out, [4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(res, Some("4,2,5,6,7,7,7,7,3,1,0".to_string()));
        assert_eq!(computer.a, 0);
    }

    #[test]
    fn test_computer4() {
        let (mut computer, program, _) = parse_input(SAMPLE5);
        let _ = computer.run(&program);

        assert_eq!(computer.b, 26);
    }

    #[test]
    fn test_computer5() {
        let (mut computer, program, _) = parse_input(SAMPLE6);
        let _ = computer.run(&program);

        assert_eq!(computer.b, 44354);
    }

    #[test]
    fn test_part_1_sample() {
        let (mut computer, program, _) = parse_input(SAMPLE);
        let result = computer.run(&program);

        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_2_sample() {
        let (_, _, raw_program) = parse_input(SAMPLE7);

        let result = find_sample_a(&raw_program);
        assert_eq!(result, 117440);
    }
}
