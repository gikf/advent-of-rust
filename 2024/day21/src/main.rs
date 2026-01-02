use std::collections::HashMap;
use std::fs;
use std::path::Path;

mod keypads;
mod robots;

use keypads::{DirectionalKeypad, Keypad};
use robots::{DirectionalRobot, KeypadRobot};

fn main() {
    let input = Path::new("2024/day21/src/input.txt");
    let codes = parse_input(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!("Complexity of codes: {:?}", codes_complexity(&codes, 2));
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Complexity of codes, with 25 directional keypads: {:?}",
        code_keys_complexity(&codes, 25)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_input(input: &str) -> Vec<(usize, Vec<Keypad>)> {
    input
        .lines()
        .map(|line| {
            let numeric_part = line[0..3].parse().unwrap();
            let keys = line.chars().map(Keypad::from).collect();

            (numeric_part, keys)
        })
        .collect()
}

fn code_key_complexity(code: &[Keypad], directional_keypads: usize) -> usize {
    let mut transition_to_press_count: HashMap<(DirectionalKeypad, DirectionalKeypad), usize> =
        HashMap::new();
    let mut transition_to_next_transitions: HashMap<
        (DirectionalKeypad, DirectionalKeypad),
        Vec<(DirectionalKeypad, DirectionalKeypad)>,
    > = HashMap::new();

    for from in DirectionalKeypad::VALUES {
        for to in DirectionalKeypad::VALUES {
            let mut robot = DirectionalRobot::new();
            robot.current_button = from;
            robot.press(&to);
            let from_to = (from, to);
            transition_to_press_count.insert(from_to, robot.record.len());

            let mut next_transistions = vec![(DirectionalKeypad::A, robot.record[0])];
            for window in robot.record.windows(2) {
                let transition_from = window[0];
                let transition_to = window[1];
                next_transistions.push((transition_from, transition_to));
            }
            transition_to_next_transitions.insert(from_to, next_transistions);
        }
    }

    let mut keypad_robot = KeypadRobot::new();
    keypad_robot.press_buttons(code);

    let mut output = vec![DirectionalKeypad::A];
    output.extend(keypad_robot.record);

    let mut transition_to_transition = HashMap::new();

    for window in output.windows(2) {
        let from_to = (window[0], window[1]);
        transition_to_transition
            .entry(from_to)
            .and_modify(|count| *count += 1)
            .or_insert(1_usize);
    }

    for _ in 0..(directional_keypads - 1) {
        let mut next_transitions: HashMap<(DirectionalKeypad, DirectionalKeypad), usize> =
            HashMap::new();

        for (transition, prev_count) in transition_to_transition.drain() {
            if let Some(ns) = transition_to_next_transitions.get(&transition) {
                for pair in ns.iter() {
                    next_transitions
                        .entry(*pair)
                        .and_modify(|count| *count += prev_count)
                        .or_insert(prev_count);
                }
            }
        }
        transition_to_transition = next_transitions;
    }

    transition_to_transition
        .iter()
        .map(|(transition, v)| transition_to_press_count.get(transition).unwrap() * v)
        .sum()
}

fn code_keys_complexity(keys: &[(usize, Vec<Keypad>)], directional_keypads: usize) -> usize {
    keys.iter()
        .map(|(numeric, code)| *numeric * code_key_complexity(code, directional_keypads))
        .sum()
}

fn code_complexity(
    (numeric_part, keys): &(usize, Vec<Keypad>),
    directional_keypads: usize,
) -> usize {
    let mut keypad_robot = KeypadRobot::new();

    keypad_robot.press_buttons(keys);
    let mut input = keypad_robot.record;
    let mut directional_robot = DirectionalRobot::new();
    for _ in 0..directional_keypads {
        directional_robot.press_buttons(&input);
        input = directional_robot.record.clone();
        directional_robot.clear();
    }
    *numeric_part * input.len()
}

fn codes_complexity(codes: &[(usize, Vec<Keypad>)], directional_keypads: usize) -> usize {
    codes
        .iter()
        .map(|code| code_complexity(code, directional_keypads))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "029A
980A
179A
456A
379A";

    #[test]
    fn test_directional_robot() {
        let mut robot = DirectionalRobot::new();
        robot.press(&DirectionalKeypad::Left);
        assert_eq!(robot.current_button, DirectionalKeypad::Left);
        robot.press(&DirectionalKeypad::Right);
        assert_eq!(robot.current_button, DirectionalKeypad::Right);
        robot.press(&DirectionalKeypad::Up);
        assert_eq!(robot.current_button, DirectionalKeypad::Up);
        robot.press(&DirectionalKeypad::Down);
        assert_eq!(robot.current_button, DirectionalKeypad::Down);
        robot.press(&DirectionalKeypad::A);
        assert_eq!(robot.current_button, DirectionalKeypad::A);
    }

    #[test]
    fn test_directional_robot_buttons() {
        let buttons = vec![
            DirectionalKeypad::Left,
            DirectionalKeypad::A,
            DirectionalKeypad::Up,
            DirectionalKeypad::A,
            DirectionalKeypad::Right,
            DirectionalKeypad::Up,
            DirectionalKeypad::Up,
            DirectionalKeypad::A,
            DirectionalKeypad::Down,
            DirectionalKeypad::Down,
            DirectionalKeypad::Down,
            DirectionalKeypad::A,
        ];

        let mut robot = DirectionalRobot::new();
        robot.press_buttons(&buttons);
        assert_eq!(robot.current_button, DirectionalKeypad::A);
        assert_eq!(robot.record.len(), 28);
        let mut second_robot = DirectionalRobot::new();
        second_robot.press_buttons(&robot.record);
        assert_eq!(second_robot.record.len(), 68);
    }

    #[test]
    fn test_keypad_robot() {
        let mut robot = KeypadRobot::new();

        robot.press(&Keypad::Key0);
        assert_eq!(robot.current_button, Keypad::Key0);
        robot.press(&Keypad::Key2);
        assert_eq!(robot.current_button, Keypad::Key2);
        robot.press(&Keypad::Key9);
        assert_eq!(robot.current_button, Keypad::Key9);
        robot.press(&Keypad::A);
        assert_eq!(robot.current_button, Keypad::A);
    }

    #[test]
    fn test_parse_input() {
        let codes = parse_input(SAMPLE);

        assert_eq!(
            codes[0],
            (
                29,
                vec![Keypad::Key0, Keypad::Key2, Keypad::Key9, Keypad::A]
            )
        );
        assert_eq!(
            codes[1],
            (
                980,
                vec![Keypad::Key9, Keypad::Key8, Keypad::Key0, Keypad::A]
            )
        );
        assert_eq!(
            codes[2],
            (
                179,
                vec![Keypad::Key1, Keypad::Key7, Keypad::Key9, Keypad::A]
            )
        );
        assert_eq!(
            codes[3],
            (
                456,
                vec![Keypad::Key4, Keypad::Key5, Keypad::Key6, Keypad::A]
            )
        );
        assert_eq!(
            codes[4],
            (
                379,
                vec![Keypad::Key3, Keypad::Key7, Keypad::Key9, Keypad::A]
            )
        );
    }

    #[test]
    fn test_code_complexity() {
        let codes = parse_input(SAMPLE);

        assert_eq!(code_complexity(&codes[0], 2), 1972);
        assert_eq!(code_complexity(&codes[1], 2), 58800);
        assert_eq!(code_complexity(&codes[2], 2), 12172);
        assert_eq!(code_complexity(&codes[3], 2), 29184);
        assert_eq!(code_complexity(&codes[4], 2), 24256);
    }

    #[test]
    fn test_codes_complexity() {
        let codes = parse_input(SAMPLE);

        assert_eq!(codes_complexity(&codes, 2), 126384);
    }

    #[test]
    fn test_key_complexity() {
        let codes = parse_input(SAMPLE);

        assert_eq!(code_key_complexity(&codes[0].1, 2), 68);
        assert_eq!(code_key_complexity(&codes[1].1, 2), 60);
        assert_eq!(code_key_complexity(&codes[2].1, 2), 68);
        assert_eq!(code_key_complexity(&codes[3].1, 2), 64);
        assert_eq!(code_key_complexity(&codes[4].1, 2), 64);
    }

    #[test]
    fn test_code_keys_complexity() {
        let codes = parse_input(SAMPLE);
        assert_eq!(code_keys_complexity(&codes, 2), 126384);
    }
}
