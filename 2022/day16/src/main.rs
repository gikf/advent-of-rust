use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::path::Path;

fn main() {
    let input = Path::new("2022/day16/src/input.txt");
    let contents = fs::read_to_string(input).unwrap();
    let valves = parse_valves(&contents);

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Most pressure to release in 30 minutes: {:?}",
        release_pressure(&valves)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Most pressure to release with elephant in 26 minutes: {:?}",
        release_pressure_with_elephant_joined(&valves)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_valves(input: &str) -> HashMap<&str, (usize, Vec<&str>)> {
    let mut valves: HashMap<&str, (usize, Vec<&str>)> = HashMap::new();
    input.lines().for_each(|line| {
        let (valve, tunnels) = line.split_once(";").unwrap();

        let mut split_valve = valve.split_ascii_whitespace();

        let name = split_valve.nth(1).unwrap();
        let rate = split_valve.last().unwrap().trim_start_matches("rate=");

        let (_, tunnels) = tunnels.split_once(" to ").unwrap();
        let (_, tunnels) = tunnels.split_once(" ").unwrap();
        let tunnels: Vec<_> = tunnels.split(", ").collect();

        valves.insert(name, (rate.parse().unwrap(), tunnels));
    });
    valves
}

fn release_pressure(valves: &HashMap<&str, (usize, Vec<&str>)>) -> usize {
    let mut queue: VecDeque<(&str, usize, usize, Vec<&str>)> = VecDeque::with_capacity(200);
    queue.push_back(("AA", 0, 30, Vec::with_capacity(valves.len())));
    let mut max_released = 0;

    let mut visited = HashSet::with_capacity(2_000_000);
    let non_zero_valves = valves.values().filter(|(rate, _)| *rate > 0).count();

    while let Some((cur_valve, pressure, minutes_left, open_valves)) = queue.pop_back() {
        if !visited.insert((cur_valve, pressure, minutes_left)) {
            continue;
        } else if minutes_left == 0 {
            max_released = max_released.max(pressure);
            continue;
        } else if open_valves.len() == non_zero_valves {
            let total_pressure =
                pressure + minutes_left * valves.values().map(|(rate, _)| *rate).sum::<usize>();
            max_released = max_released.max(total_pressure);
            continue;
        }

        let next_pressure = pressure
            + open_valves
                .iter()
                .map(|v| valves.get(v).unwrap().0)
                .sum::<usize>();

        let (valve_rate, tunnels) = valves.get(&cur_valve).unwrap();
        if *valve_rate != 0 && !open_valves.contains(&cur_valve) {
            let mut next_open = open_valves.clone();
            next_open.push(cur_valve);
            queue.push_back((cur_valve, next_pressure, minutes_left - 1, next_open));
        }

        for tunnel in tunnels.iter() {
            let next_open = open_valves.clone();
            queue.push_back((tunnel, next_pressure, minutes_left - 1, next_open));
        }
    }
    max_released
}

fn release_part_of_pressure<'a>(
    valves: &'a HashMap<&str, (usize, Vec<&str>)>,
    used_valves: Vec<&str>,
    number_of_valves_to_use: Option<usize>,
) -> (usize, Vec<&'a str>) {
    let mut queue: VecDeque<(&str, usize, usize, Vec<&str>)> = VecDeque::with_capacity(200);
    queue.push_back(("AA", 0, 26, Vec::with_capacity(valves.len())));
    let mut max_released = 0;
    let mut taken_valves = Vec::with_capacity(valves.len());

    let mut visited = HashSet::with_capacity(2_000_000);
    let non_zero_valves = valves.values().filter(|(rate, _)| *rate > 0).count();

    while let Some((cur_valve, pressure, minutes_left, open_valves)) = queue.pop_back() {
        if !visited.insert((cur_valve, pressure, minutes_left)) {
            continue;
        } else if minutes_left == 0 {
            if max_released < pressure {
                max_released = pressure;
                taken_valves = open_valves;
            }
            continue;
        } else if open_valves.len() == non_zero_valves
            || number_of_valves_to_use.is_some_and(|v| open_valves.len() == v)
        {
            let total_pressure =
                pressure + minutes_left * valves.values().map(|(rate, _)| *rate).sum::<usize>();
            if max_released < pressure {
                max_released = total_pressure;
                taken_valves = open_valves;
            }
            continue;
        }

        let next_pressure = pressure
            + open_valves
                .iter()
                .map(|v| valves.get(v).unwrap().0)
                .sum::<usize>();

        let (valve_rate, tunnels) = valves.get(&cur_valve).unwrap();
        if *valve_rate != 0
            && !used_valves.contains(&cur_valve)
            && !open_valves.contains(&cur_valve)
        {
            let mut next_open = open_valves.clone();
            next_open.push(cur_valve);
            queue.push_back((cur_valve, next_pressure, minutes_left - 1, next_open));
        }

        for tunnel in tunnels.iter() {
            let next_open = open_valves.clone();
            queue.push_back((tunnel, next_pressure, minutes_left - 1, next_open));
        }
    }
    (max_released, taken_valves)
}

fn release_pressure_with_elephant_joined(valves: &HashMap<&str, (usize, Vec<&str>)>) -> usize {
    let (part1, taken_valves) = release_part_of_pressure(valves, Vec::new(), None);
    let (part2, _) = release_part_of_pressure(valves, taken_valves, None);

    part1 + part2
}

#[allow(unused)]
fn release_pressure_with_elephant(valves: &HashMap<&str, (usize, Vec<&str>)>) -> usize {
    let mut queue: VecDeque<(&str, &str, usize, usize, Vec<&str>)> = VecDeque::with_capacity(200);
    queue.push_back(("AA", "AA", 0, 26, Vec::with_capacity(valves.len())));
    let mut max_released = 0;

    let mut visited = HashSet::with_capacity(2_000_000);
    let non_zero_valves = valves.values().filter(|(rate, _)| *rate > 0).count();
    let mut queue_part = Vec::with_capacity(10);

    while let Some((cur_valve, cur_elephant, pressure, minutes_left, open_valves)) =
        queue.pop_back()
    {
        if !visited.insert((cur_valve, cur_elephant, pressure, minutes_left))
            || (minutes_left != 26
                && !visited.insert((cur_elephant, cur_valve, pressure, minutes_left)))
        {
            continue;
        } else if minutes_left == 0 {
            max_released = max_released.max(pressure);
            continue;
        } else if open_valves.len() == non_zero_valves {
            let total_pressure =
                pressure + minutes_left * valves.values().map(|(rate, _)| *rate).sum::<usize>();
            max_released = max_released.max(total_pressure);
            continue;
        }

        let (valve_rate, tunnels) = valves.get(cur_valve).unwrap();
        let (elephant_valve_rate, elephant_tunnels) = valves.get(cur_elephant).unwrap();

        let next_pressure = pressure
            + open_valves
                .iter()
                .map(|v| valves.get(v).unwrap().0)
                .sum::<usize>();

        let _ = &queue_part.clear();

        if *valve_rate != 0 && !open_valves.contains(&cur_valve) {
            let mut next_open = open_valves.clone();
            next_open.push(cur_valve);
            queue_part.push((cur_valve, next_open));
        }

        for tunnel in tunnels.iter() {
            let next_open = open_valves.clone();
            queue_part.push((tunnel, next_open));
        }

        for (next_valve, next_open) in &queue_part {
            if *elephant_valve_rate != 0 && !open_valves.contains(&cur_elephant) {
                let mut next_open = next_open.clone();
                next_open.push(cur_elephant);
                queue.push_back((
                    next_valve,
                    cur_elephant,
                    next_pressure,
                    minutes_left - 1,
                    next_open,
                ));
            }

            for tunnel in elephant_tunnels.iter() {
                if tunnel == next_valve {
                    continue;
                }
                let next_open = next_open.clone();
                queue.push_back((
                    next_valve,
                    tunnel,
                    next_pressure,
                    minutes_left - 1,
                    next_open,
                ));
            }
        }
    }
    max_released
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn test_part_1_sample() {
        let valves = parse_valves(SAMPLE);

        assert_eq!(release_pressure(&valves), 1651);
    }

    #[test]
    fn test_part_2_sample() {
        let valves = parse_valves(SAMPLE);

        assert_eq!(release_pressure_with_elephant_joined(&valves), 1707);
    }
}
