use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq)]
enum Out {
    Left,
    Right,
}

impl From<char> for Out {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => unreachable!(),
        }
    }
}

fn main() {
    let input = Path::new("2023/day08/src/input.txt");
    let content = fs::read_to_string(input).unwrap();
    let (instructions, nodes) = parse_input(&content);

    let target = "ZZZ";

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Steps to ZZZ: {:?}",
        steps_to(&instructions, &nodes, target)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Simultaneous steps to be only on nodes ending with Z: {:?}",
        simultaneous_steps_to(&instructions, &nodes, "Z")
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_input(input: &str) -> (Vec<Out>, HashMap<&str, (&str, &str)>) {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars().map(Out::from).collect();

    let mut nodes = HashMap::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }

        let (name, rest) = line.split_once(" = ").unwrap();
        let (left, right) = rest[1..(rest.len() - 1)].split_once(", ").unwrap();
        nodes.insert(name, (left, right));
    }

    (instructions, nodes)
}

fn steps_to(instructions: &[Out], nodes: &HashMap<&str, (&str, &str)>, target: &str) -> usize {
    let mut cur_node = "AAA";

    for (index, instruction) in instructions.iter().cycle().enumerate() {
        let node_outs = nodes.get(&cur_node).unwrap();
        cur_node = match instruction {
            Out::Left => node_outs.0,
            Out::Right => node_outs.1,
        };
        if cur_node == target {
            return index + 1;
        }
    }
    0
}

fn simultaneous_steps_to(
    instructions: &[Out],
    nodes: &HashMap<&str, (&str, &str)>,
    target_ending: &str,
) -> usize {
    let mut cur_nodes: Vec<(usize, &str)> = nodes
        .keys()
        .filter(|name| name.ends_with("A"))
        .enumerate()
        .map(|(id, name)| (id, *name))
        .collect();
    let number_of_paths = cur_nodes.len();
    let mut z_found: Vec<Vec<usize>> = vec![vec![]; cur_nodes.len()];

    for (index, instruction) in instructions.iter().cycle().enumerate() {
        let mut next_nodes = Vec::with_capacity(number_of_paths);
        for (node_no, cur_node) in cur_nodes.iter() {
            let node_outs = nodes.get(*cur_node).unwrap();
            let next_node = match instruction {
                Out::Left => node_outs.0,
                Out::Right => node_outs.1,
            };
            if next_node.ends_with(target_ending) {
                z_found[*node_no].push(index);
            }
            if z_found[*node_no].len() != 2 {
                next_nodes.push((*node_no, next_node));
            }
        }
        if next_nodes.is_empty() {
            break;
        }
        cur_nodes = next_nodes;
    }

    let steps: usize = z_found
        .iter()
        .map(|z_indices| z_indices[1] - z_indices[0])
        .reduce(lcm)
        .unwrap();
    steps
}

fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    a * (b / gcd(a, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const SAMPLE2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    const SAMPLE3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_parse_input() {
        let (instructions1, nodes1) = parse_input(SAMPLE1);

        assert_eq!(instructions1, [Out::Right, Out::Left]);
        assert_eq!(
            nodes1,
            HashMap::from([
                ("AAA", ("BBB", "CCC")),
                ("BBB", ("DDD", "EEE")),
                ("CCC", ("ZZZ", "GGG")),
                ("DDD", ("DDD", "DDD")),
                ("EEE", ("EEE", "EEE")),
                ("GGG", ("GGG", "GGG")),
                ("ZZZ", ("ZZZ", "ZZZ"))
            ])
        );

        let (instructions2, nodes2) = parse_input(SAMPLE2);

        assert_eq!(instructions2, [Out::Left, Out::Left, Out::Right]);
        assert_eq!(
            nodes2,
            HashMap::from([
                ("AAA", ("BBB", "BBB")),
                ("BBB", ("AAA", "ZZZ")),
                ("ZZZ", ("ZZZ", "ZZZ")),
            ])
        )
    }

    #[test]
    fn test_part_1_sample() {
        let target = "ZZZ";

        let (instructions1, nodes1) = parse_input(SAMPLE1);
        assert_eq!(steps_to(&instructions1, &nodes1, target), 2);

        let (instructions2, nodes2) = parse_input(SAMPLE2);
        assert_eq!(steps_to(&instructions2, &nodes2, target), 6);
    }

    #[test]
    fn test_part_2_sample() {
        let target_ending = "Z";

        let (instructions, nodes) = parse_input(SAMPLE3);
        assert_eq!(
            simultaneous_steps_to(&instructions, &nodes, target_ending),
            6
        );
    }
}
