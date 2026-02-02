use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::path::Path;

fn main() {
    let input = Path::new("2021/day12/src/input.txt");
    let contents = &fs::read_to_string(input).unwrap();
    let caves = parse_caves(contents);

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Unique paths visiting small caves at most once: {:?}",
        paths(&caves)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Unique paths visiting single small cave twice and other small caves at most once: {:?}",
        paths2(&caves)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_caves(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut from_to = HashMap::new();
    input.lines().for_each(|line| {
        let (from, to) = line.split_once("-").unwrap();
        from_to
            .entry(from)
            .and_modify(|tos: &mut Vec<&str>| {
                tos.push(to);
            })
            .or_insert(vec![to]);
        from_to
            .entry(to)
            .and_modify(|tos: &mut Vec<&str>| {
                tos.push(from);
            })
            .or_insert(vec![from]);
    });

    from_to
}

fn paths(caves: &HashMap<&str, Vec<&str>>) -> usize {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_front(("start", vec!["start"]));
    let mut paths = HashSet::new();

    while let Some((cur_node, path_so_far)) = queue.pop_front() {
        if !seen.insert(path_so_far.clone()) {
            continue;
        } else if cur_node == "end" {
            paths.insert(path_so_far);
            continue;
        } else if let Some(connections) = caves.get(cur_node) {
            for next_node in connections {
                if !next_node.chars().all(|c| c.is_lowercase()) || !path_so_far.contains(next_node)
                {
                    let mut new_path = path_so_far.clone();
                    new_path.push(next_node);
                    queue.push_back((next_node, new_path));
                }
            }
        }
    }
    paths.len()
}

fn paths2(caves: &HashMap<&str, Vec<&str>>) -> usize {
    let mut cur_id = 0_u8;
    let mut cave_name_to_id = HashMap::new();
    let mut lowercase_nodes = Vec::new();
    let mut caves_ids: HashMap<u8, Vec<u8>> = HashMap::new();

    for (cave_name, connections) in caves {
        let cave_id = if let Some(id) = cave_name_to_id.get(cave_name) {
            *id
        } else {
            let id = cur_id;
            cave_name_to_id.insert(*cave_name, id);
            if cave_name.chars().all(|c| c.is_lowercase()) {
                lowercase_nodes.push(id);
            }
            cur_id += 1;
            id
        };

        caves_ids.insert(
            cave_id,
            connections
                .iter()
                .map(|connection| {
                    if let Some(id) = cave_name_to_id.get(*connection) {
                        *id
                    } else {
                        let id = cur_id;
                        cave_name_to_id.insert(*connection, id);
                        if connection.chars().all(|c| c.is_lowercase()) {
                            lowercase_nodes.push(id);
                        }
                        cur_id += 1;
                        id
                    }
                })
                .collect(),
        );
    }

    let start_id = cave_name_to_id.get("start").unwrap();
    let end_id = cave_name_to_id.get("end").unwrap();

    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_front((start_id, vec![start_id], false));
    let mut path_count = 0;

    while let Some((cur_node, path_so_far, double_visited)) = queue.pop_front() {
        if !seen.insert(path_so_far.clone()) {
            continue;
        } else if cur_node == end_id {
            path_count += 1;
            continue;
        } else if let Some(connections) = caves_ids.get(cur_node) {
            for next_node in connections {
                if next_node == start_id {
                    continue;
                }
                let already_visited = path_so_far.contains(&next_node);
                let is_node_lowercase = lowercase_nodes.contains(next_node);
                if !is_node_lowercase || !already_visited || !double_visited {
                    let next_double = if is_node_lowercase && already_visited {
                        true
                    } else {
                        double_visited
                    };
                    let mut new_path = path_so_far.clone();
                    new_path.push(next_node);
                    queue.push_back((next_node, new_path, next_double));
                }
            }
        }
    }
    path_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = "start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end";
    const SAMPLE2: &str = "dc-end\nHN-start\nstart-kj\ndc-start
dc-HN\nLN-dc\nHN-end\nkj-sa\nkj-HN\nkj-dc";
    const SAMPLE3: &str = "fs-end\nhe-DX\nfs-he\nstart-DX\npj-DX
end-zg\nzg-sl\nzg-pj\npj-he\nRW-he\nfs-DX\npj-RW\nzg-RW
start-pj\nhe-WI\nzg-he\npj-fs\nstart-RW";

    #[test]
    fn test_part_1_sample() {
        let connections1 = parse_caves(SAMPLE1);
        let connections2 = parse_caves(SAMPLE2);
        let connections3 = parse_caves(SAMPLE3);

        assert_eq!(paths(&connections1), 10);
        assert_eq!(paths(&connections2), 19);
        assert_eq!(paths(&connections3), 226);
    }

    #[test]
    fn test_part_2_sample() {
        let connections1 = parse_caves(SAMPLE1);
        let connections2 = parse_caves(SAMPLE2);
        let connections3 = parse_caves(SAMPLE3);

        assert_eq!(paths2(&connections1), 36);
        assert_eq!(paths2(&connections2), 103);
        assert_eq!(paths2(&connections3), 3509);
    }
}
