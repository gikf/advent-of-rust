use std::path::Path;
use std::fs;

const PART1_Y: isize = 2_000_000;
const PART2_MIN: isize = 0;
const PART2_MAX: isize = 4_000_000;

type Coordinates = (isize, isize);
type SensorWithBeacon = (Coordinates, Coordinates);

fn main() {
    let input = Path::new("2022/day15/src/input.txt");
    let sensors = parse_sensors(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Number of positions in row y=2000000, where cannot be beacon: {:?}",
        positions_not_beacon_at(&sensors, PART1_Y)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Tuning frequency: {:?}",
        tuning_frequency(&sensors, PART2_MIN, PART2_MAX)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_sensors(input: &str) -> Vec<SensorWithBeacon> {
    input
        .lines()
        .map(|line| {
            let (sensor, beacon) = line.split_once(":").unwrap();
            let (beacon_rest, beacon_y) = beacon.rsplit_once(", y=").unwrap();
            let (_, beacon_x) = beacon_rest.split_once("x=").unwrap();

            let beacon: (isize, isize) = (beacon_y.parse().unwrap(), beacon_x.parse().unwrap());

            let (sensor_rest, sensor_y) = sensor.rsplit_once(", y=").unwrap();

            let (_, sensor_x) = sensor_rest.split_once("x=").unwrap();

            (
                (sensor_y.parse().unwrap(), sensor_x.parse().unwrap()),
                beacon,
            )
        })
        .collect()
}

fn positions_not_beacon_at(sensors: &[SensorWithBeacon], target_y: isize) -> usize {
    let mut not_beacon = Vec::new();
    for (sensor, closest_beacon) in sensors {
        let distance =
            (sensor.0.abs_diff(closest_beacon.0) + sensor.1.abs_diff(closest_beacon.1)) as isize;

        let row_boundary_low = sensor.0 - distance;
        let row_boundary_high = sensor.0 + distance;
        let row = sensor.0;
        let col = sensor.1;

        if target_y >= row_boundary_low && target_y <= row_boundary_high {
            let offset = if row > target_y {
                target_y - row_boundary_low
            } else {
                row_boundary_high - target_y
            };
            let mut range_start = col - offset;
            let mut range_end = col + offset;

            let mut joined = Vec::new();
            for (index, (start, end)) in not_beacon.iter().enumerate() {
                if !(range_end + 1 < *start || range_start - 1 > *end) {
                    joined.push(index);
                }
            }

            if joined.is_empty() {
                not_beacon.push((range_start, range_end));
            } else if joined.len() == 1 {
                let (start, end) = not_beacon.get_mut(joined.pop().unwrap()).unwrap();
                *start = range_start.min(*start);
                *end = range_end.max(*end);
            } else {
                let mut next_ranges = Vec::new();

                while let Some((start, end)) = not_beacon.pop() {
                    if (range_end + 1) < start || range_start - 1 > end {
                        next_ranges.push((start, end));
                    } else {
                        range_start = range_start.min(start);
                        range_end = range_end.max(end);
                    }
                }
                next_ranges.push((range_start, range_end));
                not_beacon = next_ranges;
            }
        }
    }

    not_beacon
        .into_iter()
        .map(|(start, end)| (end - start) as usize)
        .sum()
}

fn tuning_frequency(
    sensors: &[SensorWithBeacon],
    min: isize,
    max: isize,
) -> isize {
    let mut non_beacon_ranges: Vec<Vec<(isize, isize)>> =
        vec![vec![(min, max)]; (max as usize) + 1];

    let t1 = std::time::Instant::now();

    for (sensor, beacon) in sensors {
        let distance_to_beacon = sensor.0.abs_diff(beacon.0) + sensor.1.abs_diff(beacon.1);

        let row = sensor.0 as usize;
        let col = sensor.1;

        let row_boundary_low = sensor.0 - (distance_to_beacon as isize);
        let row_boundary_high = sensor.0 + (distance_to_beacon as isize);

        let row_min = row_boundary_low.max(min) as usize;
        let row_max = row_boundary_high.min(max) as usize;
        let mut to_add = Vec::new();
        let mut to_remove = Vec::new();

        for row_no in (row_min..row).chain((row..=row_max).rev()) {
            if non_beacon_ranges[row_no].is_empty() {
                continue;
            }
            let offset = if row_no <= row {
                (row_no as isize) - row_boundary_low
            } else {
                row_boundary_high - (row_no as isize)
            };

            let not_beacon_start = (col - offset).max(min);
            let not_beacon_end = (col + offset).min(max);

            to_add.clear();
            to_remove.clear();

            for (maybe_beacon_start, maybe_beacon_end) in
                non_beacon_ranges[row_no].iter_mut()
            {
                let nb_start_le_mb_start = not_beacon_start <= *maybe_beacon_start;
                let nb_start_g_mb_start = not_beacon_start > *maybe_beacon_start;
                let nb_start_l_mb_end = not_beacon_start < *maybe_beacon_end;

                let nb_end_ge_mb_end = not_beacon_end >= *maybe_beacon_end;
                let nb_end_l_mb_end = not_beacon_end < *maybe_beacon_end;
                let nb_end_g_mb_start = not_beacon_end > *maybe_beacon_start;

                let contains_maybe_beacon_range = nb_start_le_mb_start && nb_end_ge_mb_end;
                let contained_by_maybe_beacon_range = nb_start_g_mb_start && nb_end_l_mb_end;

                if contained_by_maybe_beacon_range {
                    to_add.push((not_beacon_end + 1, *maybe_beacon_end));
                    *maybe_beacon_end = not_beacon_start - 1;
                } else if contains_maybe_beacon_range {
                    to_remove.push((*maybe_beacon_start, *maybe_beacon_end));
                } else if nb_start_le_mb_start && nb_end_g_mb_start && nb_end_l_mb_end {
                    *maybe_beacon_start = not_beacon_end + 1;
                } else if nb_start_g_mb_start && nb_start_l_mb_end && nb_end_ge_mb_end {
                    *maybe_beacon_end = not_beacon_start - 1;
                }
            }
            if !to_add.is_empty() {
                for range in &to_add {
                    non_beacon_ranges[row_no].push(*range);
                }
            }
            if !to_remove.is_empty() {
                non_beacon_ranges[row_no]
                    .retain(|(start, end)|! to_remove.contains(&(*start, *end)));
            }
        }
    }

    println!("Ranges added {:?}", t1.elapsed());

    let (row, cols) = non_beacon_ranges
        .iter()
        .enumerate()
        .find_map(|(row_no, ranges)| {
            if ranges.len() == 1 {
                Some((row_no, ranges))
            } else {
                None
            }
        }).unwrap();
    cols[0].0 * 4_000_000 + (row as isize)
}
#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_part_1_sample() {
        let sensors = parse_sensors(SAMPLE);

        assert_eq!(positions_not_beacon_at(&sensors, 10), 26);
    }

    #[test]
    fn test_part_2_sample() {
        let sensors = parse_sensors(SAMPLE);
        let min = 0;
        let max = 20;

        assert_eq!(tuning_frequency(&sensors, min, max), 56_000_011);
    }
}
