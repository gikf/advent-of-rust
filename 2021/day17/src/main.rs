use std::fs;
use std::path::Path;

fn main() {
    let input = Path::new("2021/day17/src/input.txt");
    let target_area = parse_target_area(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!("Highest \"y\" reached: {:?}", find_highest_y(target_area));
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Velocities hitting target area: {:?}",
        velocities_hitting_area(target_area)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_target_area(input: &str) -> ((isize, isize), (isize, isize)) {
    let (_, coord) = input.split_once(": x=").unwrap();
    let (x, y) = coord.split_once(", y=").unwrap();
    let (x_start, x_end) = x.split_once("..").unwrap();
    let (y_start, y_end) = y.split_once("..").unwrap();

    (
        (y_start.parse().unwrap(), y_end.parse().unwrap()),
        (x_start.parse().unwrap(), x_end.parse().unwrap()),
    )
}

fn find_highest_y(target_area: ((isize, isize), (isize, isize))) -> isize {
    let (y_min, y_max) = target_area.0;
    let (x_min, x_max) = target_area.1;
    let y_range = y_min..=y_max;
    let x_range = x_min..=x_max;

    let mut highest_y = 0;
    for y in 1..x_max {
        for x in 1..x_max {
            let mut position = (0, 0);
            let mut velocity = (y, x);
            let mut cur_highest = 0;

            loop {
                position = (position.0 + velocity.0, position.1 + velocity.1);

                cur_highest = cur_highest.max(position.0);
                if position.0 < y_min || position.1 > x_max {
                    break;
                }
                if y_range.contains(&position.0) && x_range.contains(&position.1) {
                    highest_y = highest_y.max(cur_highest);
                    break;
                }

                if velocity.1 != 0 {
                    velocity.1 += if velocity.1 > 0 { -1 } else { 1 };
                }
                velocity.0 -= 1;
            }
        }
    }
    highest_y
}

fn velocities_hitting_area(target_area: ((isize, isize), (isize, isize))) -> usize {
    let (y_min, y_max) = target_area.0;
    let (x_min, x_max) = target_area.1;
    let y_range = y_min..=y_max;
    let x_range = x_min..=x_max;

    let mut hits = 0;
    for y in y_min..=x_max {
        for x in 1..=x_max {
            let mut position = (0, 0);
            let mut velocity = (y, x);

            loop {
                position = (position.0 + velocity.0, position.1 + velocity.1);

                if position.0 < y_min || position.1 > x_max {
                    break;
                }
                if y_range.contains(&position.0) && x_range.contains(&position.1) {
                    hits += 1;
                    break;
                }

                if velocity.1 != 0 {
                    velocity.1 += if velocity.1 > 0 { -1 } else { 1 };
                }
                velocity.0 -= 1;
            }
        }
    }
    hits
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn test_parse_target_area() {
        let target_area = parse_target_area(SAMPLE);

        assert_eq!(target_area, ((-10, -5), (20, 30)));
    }

    #[test]
    fn test_part_1_sample() {
        let target_area = parse_target_area(SAMPLE);

        assert_eq!(find_highest_y(target_area), 45);
    }

    #[test]
    fn test_part_2_sample() {
        let target_area = parse_target_area(SAMPLE);

        assert_eq!(velocities_hitting_area(target_area), 112);
    }
}
