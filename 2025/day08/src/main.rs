use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

type JunctionBox = (isize, isize, isize);
type Circuit = HashSet<JunctionBox>;

fn main() {
    let input = Path::new("2025/day08/src/input.txt");
    let boxes = parse_boxes(&fs::read_to_string(input).unwrap());

    let distances = calculate_distances(&boxes);

    println!("Step 1");
    println!(
        "Product of sized of three top circuits after 1000 connections: {:?}",
        step_1_cirtuit_sizes(&boxes, &distances, Some(1000))
    );

    println!("Step 2");
    println!(
        "Multiplied coordinates of last connected boxes, to form single circuit: {:?}",
        step_2_last_connection_value(&boxes, &distances, None)
    );
}

fn parse_boxes(input: &str) -> Vec<JunctionBox> {
    input
        .lines()
        .map(|line| {
            let mut it = line.split(',').take(3);
            let x = it.next().unwrap().parse().unwrap();
            let y = it.next().unwrap().parse().unwrap();
            let z = it.next().unwrap().parse().unwrap();
            (x, y, z)
        })
        .collect()
}

fn distance_between(a: &JunctionBox, b: &JunctionBox) -> f64 {
    (((a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2)) as f64).sqrt()
}

fn calculate_distances(boxes: &[JunctionBox]) -> Vec<(f64, (&JunctionBox, &JunctionBox))> {
    let mut distances = Vec::new();

    for (index_a, a) in boxes.iter().take(boxes.len() - 1).enumerate() {
        for index_b in (0..boxes.len()).skip(index_a + 1) {
            let b = &boxes[index_b];
            distances.push((distance_between(a, b), (&boxes[index_a], &boxes[index_b])));
        }
    }

    let mut distance_to_boxes: HashMap<String, (&JunctionBox, &JunctionBox)> = HashMap::new();
    let mut only_distances = Vec::new();

    for (distance, boxes) in distances.iter() {
        only_distances.push(*distance);
        distance_to_boxes.insert(format!("{}", distance), *boxes);
    }

    only_distances.sort_by(f64::total_cmp);
    let mut distance_with_boxes = Vec::new();
    for distance in only_distances {
        let cur_boxes = distance_to_boxes.remove(&format!("{}", distance)).unwrap();
        distance_with_boxes.push((distance, cur_boxes));
    }

    distance_with_boxes
}

fn join_boxes<'a>(
    boxes: &'a [JunctionBox],
    distances: &'a [(f64, (&JunctionBox, &JunctionBox))],
    number_to_join: Option<usize>,
) -> (
    HashMap<usize, Circuit>,
    Option<(&'a JunctionBox, &'a JunctionBox)>,
) {
    let mut circuits: HashMap<usize, Circuit> = HashMap::new();
    let mut box_to_circuit: HashMap<JunctionBox, usize> = HashMap::new();

    for (index, one_box) in boxes.iter().enumerate() {
        let mut circuit = HashSet::new();
        circuit.insert(*one_box);
        circuits.insert(index, circuit);
        box_to_circuit.insert(*one_box, index);
    }

    let take = match number_to_join {
        Some(num) => num,
        None => distances.len(),
    };

    let mut last_connection = None;
    for (_, (a, b)) in distances.iter().take(take) {
        last_connection = Some((*a, *b));
        match (
            box_to_circuit.contains_key(a),
            box_to_circuit.contains_key(b),
        ) {
            (true, true) => {
                let circuit_a_no = box_to_circuit.remove(a).unwrap();
                let circuit_b_no = box_to_circuit.remove(b).unwrap();

                if circuit_a_no == circuit_b_no {
                    box_to_circuit.insert(**a, circuit_a_no);
                    box_to_circuit.insert(**b, circuit_a_no);
                    continue;
                }

                let mut circuit_b = circuits.remove(&circuit_b_no).unwrap();

                let circuit_a = circuits.get_mut(&circuit_a_no).unwrap();

                for cur_box in circuit_b.drain() {
                    box_to_circuit.insert(cur_box, circuit_a_no);
                    circuit_a.insert(cur_box);
                }
                box_to_circuit.insert(**a, circuit_a_no);
            }
            _ => unreachable!(),
        }
        if circuits.len() == 1 && box_to_circuit.len() == boxes.len() {
            break;
        }
    }
    (circuits, last_connection)
}

fn step_1_cirtuit_sizes(
    boxes: &[JunctionBox],
    distances: &[(f64, (&JunctionBox, &JunctionBox))],
    number_to_join: Option<usize>,
) -> usize {
    let (circuits, _) = join_boxes(boxes, distances, number_to_join);
    let mut sizes = circuits.values().map(|v| v.len()).collect::<Vec<usize>>();
    sizes.sort();
    sizes.iter().rev().take(3).product()
}

fn step_2_last_connection_value(
    boxes: &[JunctionBox],
    distances: &[(f64, (&JunctionBox, &JunctionBox))],
    number_to_join: Option<usize>,
) -> isize {
    let (_, last_connection) = join_boxes(boxes, distances, number_to_join);

    match last_connection {
        Some(((ax, _, _), (bx, _, _))) => ax * bx,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "162,817,812\n57,618,57\n906,360,560\n592,479,940\n352,342,300\n466,668,158\n542,29,236\n431,825,988\n739,650,466\n52,470,668\n216,146,977\n819,987,18\n117,168,530\n805,96,715\n346,949,466\n970,615,88\n941,993,340\n862,61,35\n984,92,344\n425,690,689";

    #[test]
    fn test_parse_boxes() {
        let input = "162,817,812\n57,618,57\n906,360,560\n592,479,940";
        let result = parse_boxes(input);

        assert_eq!(
            result,
            [
                (162, 817, 812),
                (57, 618, 57),
                (906, 360, 560),
                (592, 479, 940),
            ]
        )
    }

    #[test]
    fn test_step_1_sample() {
        let boxes = parse_boxes(SAMPLE);

        let distances = calculate_distances(&boxes);

        let result = step_1_cirtuit_sizes(&boxes, &distances, Some(10));
        assert_eq!(result, 40);
    }

    #[test]
    fn test_step_2_sample() {
        let boxes = parse_boxes(SAMPLE);
        let distances = calculate_distances(&boxes);

        let result = step_2_last_connection_value(&boxes, &distances, None);
        assert_eq!(result, 25272);
    }
}
