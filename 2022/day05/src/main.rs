use std::fs;
use std::path::Path;

type Crate = Vec<char>;
type Instruction = (usize, usize, usize);

fn main() {
    let input = Path::new("2022/day05/src/input.txt");
    let (stacks, instructions) = parse_crates(&fs::read_to_string(input).unwrap());

    let mut part1_stacks = stacks.clone();
    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Crates at the top after rearrangement: {:?}",
        crates_on_top(&mut part1_stacks, &instructions, cratemover9000)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    let mut stacks = stacks;
    println!("Part 2");
    println!(
        "Crates at the top after rearrangement with Cratemover 9001: {:?}",
        crates_on_top(&mut stacks, &instructions, cratemover9001)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_crates(input: &str) -> (Vec<Crate>, Vec<Instruction>) {
    let mut lines = input.lines();
    let mut stacks = Vec::new();
    let mut instructions = Vec::new();

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        } else if line.starts_with(" 1") {
            continue;
        }

        let mut rest = line;
        for stack_no in 0.. {
            let (on_stack, r) = if rest.len() > 3 {
                rest.split_at(3)
            } else {
                (rest, "")
            };
            if !on_stack.trim_ascii().is_empty() {
                while stacks.len() <= stack_no {
                    stacks.push(vec![]);
                }
                stacks[stack_no].push(on_stack.chars().nth(1).unwrap());
            }
            if r.is_empty() {
                break;
            }
            rest = &r[1..];
        }
    }

    for line in lines {
        let split: Vec<_> = line.split_ascii_whitespace().collect();
        let [count, from, to]: [usize; 3] =
            [split[1], split[3], split[5]].map(|num| num.parse().unwrap());
        instructions.push((count, from - 1, to - 1));
    }

    for stack in stacks.iter_mut() {
        stack.reverse();
    }

    (stacks, instructions)
}

fn cratemover9000(stacks: &mut [Crate], instructions: &[Instruction]) {
    for (count, from, to) in instructions {
        for _ in 0..(*count) {
            let one_crate = &stacks[*from].pop().unwrap();
            let _ = &stacks[*to].push(*one_crate);
        }
    }
}

fn cratemover9001(stacks: &mut [Crate], instructions: &[Instruction]) {
    for (count, from, to) in instructions {
        let crates_present = stacks[*from].len();
        let moved = &stacks[*from].split_off(crates_present - count);
        stacks[*to].extend_from_slice(&moved[..]);
    }
}

fn crates_on_top<Mover: Fn(&mut [Crate], &[Instruction])>(
    stacks: &mut [Crate],
    instructions: &[Instruction],
    mover: Mover,
) -> String {
    mover(stacks, instructions);

    stacks.iter().map(|stack| *stack.last().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    const SAMPLE2: &str = "                [M]     [W] [M]
            [L] [Q] [S] [C] [R]
            [Q] [F] [F] [T] [N] [S]
    [N]     [V] [V] [H] [L] [J] [D]
    [D] [D] [W] [P] [G] [R] [D] [F]
[T] [T] [M] [G] [G] [Q] [N] [W] [L]
[Z] [H] [F] [J] [D] [Z] [S] [H] [Q]
[B] [V] [B] [T] [W] [V] [Z] [Z] [M]
 1   2   3   4   5   6   7   8   9

move 1 from 1 to 1";

    #[test]
    fn test_parse_stacks() {
        let (stacks, instructions) = parse_crates(SAMPLE);

        assert_eq!(
            stacks,
            [
                Vec::from(['Z', 'N']),
                Vec::from(['M', 'C', 'D']),
                Vec::from(['P']),
            ]
        );
        assert_eq!(instructions, [(1, 1, 0), (3, 0, 2), (2, 1, 0), (1, 0, 1),]);

        let (stacks2, _) = parse_crates(SAMPLE2);

        assert_eq!(
            stacks2,
            [
                Vec::from(['B', 'Z', 'T']),
                Vec::from(['V', 'H', 'T', 'D', 'N']),
                Vec::from(['B', 'F', 'M', 'D']),
                Vec::from(['T', 'J', 'G', 'W', 'V', 'Q', 'L']),
                Vec::from(['W', 'D', 'G', 'P', 'V', 'F', 'Q', 'M']),
                Vec::from(['V', 'Z', 'Q', 'G', 'H', 'F', 'S']),
                Vec::from(['Z', 'S', 'N', 'R', 'L', 'T', 'C', 'W']),
                Vec::from(['Z', 'H', 'W', 'D', 'J', 'N', 'R', 'M']),
                Vec::from(['M', 'Q', 'L', 'F', 'D', 'S']),
            ]
        );
    }

    #[test]
    fn test_part_1_sample() {
        let (mut stacks, instructions) = parse_crates(SAMPLE);
        let result = crates_on_top(&mut stacks, &instructions, cratemover9000);

        assert_eq!(result, String::from("CMZ"));
    }

    #[test]
    fn test_part_2_sample() {
        let (mut stacks, instructions) = parse_crates(SAMPLE);
        let result = crates_on_top(&mut stacks, &instructions, cratemover9001);

        assert_eq!(result, String::from("MCD"));
    }
}
