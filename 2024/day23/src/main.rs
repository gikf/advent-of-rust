use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

fn main() {
    let input = Path::new("2024/day23/src/input.txt");
    let content = fs::read_to_string(input).unwrap();
    let connections = parse_connections(&content);

    let part1 = std::time::Instant::now();

    let computers = map_computers(&connections);
    let inter_connections = find_inter_connections(&computers);
    println!("Part 1");
    println!(
        "Inter-connections with computer staring with \"t\": {:?}",
        inter_connections_starting_with(&inter_connections, "t")
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Password to get into party: {:?}",
        find_most_connections(&computers, &inter_connections)
    );
    println!("In {:?}", part2.elapsed());
    let part3 = std::time::Instant::now();
    println!("Alt version");
    println!(
        "Password to get into party: {:?}",
        find_largest_party(&computers)
    );
    println!("In {:?}", part3.elapsed());
}

fn parse_connections(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .collect()
}

fn map_computers<'a>(connections: &'a [(&str, &str)]) -> HashMap<&'a str, HashSet<&'a str>> {
    let mut computers = HashMap::new();

    for (a, b) in connections.iter() {
        computers
            .entry(*a)
            .and_modify(|connected_to: &mut HashSet<&str>| {
                connected_to.insert(*b);
            })
            .or_insert(HashSet::from([*b]));
        computers
            .entry(*b)
            .and_modify(|connected_to: &mut HashSet<&str>| {
                connected_to.insert(*a);
            })
            .or_insert(HashSet::from([*a]));
    }
    computers
}

fn find_inter_connections<'a>(
    computers: &'a HashMap<&str, HashSet<&str>>,
) -> HashSet<Vec<&'a str>> {
    let mut inter_connections: HashSet<Vec<&str>> = HashSet::new();
    for (computer, connected_to) in computers {
        for other_computer in connected_to.iter() {
            let other_connections = computers.get(*other_computer).unwrap();
            let both_connected: HashSet<_> = connected_to.intersection(other_connections).collect();

            for third_connection in both_connected.iter() {
                if *third_connection != computer && *third_connection != other_computer {
                    let mut inter_connection = vec![*computer, *other_computer, **third_connection];
                    inter_connection.sort();
                    inter_connections.insert(inter_connection);
                }
            }
        }
    }

    inter_connections
}

fn find_largest_party(computers: &HashMap<&str, HashSet<&str>>) -> String {
    let mut reduced_computers = HashMap::new();
    let mut computer_id_to_computer = HashMap::new();
    let mut computer_to_computer_id = HashMap::new();
    let mut id = 0_i16;
    for (computer, connected_to) in computers {
        let mut connections = HashSet::new();

        let computer_id = match computer_to_computer_id.get(computer) {
            Some(c_id) => *c_id,
            None => {
                computer_id_to_computer.insert(id, *computer);
                computer_to_computer_id.insert(*computer, id);
                id += 1;
                id - 1
            }
        };

        for name in connected_to {
            if let Some(other_id) = computer_to_computer_id.get(name) {
                connections.insert(*other_id);
            } else {
                computer_id_to_computer.insert(id, *name);
                computer_to_computer_id.insert(*name, id);
                connections.insert(id);
                id += 1;
            }
        }
        reduced_computers.insert(computer_id, connections);
    }

    let mut parties: HashSet<Vec<i16>> =
        HashSet::from_iter(reduced_computers.keys().map(|computer| vec![*computer]));

    for (computer_id, connected_to) in reduced_computers.iter() {
        let mut new_parties = HashSet::new();

        for party in parties.drain() {
            let mut party = party;
            if party.contains(computer_id) {
                if !party
                    .iter()
                    .all(|other_id| other_id == computer_id || connected_to.contains(other_id))
                {
                    let mut new_party = vec![*computer_id];
                    for other_id in &party {
                        if connected_to.contains(other_id) {
                            new_party.push(*other_id);
                        }
                    }
                    new_party.sort();
                    new_parties.insert(new_party);
                }
                new_parties.insert(party);
            } else {
                let not_connected_to_current: Vec<_> = party
                    .iter()
                    .filter(|other_id| !connected_to.contains(*other_id))
                    .collect();
                if not_connected_to_current.is_empty() {
                    party.push(*computer_id);
                    party.sort();
                }
                new_parties.insert(party);
            }
        }
        parties = new_parties;
    }

    let mut mapped: Vec<_> = parties
        .iter()
        .map(|party| {
            let mut members: Vec<_> = party
                .iter()
                .map(|id| computer_id_to_computer.get(id).unwrap().to_string())
                .collect();
            members.sort();
            members
        })
        .collect();
    mapped.sort_by_key(|party| party.len());
    mapped.last().unwrap().join(",")
}

fn find_most_connections(
    computers: &HashMap<&str, HashSet<&str>>,
    inter_connections: &HashSet<Vec<&str>>,
) -> String {
    let mut computer_to_connections_count = HashMap::new();

    for inter_connection in inter_connections {
        for computer in inter_connection {
            computer_to_connections_count
                .entry(*computer)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }

    let max_value = computer_to_connections_count.values().max().unwrap();
    let most_connections: Vec<_> = computer_to_connections_count
        .keys()
        .filter(|name| computer_to_connections_count.get(*name).unwrap() == max_value)
        .collect();

    let mut large_parties: HashSet<Vec<_>> = HashSet::new();
    for (index, a) in most_connections.iter().enumerate() {
        let connected_to = computers.get(*a).unwrap();
        let connected: Vec<_> = most_connections
            .iter()
            .skip(index)
            .filter(|b| connected_to.contains(**b))
            .collect();
        if connected.iter().all(|b| {
            let connected_b = computers.get(**b).unwrap();
            connected
                .iter()
                .all(|c| b == c || connected_b.contains(**c))
        }) {
            let mut p: Vec<String> = connected.iter().map(|v| v.to_string()).collect();
            p.push(a.to_string());
            p.sort();
            large_parties.insert(p);
        }
    }

    let mut most_connected: Vec<_> = large_parties.into_iter().collect();
    most_connected.sort_by_key(|p| p.len());
    most_connected.last().unwrap().join(",")
}

fn inter_connections_starting_with<'a>(
    inter_connections: &'a HashSet<Vec<&str>>,
    starting: &'a str,
) -> usize {
    inter_connections
        .iter()
        .filter(|inter_connection| {
            inter_connection
                .iter()
                .any(|name| name.starts_with(starting))
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "kh-tc\nqp-kh\nde-cg\nka-co\nyn-aq\nqp-ub\ncg-tb\nvc-aq\ntb-ka\nwh-tc
yn-cg\nkh-ub\nta-co\nde-co\ntc-td\ntb-wq\nwh-td\nta-ka\ntd-qp\naq-cg\nwq-ub
ub-vc\nde-ta\nwq-aq\nwq-vc\nwh-yn\nka-de\nkh-ta\nco-tc\nwh-qp\ntb-vc\ntd-yn";

    #[test]
    fn test_parse_connections() {
        let connections = parse_connections(SAMPLE);

        assert_eq!(connections[0], ("kh", "tc"));
        assert_eq!(connections[connections.len() - 1], ("td", "yn"));
        assert_eq!(connections.len(), 32);
    }

    #[test]
    fn test_inter_connections() {
        let connections = parse_connections(SAMPLE);
        let computers = map_computers(&connections);
        let inter_connections = find_inter_connections(&computers);

        assert_eq!(inter_connections.len(), 12);
    }

    #[test]
    fn test_inter_connections_starting_with() {
        let connections = parse_connections(SAMPLE);
        let computers = map_computers(&connections);
        let inter_connections = find_inter_connections(&computers);

        assert_eq!(inter_connections_starting_with(&inter_connections, "t"), 7);
    }

    #[test]
    fn test_part_2_sample() {
        let connections = parse_connections(SAMPLE);
        let computers = map_computers(&connections);
        let inter_connections = find_inter_connections(&computers);

        assert_eq!(find_largest_party(&computers), "co,de,ka,ta");
        assert_eq!(
            find_most_connections(&computers, &inter_connections),
            "co,de,ka,ta"
        );
    }
}
