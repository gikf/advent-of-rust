use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::path::Path;
use std::thread;

type Lights = Vec<bool>;
type Buttons = Vec<Vec<usize>>;
type Joltages = Vec<usize>;

type Machine = (Lights, Buttons, Joltages);

fn main() {
    let input = Path::new("2025/day10/src/input.txt");
    let machines = parse_machines(&fs::read_to_string(input).unwrap());

    println!("Step 1");
    println!(
        "Fewest button presses to configure lights: {:?}",
        fewest_presses(&machines)
    );

    println!("Step 2");
    println!(
        "Fewest button presses to configure joltage: {:?}",
        fewest_joltage_presses(&machines)
    );
}

fn fewest_joltage_presses(machines: &[Machine]) -> usize {
    let thread_handles: Vec<_> = machines
        .iter()
        .map(|machine| {
            let machine = machine.clone();
            thread::spawn(move || find_joltage_presses(machine))
        })
        .collect();

    thread_handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .sum()
}

fn fewest_joltage_presses_single_thread(machines: &[Machine]) -> usize {
    machines
        .iter()
        .map(|machine| find_joltage_presses(machine.clone()))
        .sum()
}

fn find_joltage_presses(machine: Machine) -> usize {
    // Based on brilliance: https://old.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/

    let (_, buttons, target) = machine;
    let buttons_to_presses = buttons
        .iter()
        .map(|buttons_set| {
            (0..target.len())
                .map(|index| if buttons_set.contains(&index) { 1 } else { 0 })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let joltage_patterns = patterns(&buttons_to_presses);

    let mut _calculated: HashMap<Vec<usize>, usize> = HashMap::new();

    solve_joltages(&target, &joltage_patterns, &mut _calculated)
}

fn solve_joltages(
    target: &[usize],
    possible_presses: &HashMap<Vec<usize>, usize>,
    _calculated: &mut HashMap<Vec<usize>, usize>,
) -> usize {
    if target.iter().all(|val| *val == 0) {
        return 0;
    }

    if let Some(result) = _calculated.get(target) {
        return *result;
    }

    let mut result = 10_000;
    for (pattern, pattern_value) in possible_presses {
        if pattern
            .iter()
            .zip(target.iter())
            .all(|(value, expected)| value <= expected && value % 2 == expected % 2)
        {
            let next_target = pattern
                .iter()
                .zip(target.iter())
                .map(|(value, expected)| (expected - value).div_euclid(2))
                .collect::<Vec<_>>();
            result = result.min(
                *pattern_value + 2 * solve_joltages(&next_target, possible_presses, _calculated),
            );
        }
    }
    let _ = &_calculated.insert(target.to_vec(), result);
    result
}

fn combinations(items: &[usize], required_count: usize) -> Vec<Vec<usize>> {
    // https://docs.python.org/3/library/itertools.html#itertools.combinations

    let mut result = Vec::new();
    if required_count > items.len() {
        return result;
    }

    let mut indices = (0..required_count).collect::<Vec<_>>();

    result.push(
        indices
            .iter()
            .map(|index| items[*index])
            .collect::<Vec<_>>(),
    );

    loop {
        let mut broken = false;
        let mut i = 0;
        for index in (0..required_count).rev() {
            i = index;
            if indices[index] != index + items.len() - required_count {
                broken = true;
                break;
            }
        }

        if !broken {
            break;
        }

        indices[i] += 1;

        for j in (i + 1)..required_count {
            indices[j] = indices[j - 1] + 1
        }
        result.push(
            indices
                .iter()
                .map(|index| items[*index])
                .collect::<Vec<_>>(),
        );
    }
    result
}

fn patterns(machine_buttons: &[Vec<usize>]) -> HashMap<Vec<usize>, usize> {
    let mut unique_patterns = HashMap::new();
    for number_of_buttons_pressed in 0..=machine_buttons.len() {
        for button_presses in combinations(
            &(0..machine_buttons.len()).collect::<Vec<_>>(),
            number_of_buttons_pressed,
        ) {
            let pattern = (0..(machine_buttons[0].len()))
                .enumerate()
                .map(|(index, _)| {
                    button_presses
                        .iter()
                        .map(|button| machine_buttons[*button][index])
                        .sum::<usize>()
                })
                .collect::<Vec<_>>();
            unique_patterns
                .entry(pattern)
                .or_insert(number_of_buttons_pressed);
        }
    }
    unique_patterns
}

fn fewest_presses(machines: &[Machine]) -> usize {
    machines
        .iter()
        .map(|machine| find_machine_presses(machine.clone()))
        .sum()
}

fn find_machine_presses(machine: Machine) -> usize {
    let (target_lights, buttons_set, _) = machine;
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();

    let start: Vec<bool> = (0..target_lights.len()).map(|_| false).collect();

    queue.push_back((start, 0));

    while !queue.is_empty() {
        let (current_lights, presses) = queue.pop_front().unwrap();

        if seen.contains(&current_lights) {
            continue;
        }
        seen.insert(current_lights.clone());

        for buttons in &buttons_set {
            let mut next_lights = current_lights.clone();
            for button in buttons {
                next_lights[*button] = !next_lights[*button];
            }
            if next_lights == target_lights {
                return presses + 1;
            }

            queue.push_back((next_lights, presses + 1));
        }
    }

    0
}

fn parse_machines(input: &str) -> Vec<Machine> {
    let lines = input.lines();

    let machines: Vec<_> = lines
        .map(|line| {
            let split: Vec<_> = line.split_ascii_whitespace().collect();
            let target_lights: Vec<_> = split
                .first()
                .unwrap()
                .trim_matches(['[', ']'])
                .chars()
                .map(|c| match c {
                    '.' => false,
                    '#' => true,
                    _ => unreachable!(),
                })
                .collect();
            let buttons_sets: Vec<_> = split
                .iter()
                .skip(1)
                .take(split.len() - 2)
                .map(|buttons| {
                    buttons
                        .trim_matches(['(', ')'])
                        .split(',')
                        .map(|pressing| pressing.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect();
            let target_joltages = split
                .last()
                .unwrap()
                .trim_matches(['{', '}'])
                .split(',')
                .map(|num| num.parse::<usize>().unwrap())
                .collect();

            (target_lights, buttons_sets, target_joltages)
        })
        .collect();

    machines
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}\n[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_parse_machines() {
        let result = parse_machines(SAMPLE);
        assert_eq!(
            result,
            [
                (
                    Vec::from([false, true, true, false]),
                    Vec::from([
                        Vec::from([3]),
                        Vec::from([1, 3]),
                        Vec::from([2]),
                        Vec::from([2, 3]),
                        Vec::from([0, 2]),
                        Vec::from([0, 1])
                    ]),
                    Vec::from([3, 5, 4, 7])
                ),
                (
                    Vec::from([false, false, false, true, false]),
                    Vec::from([
                        Vec::from([0, 2, 3, 4]),
                        Vec::from([2, 3]),
                        Vec::from([0, 4]),
                        Vec::from([0, 1, 2]),
                        Vec::from([1, 2, 3, 4])
                    ]),
                    Vec::from([7, 5, 12, 7, 2])
                ),
                (
                    Vec::from([false, true, true, true, false, true]),
                    Vec::from([
                        Vec::from([0, 1, 2, 3, 4]),
                        Vec::from([0, 3, 4]),
                        Vec::from([0, 1, 2, 4, 5]),
                        Vec::from([1, 2])
                    ]),
                    Vec::from([10, 11, 11, 5, 10, 5])
                ),
            ]
        );
    }

    #[test]
    fn test_fewest_total_presses_step_1() {
        let machines = parse_machines(SAMPLE);
        assert_eq!(fewest_presses(&machines), 7);
    }

    #[test]
    fn test_fewest_joltage_presses_step_2() {
        let machines = parse_machines(SAMPLE);
        assert_eq!(fewest_joltage_presses(&machines), 33);
    }
}
