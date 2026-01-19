use std::collections::VecDeque;
use std::fs;
use std::path::Path;

const ROUNDS_PART1: usize = 20;
const ROUNDS_PART2: usize = 10_000;

struct Monkey {
    items: VecDeque<usize>,
    condition: usize,
    operation: Box<dyn Fn(usize) -> usize>,
    target_true: usize,
    target_false: usize,
    times_inspected: usize,
}

fn main() {
    let input = Path::new("2022/day11/src/input.txt");
    let mut monkeys = parse_monkeys(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Monkey business level: {:?}",
        monkey_business_level(&mut monkeys, ROUNDS_PART1)
    );
    println!("In {:?}", part1.elapsed());

    let mut monkeys = parse_monkeys(&fs::read_to_string(input).unwrap());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Monkey business level with less worry: {:?}",
        monkey_business_level_less_worry(&mut monkeys, ROUNDS_PART2)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    let mut lines = input.lines();

    let mut monkeys = Vec::new();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            continue;
        }

        let (_, items) = lines.next().unwrap().split_once(": ").unwrap();
        let items: VecDeque<usize> = items
            .split(", ")
            .map(|item| item.parse().unwrap())
            .collect();

        let (_, operation) = lines.next().unwrap().split_once(" = ").unwrap();

        let parts: Vec<_> = operation.split_ascii_whitespace().collect();

        let op = match parts[1] {
            "+" => sum_op,
            "*" => mul_op,
            _ => unimplemented!(),
        };

        let func: Box<dyn Fn(usize) -> usize> = if parts[2] == "old" {
            Box::new(move |old| op(old, old))
        } else {
            let other: usize = parts[2].parse().unwrap();
            Box::new(move |old| op(old, other))
        };

        let (_, test) = lines.next().unwrap().rsplit_once(" ").unwrap();

        let (_, if_true) = lines.next().unwrap().rsplit_once(" ").unwrap();
        let (_, if_false) = lines.next().unwrap().rsplit_once(" ").unwrap();

        monkeys.push(Monkey {
            items,
            condition: test.parse().unwrap(),
            operation: func,
            target_true: if_true.parse().unwrap(),
            target_false: if_false.parse().unwrap(),
            times_inspected: 0,
        })
    }
    monkeys
}

fn sum_op(a: usize, b: usize) -> usize {
    a + b
}

fn mul_op(a: usize, b: usize) -> usize {
    a * b
}

fn monkey_business_level(monkeys: &mut [Monkey], rounds: usize) -> usize {
    for _ in 0..rounds {
        let mut current_monkey = 0;
        while current_monkey < monkeys.len() {
            let mut thrown_true = Vec::new();
            let mut thrown_false = Vec::new();

            let monkey = monkeys.get_mut(current_monkey).unwrap();

            let throw_true_to = monkey.target_true;
            let throw_false_to = monkey.target_false;

            for item in monkey.items.drain(..) {
                monkey.times_inspected += 1;
                let worry_level = (monkey.operation)(item) / 3;
                if worry_level % monkey.condition == 0 {
                    thrown_true.push(worry_level);
                } else {
                    thrown_false.push(worry_level);
                }
            }

            if !thrown_true.is_empty() {
                monkeys
                    .get_mut(throw_true_to)
                    .unwrap()
                    .items
                    .extend(thrown_true);
            }
            if !thrown_false.is_empty() {
                monkeys
                    .get_mut(throw_false_to)
                    .unwrap()
                    .items
                    .extend(thrown_false);
            }

            current_monkey += 1;
        }
    }

    monkeys.sort_by(|a, b| b.times_inspected.cmp(&a.times_inspected));

    monkeys[0].times_inspected * monkeys[1].times_inspected
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
    b
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn monkey_business_level_less_worry(monkeys: &mut [Monkey], rounds: usize) -> usize {
    let lcm = monkeys
        .iter()
        .map(|monkey| monkey.condition)
        .reduce(lcm)
        .unwrap();

    for _ in 0..rounds {
        let mut current_monkey = 0;
        while current_monkey < monkeys.len() {
            let mut thrown_true = Vec::new();
            let mut thrown_false = Vec::new();

            let monkey = monkeys.get_mut(current_monkey).unwrap();

            let throw_true_to = monkey.target_true;
            let throw_false_to = monkey.target_false;

            for item in monkey.items.drain(..) {
                monkey.times_inspected += 1;
                let worry_level = (monkey.operation)(item) % lcm;
                if worry_level % monkey.condition == 0 {
                    thrown_true.push(worry_level);
                } else {
                    thrown_false.push(worry_level);
                }
            }

            if !thrown_true.is_empty() {
                monkeys
                    .get_mut(throw_true_to)
                    .unwrap()
                    .items
                    .extend(thrown_true);
            }
            if !thrown_false.is_empty() {
                monkeys
                    .get_mut(throw_false_to)
                    .unwrap()
                    .items
                    .extend(thrown_false);
            }

            current_monkey += 1;
        }
    }

    monkeys.sort_by(|a, b| b.times_inspected.cmp(&a.times_inspected));

    monkeys[0].times_inspected * monkeys[1].times_inspected
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_part_1_sample() {
        let mut monkeys = parse_monkeys(SAMPLE);

        assert_eq!(monkey_business_level(&mut monkeys, ROUNDS_PART1), 10605);
    }

    #[test]
    fn test_part_2_sample() {
        let mut monkeys = parse_monkeys(SAMPLE);

        assert_eq!(
            monkey_business_level_less_worry(&mut monkeys, ROUNDS_PART2),
            2713310158
        );
    }
}
