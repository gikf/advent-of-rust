use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

fn main() {
    let input = Path::new("2023/day25/src/input.txt");
    let contents = fs::read_to_string(input).unwrap();
    let connections = parse_connections(&contents);

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Sizes of two separate groups multiplied: {:?}",
        separate_groups(&connections)
    );
    println!("In {:?}", part1.elapsed());
}

fn parse_connections(input: &str) -> HashMap<u16, HashSet<u16>> {
    let mut node_id = 0_u16;
    let mut node_to_id = HashMap::new();
    let mut connections: HashMap<u16, HashSet<u16>> = HashMap::new();
    input.lines().for_each(|line| {
        let (source, targets) = line.split_once(": ").unwrap();
        let source_id = match node_to_id.get(source) {
            Some(&node_id) => node_id,
            None => {
                node_to_id.insert(source, node_id);
                node_id += 1;
                node_id - 1
            }
        };
        for target in targets.split_ascii_whitespace() {
            let target_id = match node_to_id.get(target) {
                Some(&node_id) => node_id,
                None => {
                    node_to_id.insert(target, node_id);
                    node_id += 1;
                    node_id - 1
                }
            };
            connections
                .entry(source_id)
                .and_modify(|t| {
                    t.insert(target_id);
                })
                .or_insert(HashSet::from([target_id]));
            connections
                .entry(target_id)
                .and_modify(|t| {
                    t.insert(source_id);
                })
                .or_insert(HashSet::from([source_id]));
        }
    });
    connections
}

fn separate_groups(connections: &HashMap<u16, HashSet<u16>>) -> usize {
    // Based on https://old.reddit.com/r/adventofcode/comments/18qbsxs/2023_day_25_solutions/ketzp94/

    let mut group = HashSet::new();
    for key in connections.keys() {
        group.insert(*key);
    }

    let count = |k: u16, set: &HashSet<u16>| {
        connections
            .get(&k)
            .unwrap()
            .iter()
            .filter(|n| !set.contains(*n))
            .count()
    };

    while group.iter().map(|v| count(*v, &group)).sum::<usize>() != 3 {
        if let Some(max) = group.iter().max_by_key(|v| count(**v, &group)) {
            group.remove(&(max.clone()));
        }
    }

    let mut not_in_group = HashSet::new();
    for key in connections.keys() {
        if !group.contains(key) {
            not_in_group.insert(*key);
        }
    }

    group.len() * not_in_group.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    #[test]
    fn test_part_1_sample() {
        let connections = parse_connections(SAMPLE);

        separate_groups(&connections);
    }

    #[test]
    fn test_part_2_sample() {}
}
