use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs;
use std::path::Path;

#[derive(Debug)]
enum Value {
    Number(usize),
    List(Vec<Value>),
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::Number(num1), Value::Number(num2)) => num1.cmp(num2),
            (Value::Number(num), Value::List(_)) => {
                Value::List(vec![Value::Number(*num)]).cmp(other)
            }
            (Value::List(_), Value::Number(num)) => {
                self.cmp(&Value::List(vec![Value::Number(*num)]))
            }
            (Value::List(l1), Value::List(l2)) if l1.is_empty() && !l2.is_empty() => Ordering::Less,
            (Value::List(l1), Value::List(l2)) if l2.is_empty() && !l1.is_empty() => {
                Ordering::Greater
            }
            (Value::List(l1), Value::List(l2)) => {
                let mut it1 = l1.iter();
                let mut it2 = l2.iter();
                loop {
                    match (it1.next(), it2.next()) {
                        (None, None) => {
                            return Ordering::Equal;
                        }
                        (None, Some(_)) => {
                            return Ordering::Less;
                        }
                        (Some(_), None) => {
                            return Ordering::Greater;
                        }
                        (Some(a), Some(b)) => {
                            let comparation = (*a).cmp(b);
                            if matches!(comparation, Ordering::Less | Ordering::Greater) {
                                return comparation;
                            } else {
                                continue;
                            }
                        }
                    };
                }
            }
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.cmp(other), Ordering::Equal)
    }
}

impl Eq for Value {}

fn main() {
    let input = Path::new("2022/day13/src/input.txt");
    let pairs = parse_pairs(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Sum of indices of ordered pairs: {:?}",
        compare_pairs(&pairs)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!("Decoder key for distress signal: {:?}", decoder_key(pairs));
    println!("In {:?}", part2.elapsed());
}

fn parse_pairs(input: &str) -> Vec<(Value, Value)> {
    let mut lines = input.lines();
    let mut pairs = Vec::new();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            continue;
        }

        let a = parse_values(line);
        let b = parse_values(lines.next().unwrap());

        pairs.push((a, b));
    }

    pairs
}

fn parse_values(line: &str) -> Value {
    let mut stack = Vec::new();

    line.split(',').for_each(|mut part| {
        while !part.is_empty() {
            if part.starts_with("[") {
                stack.push(Value::List(Vec::with_capacity(0)));
                part = &part[1..];
            } else if part.starts_with("]") {
                let mut items = Vec::new();
                loop {
                    if let Value::List(l) = stack.last().unwrap()
                        && l.capacity() == 0
                    {
                        break;
                    }
                    items.push(stack.pop().unwrap());
                }

                let _ = stack.pop();

                let mut it = Vec::with_capacity(1);
                it.extend(items.into_iter().rev());
                let list = Value::List(it);
                stack.push(list);
                part = &part[1..];
            } else if let Some(index) = part.chars().position(|c| !c.is_numeric()) {
                stack.push(Value::Number(part[..index].parse().unwrap()));
                part = &part[index..];
            } else {
                stack.push(Value::Number(part.parse().unwrap()));
                break;
            }
        }
    });
    stack.pop().unwrap()
}

fn compare_pairs(pairs: &[(Value, Value)]) -> usize {
    let mut sum_indices_of_ordered = 0;
    for (pair_no, (a, b)) in (1..).zip(pairs.iter()) {
        if *a < *b {
            sum_indices_of_ordered += pair_no;
        }
    }
    sum_indices_of_ordered
}

const MARKER1: &str = "[[2]]";
const MARKER2: &str = "[[6]]";

fn decoder_key(pairs: Vec<(Value, Value)>) -> usize {
    let mut packets = BinaryHeap::new();
    for (a, b) in pairs {
        packets.push(a);
        packets.push(b);
    }

    for item in [MARKER1, MARKER2] {
        let packet = parse_values(item);
        packets.push(packet);
    }

    let two = parse_values(MARKER1);
    let six = parse_values(MARKER2);

    let mut indices = Vec::new();
    for (index, packet) in (1..).zip(packets.into_sorted_vec().into_iter()) {
        if packet == two || packet == six {
            indices.push(index);
        }
    }
    indices.into_iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_parse_values() {
        assert_eq!(
            parse_values("[1,1,3,1,1]"),
            Value::List(vec![
                Value::Number(1),
                Value::Number(1),
                Value::Number(3),
                Value::Number(1),
                Value::Number(1)
            ])
        );

        assert_eq!(
            parse_values("[[1],4]"),
            Value::List(vec![Value::List(vec![Value::Number(1)]), Value::Number(4),])
        );

        assert_eq!(
            parse_values("[[1],[2,3,4]]"),
            Value::List(vec![
                Value::List(vec![Value::Number(1)]),
                Value::List(vec![Value::Number(2), Value::Number(3), Value::Number(4)]),
            ])
        );

        assert_eq!(parse_values("[9]"), Value::List(vec![Value::Number(9),]));

        assert_eq!(
            parse_values("[[8,7,6]]"),
            Value::List(vec![Value::List(vec![
                Value::Number(8),
                Value::Number(7),
                Value::Number(6)
            ]),])
        );

        assert_eq!(
            parse_values("[[4,4],4,4]"),
            Value::List(vec![
                Value::List(vec![Value::Number(4), Value::Number(4)]),
                Value::Number(4),
                Value::Number(4),
            ])
        );

        assert_eq!(parse_values("[]"), Value::List(vec![]));

        assert_eq!(
            parse_values("[[[]]]"),
            Value::List(vec![Value::List(vec![Value::List(vec![])])])
        );

        assert_eq!(
            parse_values("[1,[2,[3,[4,[5,6,7]]]],8,9]"),
            Value::List(vec![
                Value::Number(1),
                Value::List(vec![
                    Value::Number(2),
                    Value::List(vec![
                        Value::Number(3),
                        Value::List(vec![
                            Value::Number(4),
                            Value::List(
                                vec![Value::Number(5), Value::Number(6), Value::Number(7),]
                            )
                        ])
                    ])
                ]),
                Value::Number(8),
                Value::Number(9),
            ])
        );
    }

    #[test]
    fn test_compare() {
        let a = parse_values("[[1],[2,3,4]]");
        let b = parse_values("[[1],4]");
        assert_eq!(a < b, true);

        let a = parse_values("[7,7,7]");
        let b = parse_values("[9]");
        assert_eq!(a < b, true);

        let a = parse_values("[7,7,7,7]");
        let b = parse_values("[[8,7,6]]");
        assert_eq!(a < b, true);
    }

    #[test]
    fn test_part_1_sample() {
        let pairs = parse_pairs(SAMPLE);

        assert_eq!(compare_pairs(&pairs), 13);
    }

    #[test]
    fn test_part_2_sample() {
        let pairs = parse_pairs(SAMPLE);

        assert_eq!(decoder_key(pairs), 140);
    }
}
