use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::path::Path;

type Op<'a> = (Gate, &'a str, &'a str, &'a str);

#[derive(Debug, PartialEq, Clone, Copy)]
enum Gate {
    And,
    Or,
    Xor,
}

impl Gate {
    fn output(&self, a: u8, b: u8) -> u8 {
        match self {
            Self::And => {
                if a == 1 && b == 1 {
                    1
                } else {
                    0
                }
            }
            Self::Or => {
                if a == 1 || b == 1 {
                    1
                } else {
                    0
                }
            }
            Self::Xor => a ^ b,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct GateParseError;

impl std::str::FromStr for Gate {
    type Err = GateParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "AND" => Ok(Gate::And),
            "OR" => Ok(Gate::Or),
            "XOR" => Ok(Gate::Xor),
            _ => Err(GateParseError),
        }
    }
}

fn main() {
    let input = Path::new("2024/day24/src/input.txt");
    let contents = fs::read_to_string(input).unwrap();
    let (addresses, ops) = parse_input(&contents);

    let mut part1_addresses = addresses.clone();
    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Decimal on the wires starting with \"z\": {:?}",
        run_ops(&mut part1_addresses, &ops)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();

    let mut ops = ops;
    let confirmed = confirm_swaps(
        &addresses,
        &mut ops,
        vec![
            ("z08", "vvr"),
            ("rnq", "bkr"),
            ("z28", "tfb"),
            ("z39", "mqh"),
        ],
    );
    println!("Part 2");
    println!("Confirmed solution: {:?}", confirmed);
    println!("In {:?}", part2.elapsed());
}

fn parse_input(input: &str) -> (HashMap<&str, u8>, Vec<Op<'_>>) {
    let mut lines = input.lines();

    let mut addresses: HashMap<&str, u8> = HashMap::new();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let (address, bit) = line.split_once(": ").unwrap();
        addresses.insert(address, bit.parse().unwrap());
    }

    let mut writes: Vec<Op> = Vec::new();
    for line in lines {
        let (left, rest) = line.split_once(" ").unwrap();
        let (gate, rest) = rest.split_once(" ").unwrap();
        let (right, rest) = rest.split_once(" ").unwrap();
        let write_to = rest.strip_prefix("-> ").unwrap();

        writes.push((gate.parse().unwrap(), left, right, write_to));
    }
    (addresses, writes)
}

fn run_ops<'a>(addresses: &mut HashMap<&'a str, u8>, ops: &'a [Op]) -> usize {
    let mut done = HashSet::new();
    while done.len() != ops.len() {
        for (op_no, (gate, left, right, write_to)) in ops.iter().enumerate() {
            if done.contains(&op_no) {
                continue;
            }
            if let (Some(a), Some(b)) = (addresses.get(*left), addresses.get(*right)) {
                done.insert(op_no);
                let result = gate.output(*a, *b);
                addresses.insert(*write_to, result);
            }
        }
    }

    let mut zs: Vec<(&str, &u8)> = addresses
        .iter()
        .filter(|(address, _)| address.starts_with("z"))
        .map(|(a, b)| (*a, b))
        .collect();
    zs.sort_by(|a, b| b.cmp(a));
    let binary: Vec<u8> = zs.iter().map(|(_, bit)| **bit).collect();

    let result: usize = binary
        .into_iter()
        .rev()
        .enumerate()
        .map(|(pow, bit)| (bit as usize) * 2_usize.pow(pow as u32))
        .sum();
    result
}

#[allow(unused)]
fn follow_wires(ops: &[Op]) -> Vec<Vec<(i32, String)>> {
    let mut target_to_wires: HashMap<&str, HashSet<&str>> = HashMap::new();

    for (_, a, b, target) in ops.iter() {
        if target_to_wires.contains_key(*target) {
            println!("duplicate, {:?}", target);
        }
        target_to_wires.insert(*target, HashSet::from([*a, *b]));
    }

    let mut zs: Vec<_> = target_to_wires
        .iter()
        .filter(|(address, _)| address.starts_with("z"))
        .collect();
    zs.sort_by(|(a, _), (b, _)| b.cmp(a));

    let mut wire_connections: Vec<_> = Vec::new();
    for z_no in 0..(zs.len() - 1) {
        let z = format!("z{:02}", z_no);
        let mut connections: Vec<_> = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_front((0, z));

        while let Some((distance, wire)) = queue.pop_front() {
            if let Some(next) = target_to_wires.get(wire.as_str()) {
                for name in next.iter() {
                    queue.push_back((distance + 1, name.to_string()));
                }
            }
            connections.push((distance, wire));
        }
        wire_connections.push(connections);
    }
    wire_connections
}

fn confirm_swaps(
    addresses: &HashMap<&str, u8>,
    ops: &mut [Op],
    swaps: Vec<(&str, &str)>,
) -> String {
    let mut mod_ops = ops.to_owned();
    for (_, _, _, target) in mod_ops.iter_mut() {
        for (swap_a, swap_b) in swaps.iter() {
            if target == swap_a {
                *target = swap_b;
            } else if target == swap_b {
                *target = swap_a;
            }
        }
    }

    for bit in 0..=44 {
        let mut addr = addresses.clone();
        for to_change in 0..=44 {
            let x = format!("x{:02}", to_change);
            let y = format!("y{:02}", to_change);
            if bit >= to_change {
                *addr.get_mut(x.as_str()).unwrap() = 1;
                *addr.get_mut(y.as_str()).unwrap() = 1;
            } else {
                *addr.get_mut(x.as_str()).unwrap() = 0;
                *addr.get_mut(y.as_str()).unwrap() = 0;
            }
        }
        let mut binary = vec![1; bit + 1];
        binary[0] = 1;
        let expected: usize = binary
            .into_iter()
            .rev()
            .enumerate()
            .map(|(pow, bit)| bit as usize * 2_usize.pow(pow as u32))
            .sum::<usize>()
            * 2;

        let result = run_ops(&mut addr, &mod_ops);
        if expected != result {
            return format!(
                "Failed to confirm: diff on bit {:?}, expected {:?}, got {:?}",
                bit, expected, result
            );
        }
    }

    let mut wires: Vec<_> = swaps
        .iter()
        .flat_map(|(a, b)| vec![a.to_string(), b.to_string()])
        .collect();
    wires.sort();
    wires.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = "x00: 1\nx01: 1\nx02: 1\ny00: 0\ny01: 1\ny02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";
    const SAMPLE2: &str = "x00: 1\nx01: 0\nx02: 1\nx03: 1
x04: 0\ny00: 1\ny01: 1\ny02: 1\ny03: 1\ny04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";
    const SAMPLE3: &str = "x00: 0\nx01: 1\nx02: 0\nx03: 1\nx04: 0
x05: 1\ny00: 0\ny01: 0\ny02: 1\ny03: 1\ny04: 0\ny05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00";

    #[test]
    fn test_gate_ops() {
        let zero = 0_u8;
        let one = 1_u8;

        assert_eq!(Gate::And.output(one, one), one);
        assert_eq!(Gate::And.output(zero, zero), zero);
        assert_eq!(Gate::And.output(one, zero), zero);
        assert_eq!(Gate::And.output(zero, one), zero);

        assert_eq!(Gate::Or.output(one, one), one);
        assert_eq!(Gate::Or.output(zero, zero), zero);
        assert_eq!(Gate::Or.output(one, zero), one);
        assert_eq!(Gate::Or.output(zero, one), one);

        assert_eq!(Gate::Xor.output(one, one), zero);
        assert_eq!(Gate::Xor.output(zero, zero), zero);
        assert_eq!(Gate::Xor.output(one, zero), one);
        assert_eq!(Gate::Xor.output(zero, one), one);
    }

    #[test]
    fn test_run_ops() {
        let (mut addresses1, ops1) = parse_input(SAMPLE1);
        assert_eq!(run_ops(&mut addresses1, &ops1), 4);

        let (mut addresses2, ops2) = parse_input(SAMPLE2);
        assert_eq!(run_ops(&mut addresses2, &ops2), 2024);
    }

    #[test]
    fn test_follow_wire() {
        let (_, ops) = parse_input(SAMPLE3);

        follow_wires(&ops);
    }
}
