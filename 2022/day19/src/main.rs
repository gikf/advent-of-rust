use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn main() {
    let input = Path::new("2022/day19/src/input.txt");
    let blueprints = parse_blueprints(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Sum of quality levels: {:?}",
        sum_quality_levels(&blueprints)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Multiplied most opened geodes in 32 minutes from three first blueprints: {:?}",
        geodes_part2(&blueprints)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_blueprints(input: &str) -> Vec<[[usize; 3]; 4]> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<Vec<&str>> = line
                .split('.')
                .map(|part| part.split_ascii_whitespace().collect())
                .collect();
            [
                [parts[0][6].parse().unwrap(), 0, 0],
                [parts[1][4].parse().unwrap(), 0, 0],
                [
                    parts[2][4].parse().unwrap(),
                    parts[2][7].parse().unwrap(),
                    0,
                ],
                [
                    parts[3][4].parse().unwrap(),
                    0,
                    parts[3][7].parse().unwrap(),
                ],
            ]
        })
        .collect()
}

type Robots = [usize; 4];
type Resources = [usize; 4];
type PrevSkippedProduction = Option<Vec<usize>>;

fn most_geode_cracked(blueprint: &[[usize; 3]; 4], minutes: usize) -> usize {
    let mut queue: Vec<(Robots, Resources, PrevSkippedProduction, usize)> =
        Vec::with_capacity(100_000);
    queue.push(([1, 0, 0, 0], [0, 0, 0, 0], None, 1));

    let mut most_geodes = 0;
    let mut minute_to_geode = vec![0_usize; minutes + 2];
    let mut rr_to_minute = HashMap::with_capacity(500_000);

    while let Some((robots, resources, skipped_production, minutes_so_far)) = queue.pop() {
        if rr_to_minute
            .get(&(robots, resources))
            .is_some_and(|minute| *minute <= minutes_so_far)
        {
            continue;
        }

        rr_to_minute.insert((robots, resources), minutes_so_far);

        if minutes_so_far == minutes + 1 {
            if most_geodes < resources[3] {
                most_geodes = resources[3];
            }
            continue;
        } else if minutes_so_far > minutes {
            continue;
        }

        let mut next_resources = resources;
        let next_robots = robots;
        let mut next_minutes = minutes_so_far + 1;

        let mut buildable_robots: Vec<_> = robots_to_build(blueprint, &next_resources);

        next_resources = add_resources(&next_resources, &robots);
        while buildable_robots.is_empty() && next_minutes < minutes {
            buildable_robots = robots_to_build(blueprint, &next_resources);
            next_resources = add_resources(&next_resources, &robots);
            next_minutes += 1;
        }

        if minute_to_geode[next_minutes] > 0
            && minute_to_geode[next_minutes].abs_diff(next_resources[3]) > 2
        {
            continue;
        } else if minute_to_geode[next_minutes] < next_resources[3] {
            minute_to_geode[next_minutes] = next_resources[3];
        }

        let can_buy_goede = buildable_robots.contains(&3);
        if can_buy_goede {
            buildable_robots = vec![3];
        }

        for robot_id in buildable_robots.iter() {
            if skipped_production
                .as_ref()
                .is_some_and(|prev_possible| prev_possible.contains(robot_id))
            {
                continue;
            }
            let cost = blueprint[*robot_id];
            let robots_of_type = next_robots[*robot_id];
            let produces_max_of_type = *robot_id != 3
                && blueprint
                    .iter()
                    .all(|costs| costs[*robot_id] <= robots_of_type);
            if produces_max_of_type {
                continue;
            }
            let mut n_robots = next_robots;
            let mut n_resources = next_resources;
            n_resources[0] -= cost[0];
            n_resources[1] -= cost[1];
            n_resources[2] -= cost[2];
            n_robots[*robot_id] += 1;

            let remaining_time = (minutes + 1) - next_minutes;
            if remaining_time > 1 {
                let max_possible_geodes = n_robots[3] * remaining_time
                    + next_resources[3]
                    + remaining_time * (remaining_time - 1) / 2;
                if max_possible_geodes <= most_geodes {
                    continue;
                }
            }
            queue.push((n_robots, n_resources, None, next_minutes));
        }

        if buildable_robots.len() != 4 && !can_buy_goede {
            queue.push((
                next_robots,
                next_resources,
                Some(buildable_robots),
                next_minutes,
            ));
        }
    }
    most_geodes
}

fn add_resources(resources: &Resources, robots: &Robots) -> Resources {
    [
        resources[0] + robots[0],
        resources[1] + robots[1],
        resources[2] + robots[2],
        resources[3] + robots[3],
    ]
}

fn robots_to_build(blueprint: &[[usize; 3]; 4], resources: &Resources) -> Vec<usize> {
    blueprint
        .iter()
        .enumerate()
        .filter_map(|(robot_id, cost)| {
            if cost.iter().zip(resources.iter()).all(|(c, r)| c <= r) {
                Some(robot_id)
            } else {
                None
            }
        })
        .collect()
}

fn sum_quality_levels(blueprints: &[[[usize; 3]; 4]]) -> usize {
    (1_usize..)
        .zip(blueprints.iter())
        .map(|(id, blueprint)| id * most_geode_cracked(blueprint, 24))
        .sum()
}

fn geodes_part2(blueprints: &[[[usize; 3]; 4]]) -> usize {
    blueprints[..3]
        .iter()
        .map(|blueprint| most_geode_cracked(blueprint, 32))
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn test_most_geode_cracked_24() {
        let blueprints = parse_blueprints(SAMPLE);

        assert_eq!(most_geode_cracked(&blueprints[0], 24), 9);
        assert_eq!(most_geode_cracked(&blueprints[1], 24), 12);
    }

    #[test]
    fn test_most_geode_cracked_32() {
        let blueprints = parse_blueprints(SAMPLE);

        assert_eq!(most_geode_cracked(&blueprints[0], 32), 56);
        assert_eq!(most_geode_cracked(&blueprints[1], 32), 62);
    }

    #[test]
    fn test_part_1_sample() {
        let blueprints = parse_blueprints(SAMPLE);

        assert_eq!(sum_quality_levels(&blueprints), 33);
    }
}
