use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq)]
enum Ref {
    Num(isize),
    Ref(char),
}

impl Ref {
    fn from(value: &str) -> Self {
        match value.parse::<isize>() {
            Ok(num) => Ref::Num(num),
            Err(_) => Ref::Ref(value.chars().next().unwrap()),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Inp(Ref),
    Add(Ref, Ref),
    Mul(Ref, Ref),
    Div(Ref, Ref),
    Mod(Ref, Ref),
    Eq(Ref, Ref),
}

fn main() {
    let input = Path::new("2021/day24/src/input.txt");
    let instructions = parse_instructions(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");

    println!(
        "Largest accepted model number: {:?}",
        find_model_number(&instructions, ascending)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Lowest accepted model number: {:?}",
        find_model_number(&instructions, descending)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for line in input.lines() {
        let parts: Vec<_> = line.split_ascii_whitespace().collect();
        let instruction = match parts[0] {
            "inp" => Instruction::Inp(Ref::from(parts[1])),
            "add" => Instruction::Add(Ref::from(parts[1]), Ref::from(parts[2])),
            "mul" => Instruction::Mul(Ref::from(parts[1]), Ref::from(parts[2])),
            "div" => Instruction::Div(Ref::from(parts[1]), Ref::from(parts[2])),
            "mod" => Instruction::Mod(Ref::from(parts[1]), Ref::from(parts[2])),
            "eql" => Instruction::Eq(Ref::from(parts[1]), Ref::from(parts[2])),
            _ => unimplemented!(),
        };
        instructions.push(instruction);
    }

    instructions
}

fn diff_values(instructions: &[Instruction]) -> Vec<[isize; 3]> {
    let mut values = Vec::new();
    for offset in 0..14 {
        let block_start = offset * 18;
        let div = match instructions[block_start + 4] {
            Instruction::Div(Ref::Ref('z'), Ref::Num(num)) => num,
            _ => unreachable!(),
        };
        let add1 = match instructions[block_start + 5] {
            Instruction::Add(Ref::Ref('x'), Ref::Num(num)) => num,
            _ => unreachable!(),
        };
        let add2 = match instructions[block_start + 15] {
            Instruction::Add(Ref::Ref('y'), Ref::Num(num)) => num,
            _ => unreachable!(),
        };
        values.push([div, add1, add2]);
    }
    values
}

fn get_prev_zs(div1: isize, add1: isize, add2: isize, target_z: isize, w: isize) -> Vec<isize> {
    let mut zs = Vec::new();

    let x = target_z - w - add2;
    if x % 26 == 0 {
        zs.push((x / 26) * div1);
    }
    let wadd = w - add1;
    if (0..26).contains(&wadd) {
        let z0 = target_z * div1;
        zs.push(wadd + z0);
    }

    zs
}

fn ascending() -> Box<dyn Iterator<Item = isize>> {
    Box::new(1..=9)
}

fn descending() -> Box<dyn Iterator<Item = isize>> {
    Box::new((1..=9).rev())
}

fn find_model_number<ItemFunc: Fn() -> Box<dyn Iterator<Item = isize>>>(
    instructions: &[Instruction],
    items: ItemFunc,
) -> isize {
    // Based on https://old.reddit.com/r/adventofcode/comments/rnejv5/2021_day_24_solutions/hpw9wcb/

    let differing = diff_values(instructions);

    let mut zs = vec![0_isize];
    let mut result = HashMap::new();

    for [div1, add1, add2] in differing.into_iter().rev() {
        let mut seen = HashSet::new();
        let mut next_z = Vec::new();
        for target_z in zs.iter().rev() {
            for w in items() {
                let results = get_prev_zs(div1, add1, add2, *target_z, w);
                for z in results {
                    if !seen.insert(z) {
                        continue;
                    }
                    next_z.push(z);
                    let mut digits_so_far = result.get(target_z).unwrap_or(&vec![]).clone();
                    digits_so_far.push(w);
                    result.insert(z, digits_so_far);
                }
            }
        }
        zs = next_z;
    }
    let number = result.get(&0).unwrap().to_owned();
    number
        .into_iter()
        .enumerate()
        .map(|(index, d)| d * 10_isize.pow(index as u32))
        .sum()
}
