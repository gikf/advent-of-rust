use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq)]
enum Op<'a> {
    Remove(&'a str),
    Insert(&'a str, usize),
}

fn main() {
    let input = Path::new("2023/day15/src/input.txt");
    let contents = fs::read_to_string(input).unwrap();
    let steps = parse_input(&contents);

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!("Initialization sum: {:?}", initialization_sum(&steps));
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    let operations = parse_operations(&contents);
    println!("Part 2");
    println!("Focusing power: {:?}", initialize_lenses(&operations));
    println!("In {:?}", part2.elapsed());
}

fn parse_input(input: &str) -> Vec<&str> {
    input.split(",").collect()
}

fn parse_operations(input: &str) -> Vec<Op<'_>> {
    input
        .split(",")
        .map(|operation| {
            if let Some((label, focal_length)) = operation.split_once("=") {
                Op::Insert(label, focal_length.parse().unwrap())
            } else {
                let label = operation.trim_end_matches("-");
                Op::Remove(label)
            }
        })
        .collect()
}

fn hash_it(text: &str) -> usize {
    let mut value = 0;
    text.as_bytes().iter().for_each(|code| {
        value = ((value + (*code as usize)) * 17) % 256;
    });
    value
}

fn initialization_sum(steps: &[&str]) -> usize {
    steps.iter().map(|step| hash_it(step)).sum()
}

fn initialize<'a>(operations: &'a [Op]) -> Vec<Vec<(&'a str, usize)>> {
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];

    operations.iter().for_each(|op| match op {
        Op::Remove(label) => {
            let box_index = hash_it(label);
            if let Some(index) = boxes[box_index].iter().position(|(l, _)| l == label) {
                boxes[box_index].remove(index);
            }
        }
        Op::Insert(label, focal_length) => {
            let box_index = hash_it(label);
            if let Some(index) = boxes[box_index].iter().position(|(l, _)| l == label) {
                boxes[box_index][index].1 = *focal_length;
            } else {
                boxes[box_index].push((label, *focal_length));
            }
        }
    });
    boxes
}

fn initialize_lenses(operations: &[Op]) -> usize {
    focusing_power(&initialize(operations))
}

fn focusing_power(boxes: &[Vec<(&str, usize)>]) -> usize {
    boxes
        .iter()
        .enumerate()
        .map(|(box_no, slots)| {
            slots
                .iter()
                .enumerate()
                .map(|(slot_no, (_, focal_length))| (box_no + 1) * (slot_no + 1) * *focal_length)
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_hash_it() {
        assert_eq!(hash_it("HASH"), 52);
        assert_eq!(hash_it("rn=1"), 30);
        assert_eq!(hash_it("cm-"), 253);
        assert_eq!(hash_it("qp=3"), 97);
        assert_eq!(hash_it("cm=2"), 47);
        assert_eq!(hash_it("qp-"), 14);
        assert_eq!(hash_it("pc=4"), 180);
        assert_eq!(hash_it("ot=9"), 9);
        assert_eq!(hash_it("ab=5"), 197);
        assert_eq!(hash_it("pc-"), 48);
        assert_eq!(hash_it("pc=6"), 214);
        assert_eq!(hash_it("ot=7"), 231);
    }

    #[test]
    fn test_part_1_sample() {
        let steps = parse_input(SAMPLE);

        assert_eq!(initialization_sum(&steps), 1320);
    }

    #[test]
    fn test_part_2_sample() {
        let operations = parse_operations(SAMPLE);

        assert_eq!(initialize_lenses(&operations), 145);
    }
}
