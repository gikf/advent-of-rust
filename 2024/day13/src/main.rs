use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq)]
struct Machine {
    button_a: (f64, f64),
    button_b: (f64, f64),
    prize: (f64, f64),
}

const BUTTON_A_PRESS: usize = 3;
const BUTTON_B_PRESS: usize = 1;

const BUTTON_PRESS_LIMIT: f64 = 100.;

fn main() {
    let input = Path::new("2024/day13/src/input.txt");
    let machines = parse_machines(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Fewest tokens to win all possible prizes: {:?}",
        fewest_tokens(&machines, button_presses)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Fewest tokens to win all possible prizes, with corrected button presses: {:?}",
        fewest_tokens(&machines, corrected_presses)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_machines(input: &str) -> Vec<Machine> {
    let mut machines = Vec::new();

    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            continue;
        }

        if line.starts_with("Button A") {
            let button_a = parse_button(line);
            let button_b = parse_button(lines.next().unwrap());
            let prize = parse_prize(lines.next().unwrap());
            machines.push(Machine {
                button_a,
                button_b,
                prize,
            })
        }
    }

    machines
}

fn parse_button(line: &str) -> (f64, f64) {
    let x_start = line.find("X+").unwrap();
    let x_end = line.find(",").unwrap();
    let y_start = line.find("Y+").unwrap();

    let x = line[(x_start + 2)..x_end].parse().unwrap();
    let y = line[(y_start + 2)..].parse().unwrap();

    (x, y)
}

fn parse_prize(line: &str) -> (f64, f64) {
    let x_start = line.find("X=").unwrap();
    let x_end = line.find(",").unwrap();
    let y_start = line.find("Y=").unwrap();

    let x = line[(x_start + 2)..x_end].parse().unwrap();
    let y = line[(y_start + 2)..].parse().unwrap();

    (x, y)
}

fn fewest_tokens<PressesCalculator: Fn(&Machine) -> Option<(usize, usize)>>(
    machines: &[Machine],
    button_calc: PressesCalculator,
) -> usize {
    machines
        .iter()
        .filter_map(button_calc)
        .map(|(a_presses, b_presses)| needed_tokens(a_presses, b_presses))
        .sum()
}

fn needed_tokens(a_presses: usize, b_presses: usize) -> usize {
    a_presses * BUTTON_A_PRESS + b_presses * BUTTON_B_PRESS
}

fn solve_reduced((ax, ay): (f64, f64), (bx, by): (f64, f64), (p1, p2): (f64, f64)) -> (f64, f64) {
    /*
     * Reduced equations:
     * a*ax + b*bx = p1
     * a*ay + b*by = p2
     */

    let b = (p2 - p1 / ax * ay) / (by - bx / ax * ay);
    let a = (p1 - b * bx) / ax;

    (a, b)
}

fn button_presses(machine: &Machine) -> Option<(usize, usize)> {
    let p1 = machine.prize.0;
    let p2 = machine.prize.1;
    let (ax, ay) = machine.button_a;
    let (bx, by) = machine.button_b;

    if (ax + bx) * BUTTON_PRESS_LIMIT < p1 || (ay + by) * BUTTON_PRESS_LIMIT < p2 {
        return None;
    }

    let (a, b) = solve_reduced((ax, ay), (bx, by), (p1, p2));
    if !(0. ..BUTTON_PRESS_LIMIT).contains(&a) || !(0. ..BUTTON_PRESS_LIMIT).contains(&b) {
        return None;
    }

    let a_presses = a.round();
    let b_presses = b.round();
    if a_presses * ax + b_presses * bx != p1 || a_presses * ay + b_presses * by != p2 {
        None
    } else {
        Some((a_presses as usize, b_presses as usize))
    }
}

fn corrected_presses(machine: &Machine) -> Option<(usize, usize)> {
    let p1 = 10_000_000_000_000. + machine.prize.0;
    let p2 = 10_000_000_000_000. + machine.prize.1;
    let (ax, ay) = machine.button_a;
    let (bx, by) = machine.button_b;

    let (a, b) = solve_reduced((ax, ay), (bx, by), (p1, p2));
    if a < 0. || b < 0. {
        return None;
    }

    let a_presses = a.round();
    let b_presses = b.round();
    if a_presses * ax + b_presses * bx != p1 || a_presses * ay + b_presses * by != p2 {
        None
    } else {
        Some((a_presses as usize, b_presses as usize))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_parse_machines() {
        let machines = parse_machines(SAMPLE);

        assert_eq!(
            machines,
            vec![
                Machine {
                    button_a: (94., 34.),
                    button_b: (22., 67.),
                    prize: (8400., 5400.)
                },
                Machine {
                    button_a: (26., 66.),
                    button_b: (67., 21.),
                    prize: (12748., 12176.)
                },
                Machine {
                    button_a: (17., 86.),
                    button_b: (84., 37.),
                    prize: (7870., 6450.)
                },
                Machine {
                    button_a: (69., 23.),
                    button_b: (27., 71.),
                    prize: (18641., 10279.)
                },
            ]
        )
    }

    #[test]
    fn test_button_presses() {
        let machines = parse_machines(SAMPLE);

        assert_eq!(button_presses(&machines[0]), Some((80, 40)));
        assert_eq!(button_presses(&machines[1]), None);
        assert_eq!(button_presses(&machines[2]), Some((38, 86)));
        assert_eq!(button_presses(&machines[3]), None);
    }

    #[test]
    fn test_needed_tokens() {
        assert_eq!(needed_tokens(80, 40), 280);
        assert_eq!(needed_tokens(38, 86), 200);
    }

    #[test]
    fn test_part_1_sample() {
        let machines = parse_machines(SAMPLE);

        assert_eq!(fewest_tokens(&machines, button_presses), 480);
    }

    #[test]
    fn test_part_2_sample() {
        let machines = parse_machines(SAMPLE);

        assert_eq!(fewest_tokens(&machines, corrected_presses), 875318608908);
    }
}
