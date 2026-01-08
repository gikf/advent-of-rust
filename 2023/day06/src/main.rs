use std::fs;
use std::path::Path;

fn main() {
    let input = Path::new("2023/day06/src/input.txt");
    let races = parse_races(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Ways to win races, multiplied together: {:?}",
        ways_to_win_races(&races)
    );
    println!("In {:?}", part1.elapsed());

    let race = parse_race(&fs::read_to_string(input).unwrap());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!("Ways to win race: {:?}", ways_to_win_binary(&race));
    println!("In {:?}", part2.elapsed());
}

fn parse_races(input: &str) -> Vec<(usize, usize)> {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|num| num.parse().unwrap());
    let distances = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|num| num.parse().unwrap());
    times.zip(distances).collect()
}

fn parse_race(input: &str) -> (usize, usize) {
    let mut lines = input.lines();
    let time: usize = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .collect::<Vec<_>>()
        .join("")
        .parse()
        .unwrap();
    let distance: usize = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .collect::<Vec<_>>()
        .join("")
        .parse()
        .unwrap();

    (time, distance)
}

fn ways_to_win_race(race: &(usize, usize)) -> usize {
    let (time, distance) = race;

    let mut min_time_charged = 0;
    for time_charged in 1..*time {
        let remaining_time = time - time_charged;
        let traveled = remaining_time * time_charged;
        if traveled > *distance {
            min_time_charged = time_charged;
            break;
        }
    }

    let mut max_time_charged = *time;
    for time_charged in (min_time_charged..*time).rev() {
        let remaining_time = time - time_charged;
        let traveled = remaining_time * time_charged;
        if traveled > *distance {
            max_time_charged = time_charged + 1;
            break;
        }
    }
    max_time_charged - min_time_charged
}

fn ways_to_win_binary(race: &(usize, usize)) -> usize {
    let (time, distance) = race;
    let mut bottom_low = 0;
    let mut bottom_high = *time;

    while bottom_high - bottom_low > 10 {
        let middle = (bottom_high + bottom_low) / 2;
        let remaining_time = time - middle;
        let traveled = remaining_time * middle;
        if traveled > *distance {
            bottom_high = middle - 1;
        } else if traveled < *distance {
            bottom_low = middle + 1;
        } else {
            break;
        }
    }

    let mut min_time_charged = 0;
    for time_charged in bottom_low.. {
        let remaining_time = time - time_charged;
        let traveled = remaining_time * time_charged;
        if traveled > *distance {
            min_time_charged = time_charged;
            break;
        }
    }

    let mut top_low = 0;
    let mut top_high = *time;

    while top_high - top_low > 10 {
        let middle = (top_high + top_low) / 2;
        let remaining_time = time - middle;
        let traveled = remaining_time * middle;
        if traveled > *distance {
            top_low = middle + 1;
        } else if traveled < *distance {
            top_high = middle - 1;
        } else {
            break;
        }
    }

    let mut max_time_charged = *time;
    for time_charged in (0..=top_high).rev() {
        let remaining_time = time - time_charged;
        let traveled = remaining_time * time_charged;
        if traveled > *distance {
            max_time_charged = time_charged + 1;
            break;
        }
    }

    max_time_charged - min_time_charged
}

fn ways_to_win_races(races: &[(usize, usize)]) -> usize {
    races.iter().map(ways_to_win_race).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_parse_races() {
        let races = parse_races(SAMPLE);

        assert_eq!(races, [(7, 9), (15, 40), (30, 200)]);
    }

    #[test]
    fn test_ways_to_win() {
        let races = parse_races(SAMPLE);

        assert_eq!(ways_to_win_race(&races[0]), 4);
        assert_eq!(ways_to_win_race(&races[1]), 8);
        assert_eq!(ways_to_win_race(&races[2]), 9);

        let race = parse_race(SAMPLE);
        assert_eq!(ways_to_win_race(&race), 71503);
    }

    #[test]
    fn test_ways_to_win_races() {
        let races = parse_races(SAMPLE);

        assert_eq!(ways_to_win_races(&races), 288);
    }

    #[test]
    fn test_ways_to_win_race_binary() {
        let races = parse_races(SAMPLE);

        assert_eq!(ways_to_win_binary(&races[0]), 4);
        assert_eq!(ways_to_win_binary(&races[1]), 8);
        assert_eq!(ways_to_win_binary(&races[2]), 9);

        let race = parse_race(SAMPLE);
        assert_eq!(ways_to_win_binary(&race), 71503);
    }
}
