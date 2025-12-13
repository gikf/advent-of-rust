use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq)]
struct Device<'a> {
    name: &'a str,
    out_connections: Vec<&'a str>,
}

const DAC: &str = "dac";
const FFT: &str = "fft";
const OUT: &str = "out";
const SVR: &str = "svr";
const YOU: &str = "you";

fn main() {
    let input = Path::new("2025/day11/src/input.txt");
    let contents = &fs::read_to_string(input).unwrap();
    let devices = parse_devices(contents);

    println!("Step 1");
    println!(
        "Unique paths from 'you' to 'out': {:?}",
        number_of_paths(&devices, YOU, OUT)
    );

    println!("Step 2");
    println!(
        "Unique paths from 'svr' to 'out', passing both 'dac' and 'fft': {:?}",
        number_of_paths_passing_through(&devices, SVR, OUT, &[DAC, FFT])
    );
}

fn number_of_paths(devices: &[Device], from: &str, to: &str) -> usize {
    let mut device_to_out = HashMap::new();
    for device in devices {
        device_to_out.insert(device.name, &device.out_connections);
    }

    count_paths(&device_to_out, from, to, &mut HashMap::new())
}

fn count_paths<'a>(
    device_to_out: &HashMap<&str, &Vec<&'a str>>,
    start: &'a str,
    end: &str,
    _calculated: &mut HashMap<&'a str, usize>,
) -> usize {
    if let Some(result) = _calculated.get(&start) {
        return *result;
    }
    let result = device_to_out
        .get(start)
        .unwrap()
        .iter()
        .map(|out_name| {
            if *out_name == end {
                1
            } else {
                count_paths(device_to_out, out_name, end, _calculated)
            }
        })
        .sum();
    _calculated.insert(start, result);
    result
}

fn number_of_paths_passing_through(
    devices: &[Device],
    start: &str,
    end: &str,
    need_to_pass: &[&str],
) -> usize {
    let device_to_out: HashMap<_, _> = devices
        .iter()
        .map(|device| (device.name, &device.out_connections))
        .collect();

    let (count, _) = count_passing_through(
        &device_to_out,
        start,
        end,
        need_to_pass,
        &mut HashMap::new(),
    );
    count
}

fn count_passing_through<'a>(
    device_to_out: &HashMap<&str, &Vec<&'a str>>,
    start: &'a str,
    end: &str,
    need_to_pass: &[&str],
    _calculated: &mut HashMap<&'a str, (usize, Vec<bool>)>,
) -> (usize, Vec<bool>) {
    if let Some(result) = _calculated.get(&start) {
        return result.clone();
    }
    let out_connections = device_to_out.get(&start).unwrap();
    let passes_fulfilled = need_to_pass
        .iter()
        .map(|visit_target| start == *visit_target)
        .collect();

    if out_connections.len() == 1 && out_connections[0] == end {
        return (1, passes_fulfilled);
    }

    let out_paths = out_connections.iter().map(|out_target| {
        count_passing_through(device_to_out, out_target, end, need_to_pass, _calculated)
    });
    let mut total_paths = 0;
    let mut throughs_completed_to_count = HashMap::new();
    let mut most_through = 0;
    let mut most_through_visit = None;

    for (paths_count, through_visits) in out_paths {
        total_paths += paths_count;
        let through_count = through_visits.iter().filter(|v| **v).count();
        if through_count > most_through {
            most_through = through_count;
            most_through_visit = Some(through_visits.clone());
        }

        throughs_completed_to_count
            .entry(through_visits)
            .and_modify(|paths| *paths += paths_count)
            .or_insert(paths_count);
    }

    if let Some(most_through_fo_far) = most_through_visit
        && let Some((passes_through_below, count)) =
            throughs_completed_to_count.remove_entry(&most_through_fo_far)
    {
        let new_throughs: Vec<_> = passes_through_below
            .iter()
            .zip(passes_fulfilled.iter())
            .map(|(a, b)| *a || *b)
            .collect();
        _calculated.insert(start, (count, new_throughs.clone()));

        return (count, new_throughs);
    }
    _calculated.insert(start, (total_paths, passes_fulfilled.clone()));
    (total_paths, passes_fulfilled)
}

fn parse_devices(input: &str) -> Vec<Device<'_>> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split_ascii_whitespace().collect();
            let name = parts.first().unwrap().trim_end_matches(':');
            let out_connections = parts.into_iter().skip(1).collect();
            Device {
                name,
                out_connections,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "aaa: you hhh\nyou: bbb ccc\nbbb: ddd eee\nccc: ddd eee fff\nddd: ggg\neee: out\nfff: out\nggg: out\nhhh: ccc fff iii\niii: out";
    const SAMPLE2: &str = "svr: aaa bbb\naaa: fft\nfft: ccc\nbbb: tty\ntty: ccc\nccc: ddd eee\nddd: hub\nhub: fff\neee: dac\ndac: fff\nfff: ggg hhh\nggg: out\nhhh: out";

    #[test]
    fn test_parse_devices() {
        let result = parse_devices(SAMPLE);

        assert_eq!(
            result[..4],
            [
                Device {
                    name: "aaa",
                    out_connections: Vec::from(["you", "hhh"]),
                },
                Device {
                    name: "you",
                    out_connections: Vec::from(["bbb", "ccc"]),
                },
                Device {
                    name: "bbb",
                    out_connections: Vec::from(["ddd", "eee"]),
                },
                Device {
                    name: "ccc",
                    out_connections: Vec::from(["ddd", "eee", "fff"]),
                },
            ]
        );
    }

    #[test]
    fn test_step_1() {
        let devices = parse_devices(SAMPLE);
        let result = number_of_paths(&devices, "you", OUT);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_step_2() {
        let devices = parse_devices(SAMPLE2);
        let result = number_of_paths_passing_through(&devices, SVR, OUT, &Vec::from([DAC, FFT]));
        assert_eq!(result, 2);
    }
}
