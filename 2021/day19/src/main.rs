use std::collections::HashSet;
use std::fs;
use std::path::Path;

type Coordinates = [isize; 3];

fn main() {
    let input = Path::new("2021/day19/src/input.txt");
    let scanners = parse_scanners(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    let (all_beacons, scanners_coords) = align_scanners(&scanners);
    println!("Part 1");
    println!("Number of beacons: {:?}", beacon_count(&all_beacons));
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Largest distance between scanners: {:?}",
        largest_distance_between_scanners(&scanners_coords)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_scanners(input: &str) -> Vec<Vec<Coordinates>> {
    let mut scanners = Vec::new();
    let mut scanner = Vec::new();
    for line in input.lines().chain([""]) {
        if line.starts_with("--") {
            continue;
        } else if line.is_empty() {
            scanners.push(scanner);
            scanner = Vec::new();
        } else {
            let mut nums = line.split(",");
            scanner.push([
                nums.next().unwrap().parse().unwrap(),
                nums.next().unwrap().parse().unwrap(),
                nums.next().unwrap().parse().unwrap(),
            ]);
        }
    }
    scanners
}

fn beacon_relative_distances(beacons: &HashSet<Coordinates>) -> Vec<(HashSet<usize>, Coordinates)> {
    let mut all_relatives = Vec::new();
    for beacon1 in beacons.iter() {
        let mut beacons_to_beacon = HashSet::new();
        for other_beacon in beacons.iter() {
            beacons_to_beacon.insert(distance(*beacon1, *other_beacon));
        }
        all_relatives.push((beacons_to_beacon, *beacon1));
    }
    all_relatives
}

fn align_scanners(scanners: &[Vec<Coordinates>]) -> (HashSet<Coordinates>, Vec<Coordinates>) {
    let mut relatives = Vec::new();

    for scanner in scanners {
        let mut scanner_relatives = Vec::new();
        for beacon in scanner.iter() {
            let mut beacons_to_beacon = HashSet::new();
            for other_beacon in scanner.iter() {
                beacons_to_beacon.insert(distance(*beacon, *other_beacon));
            }
            scanner_relatives.push(beacons_to_beacon);
        }
        relatives.push(scanner_relatives);
    }

    let mut all_beacons = HashSet::new();
    for beacon in &scanners[0] {
        all_beacons.insert(*beacon);
    }

    let mut scanners_coords = vec![[0, 0, 0]; scanners.len()];
    let mut aligned_scanners = vec![0];

    while aligned_scanners.len() != scanners.len() {
        for (scanner_no, scanner) in relatives.iter().enumerate() {
            if aligned_scanners.contains(&scanner_no) {
                continue;
            }
            let all_relatives = beacon_relative_distances(&all_beacons);

            'outer: for (beacons1, ref_beacon) in all_relatives.iter() {
                let reference_distances: Vec<_> = all_beacons
                    .iter()
                    .map(|beacon| relative_distances(*ref_beacon, *beacon))
                    .collect();
                for (beacon2_no, beacons2) in scanner.iter().enumerate() {
                    let matching_beacons = beacons1.intersection(beacons2).count();
                    let scanner1_beacon = *ref_beacon;
                    let scanner2_beacon = scanners[scanner_no][beacon2_no];

                    if matching_beacons >= 12 {
                        let other_distances: Vec<_> = scanners[scanner_no]
                            .iter()
                            .map(|beacon| {
                                relative_distances(scanners[scanner_no][beacon2_no], *beacon)
                            })
                            .collect();
                        for distances in &reference_distances {
                            for others in &other_distances {
                                if distances.iter().any(|n| *n != 0)
                                    && distances.iter().all(|num| {
                                        others.contains(num) || others.contains(&(-*num))
                                    })
                                {
                                    let [ref_a, ref_b, ref_c] = *distances;
                                    let [a, b, c] = *others;
                                    let ref_abs = (ref_a.abs(), ref_b.abs(), ref_c.abs());
                                    let abs = (a.abs(), b.abs(), c.abs());

                                    let mut offsets = [1, 1, 1];

                                    let coord_rotation = if ref_abs == (abs.0, abs.1, abs.2) {
                                        if *distances == [a, b, c] {
                                        } else if *distances == [a, -b, -c] {
                                            offsets[1] = -1;
                                            offsets[2] = -1;
                                        } else if *distances == [-a, b, -c] {
                                            offsets[0] = -1;
                                            offsets[2] = -1;
                                        } else if *distances == [-a, -b, c] {
                                            offsets[0] = -1;
                                            offsets[1] = -1;
                                        }
                                        [0, 1, 2]
                                    } else if ref_abs == (abs.0, abs.2, abs.1) {
                                        if *distances == [a, c, -b] {
                                            offsets[2] = -1;
                                        } else if *distances == [a, -c, b] {
                                            offsets[1] = -1;
                                        } else if *distances == [-a, c, b] {
                                            offsets[0] = -1;
                                        } else if *distances == [-a, -c, -b] {
                                            offsets[0] = -1;
                                            offsets[1] = -1;
                                            offsets[2] = -1;
                                        }
                                        [0, 2, 1]
                                    } else if ref_abs == (abs.1, abs.0, abs.2) {
                                        if *distances == [b, -a, c] {
                                            offsets[1] = -1;
                                        } else if *distances == [b, a, -c] {
                                            offsets[2] = -1;
                                        } else if *distances == [-b, a, c] {
                                            offsets[0] = -1;
                                        } else if *distances == [-b, -a, -c] {
                                            offsets[0] = -1;
                                            offsets[1] = -1;
                                            offsets[2] = -1;
                                        }
                                        [1, 0, 2]
                                    } else if ref_abs == (abs.1, abs.2, abs.0) {
                                        if *distances == [b, -c, -a] {
                                            offsets[1] = -1;
                                            offsets[2] = -1;
                                        } else if *distances == [-b, c, -a] {
                                            offsets[0] = -1;
                                            offsets[2] = -1;
                                        } else if *distances == [-b, -c, a] {
                                            offsets[0] = -1;
                                            offsets[1] = -1;
                                        }
                                        [1, 2, 0]
                                    } else if ref_abs == (abs.2, abs.1, abs.0) {
                                        if *distances == [c, b, -a] {
                                            offsets[2] = -1;
                                        } else if *distances == [c, -b, a] {
                                            offsets[1] = -1;
                                        } else if *distances == [-c, b, a] {
                                            offsets[0] = -1;
                                        } else if *distances == [-c, -b, -a] {
                                            offsets[0] = -1;
                                            offsets[1] = -1;
                                            offsets[2] = -1;
                                        }
                                        [2, 1, 0]
                                    } else if ref_abs == (abs.2, abs.0, abs.1) {
                                        if *distances == [c, -a, -b] {
                                            offsets[1] = -1;
                                            offsets[2] = -1;
                                        } else if *distances == [-c, -a, b] {
                                            offsets[0] = -1;
                                            offsets[1] = -1;
                                        } else if *distances == [-c, a, -b] {
                                            offsets[0] = -1;
                                            offsets[2] = -1;
                                        }
                                        [2, 0, 1]
                                    } else {
                                        continue;
                                    };

                                    let scanner_coords = [
                                        scanner1_beacon[0]
                                            - offsets[0] * scanner2_beacon[coord_rotation[0]],
                                        scanner1_beacon[1]
                                            - offsets[1] * scanner2_beacon[coord_rotation[1]],
                                        scanner1_beacon[2]
                                            - offsets[2] * scanner2_beacon[coord_rotation[2]],
                                    ];
                                    scanners_coords.push(scanner_coords);
                                    let aligned_beacons: HashSet<_> = scanners[scanner_no]
                                        .iter()
                                        .map(|beacon| {
                                            [
                                                scanner_coords[0]
                                                    + offsets[0] * beacon[coord_rotation[0]],
                                                scanner_coords[1]
                                                    + offsets[1] * beacon[coord_rotation[1]],
                                                scanner_coords[2]
                                                    + offsets[2] * beacon[coord_rotation[2]],
                                            ]
                                        })
                                        .collect();
                                    if all_beacons.intersection(&aligned_beacons).count() >= 12 {
                                        all_beacons.extend(aligned_beacons);
                                        aligned_scanners.push(scanner_no);
                                        break 'outer;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    (all_beacons, scanners_coords)
}

fn beacon_count(all_beacons: &HashSet<Coordinates>) -> usize {
    all_beacons.len()
}

fn largest_distance_between_scanners(scanners_coords: &[Coordinates]) -> usize {
    let mut largest_distance = 0;
    for (index, scanner1) in scanners_coords.iter().enumerate() {
        for scanner2 in scanners_coords.iter().skip(index + 1) {
            largest_distance = largest_distance.max(distance(*scanner1, *scanner2));
        }
    }
    largest_distance
}

fn relative_distances(beacon1: Coordinates, beacon2: Coordinates) -> [isize; 3] {
    [
        beacon1[0] - beacon2[0],
        beacon1[1] - beacon2[1],
        beacon1[2] - beacon2[2],
    ]
}

fn distance(beacon1: Coordinates, beacon2: Coordinates) -> usize {
    beacon1[0].abs_diff(beacon2[0])
        + beacon1[1].abs_diff(beacon2[1])
        + beacon1[2].abs_diff(beacon2[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "--- scanner 0 ---
404,-588,-901\n528,-643,409\n-838,591,734\n390,-675,-793\n-537,-823,-458\n-485,-357,347
-345,-311,381\n-661,-816,-575\n-876,649,763\n-618,-824,-621\n553,345,-567\n474,580,667
-447,-329,318\n-584,868,-557\n544,-627,-890\n564,392,-477\n455,729,728\n-892,524,684
-689,845,-530\n423,-701,434\n7,-33,-71\n630,319,-379\n443,580,662\n-789,900,-551\n459,-707,401

--- scanner 1 ---
686,422,578\n605,423,415\n515,917,-361\n-336,658,858\n95,138,22\n-476,619,847
-340,-569,-846\n567,-361,727\n-460,603,-452\n669,-402,600\n729,430,532\n-500,-761,534
-322,571,750\n-466,-666,-811\n-429,-592,574\n-355,545,-477\n703,-491,-529\n-328,-685,520
413,935,-424\n-391,539,-444\n586,-435,557\n-364,-763,-893\n807,-499,-711\n755,-354,-619\n553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524\n-644,584,-595\n-588,-843,648\n-30,6,44\n-674,560,763\n500,723,-460
609,671,-379\n-555,-800,653\n-675,-892,-343\n697,-426,-610\n578,704,681\n493,664,-388
-671,-858,530\n-667,343,800\n571,-461,-707\n-138,-166,112\n-889,563,-600\n646,-828,498
640,759,510\n-630,509,768\n-681,-892,-333\n673,-379,-804\n-742,-814,-386\n577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669\n-500,565,-823\n-660,373,557\n-458,-679,-417\n-488,449,543\n-626,468,-788
338,-750,-386\n528,-832,-391\n562,-778,733\n-938,-730,414\n543,643,-506\n-524,371,-870
407,773,750\n-104,29,83\n378,-903,-323\n-778,-728,485\n426,699,580\n-438,-605,-362
-469,-447,-387\n509,732,623\n647,635,-688\n-868,-804,481\n614,-800,639\n595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779\n441,611,-461\n-714,465,-776\n-743,427,-804\n-660,-479,-426
832,-632,460\n927,-485,-438\n408,393,-506\n466,436,-512\n110,16,151
-258,-428,682\n-393,719,612\n-211,-452,876\n808,-476,-593\n-575,615,604
-485,667,467\n-680,325,-822\n-627,-443,-432\n872,-547,-609\n833,512,582
807,604,487\n839,-516,451\n891,-625,532\n-652,-548,-490\n30,-46,-14";

    #[test]
    fn test_part_1_sample() {
        let scanners = parse_scanners(SAMPLE);
        let (all_beacons, _) = align_scanners(&scanners);

        assert_eq!(beacon_count(&all_beacons), 79);
    }

    #[test]
    fn test_part_2_sample() {
        let scanners = parse_scanners(SAMPLE);
        let (_, scanners_coords) = align_scanners(&scanners);

        assert_eq!(largest_distance_between_scanners(&scanners_coords), 3621);
    }
}
