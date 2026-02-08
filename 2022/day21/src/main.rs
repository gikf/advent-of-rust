use std::collections::HashMap;
use std::fs;
use std::path::Path;

const START_MONKEY: &str = "root";
const YOU_MONKEY: &str = "humn";

#[derive(Debug, PartialEq)]
enum Yell {
    Number(isize),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

fn main() {
    let input = Path::new("2022/day21/src/input.txt");
    let contents = fs::read_to_string(input).unwrap();
    let monkeys = parse_monkeys(&contents);

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Number yelled by monkey named \"root\": {:?}",
        monkey_yell(&monkeys, START_MONKEY)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Number to yell, so \"root\" monkey's equality test passes: {:?}",
        what_to_yell(&monkeys, START_MONKEY, YOU_MONKEY)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_monkeys(input: &str) -> HashMap<&str, Yell> {
    let mut monkeys = HashMap::new();

    for line in input.lines() {
        let (name, rest) = line.split_once(": ").unwrap();
        let split: Vec<_> = rest.split_ascii_whitespace().collect();

        if split.len() == 1 {
            monkeys.insert(name, Yell::Number(split[0].parse().unwrap()));
        } else {
            let name1 = split[0].to_string();
            let name2 = split[2].to_string();
            match split[1] {
                "+" => {
                    monkeys.insert(name, Yell::Add(name1, name2));
                }
                "-" => {
                    monkeys.insert(name, Yell::Sub(name1, name2));
                }
                "/" => {
                    monkeys.insert(name, Yell::Div(name1, name2));
                }
                "*" => {
                    monkeys.insert(name, Yell::Mul(name1, name2));
                }
                _ => unimplemented!(),
            }
        }
    }

    monkeys
}

fn monkey_yell(monkeys: &HashMap<&str, Yell>, target_monkey: &str) -> isize {
    follow_monkey(target_monkey, monkeys)
}

fn follow_monkey(cur_monkey: &str, monkeys: &HashMap<&str, Yell>) -> isize {
    let yell = monkeys.get(cur_monkey).unwrap();
    match yell {
        Yell::Number(num) => *num,
        Yell::Add(a, b) => follow_monkey(a, monkeys) + follow_monkey(b, monkeys),
        Yell::Sub(a, b) => follow_monkey(a, monkeys) - follow_monkey(b, monkeys),
        Yell::Mul(a, b) => follow_monkey(a, monkeys) * follow_monkey(b, monkeys),
        Yell::Div(a, b) => follow_monkey(a, monkeys) / follow_monkey(b, monkeys),
    }
}

fn what_to_yell(monkeys: &HashMap<&str, Yell>, start_monkey: &str, you: &str) -> isize {
    let root = monkeys.get(start_monkey).unwrap();
    let (left, right) = match root {
        Yell::Add(a, b) | Yell::Div(a, b) | Yell::Mul(a, b) | Yell::Sub(a, b) => (a, b),
        _ => unimplemented!(),
    };

    let (mut cur_result, mut cur_monkey) = if is_contained(you, left, monkeys) {
        (follow_monkey(right.as_str(), monkeys), left.as_str())
    } else {
        (follow_monkey(left.as_str(), monkeys), right.as_str())
    };

    while cur_monkey != you {
        cur_monkey = match monkeys.get(cur_monkey).unwrap() {
            Yell::Add(a, b) => {
                if is_contained(you, a, monkeys) {
                    cur_result -= follow_monkey(b, monkeys);
                    a
                } else {
                    cur_result -= follow_monkey(a, monkeys);
                    b
                }
            }
            Yell::Div(a, b) => {
                if is_contained(you, a, monkeys) {
                    cur_result *= follow_monkey(b, monkeys);
                    a
                } else {
                    cur_result /= follow_monkey(a, monkeys);
                    b
                }
            }
            Yell::Mul(a, b) => {
                if is_contained(you, a, monkeys) {
                    cur_result /= follow_monkey(b, monkeys);
                    a
                } else {
                    cur_result /= follow_monkey(a, monkeys);
                    b
                }
            }
            Yell::Sub(a, b) => {
                if is_contained(you, a, monkeys) {
                    cur_result += follow_monkey(b, monkeys);
                    a
                } else {
                    cur_result = follow_monkey(a, monkeys) - cur_result;
                    b
                }
            }
            _ => unimplemented!(),
        };
    }
    cur_result
}

fn is_contained(target: &str, start_monkey: &str, monkeys: &HashMap<&str, Yell>) -> bool {
    if start_monkey == target {
        true
    } else {
        let yell = monkeys.get(start_monkey).unwrap();
        match yell {
            Yell::Add(a, b) | Yell::Div(a, b) | Yell::Mul(a, b) | Yell::Sub(a, b) => {
                is_contained(target, a, monkeys) || is_contained(target, b, monkeys)
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn test_part_1_sample() {
        let monkeys = parse_monkeys(SAMPLE);

        assert_eq!(monkey_yell(&monkeys, START_MONKEY), 152);
    }

    #[test]
    fn test_part_2_sample() {
        let monkeys = parse_monkeys(SAMPLE);

        assert_eq!(what_to_yell(&monkeys, START_MONKEY, YOU_MONKEY), 301);
    }
}
