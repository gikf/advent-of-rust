use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::path::Path;

type Xyz = (usize, usize, usize);
type Brick = (Xyz, Xyz);

fn main() {
    let input = Path::new("2023/day22/src/input.txt");
    let bricks = parse_bricks(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Number of bricks to safely disintegrate: {:?}",
        bricks_to_disintegrate(&bricks)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Sum of bricks that would fall, after disintegrating individual bricks: {:?}",
        bricks_falling_after_desintegration(&bricks)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_bricks(input: &str) -> Vec<Brick> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split("~").map(|part| {
                part.split(",")
                    .map(|v| v.parse::<usize>().unwrap())
                    .collect()
            });
            let start: Vec<_> = parts.next().unwrap();
            let end = parts.next().unwrap();
            ((start[0], start[1], start[2]), (end[0], end[1], end[2]))
        })
        .collect()
}

fn fall_bricks(bricks: &[Brick]) -> (HashMap<Brick, Vec<Brick>>, HashMap<Brick, Vec<Brick>>) {
    let mut bricks = bricks.to_vec();
    bricks.sort_by_key(|b| b.0.2);
    let mut on_ground = Vec::new();
    let mut supports = HashMap::new();
    let mut supported_by = HashMap::new();

    for mut brick in bricks.drain(..) {
        let (start, _) = &brick;
        if start.2 == 1 {
            on_ground.push(brick);
            supports.insert(brick, vec![]);
            continue;
        }

        let height = brick.1.2 - brick.0.2;
        let mut highest_hit = 0;
        let mut hit_bricks = Vec::new();
        for other_brick in on_ground.iter().rev() {
            if are_overlaping(&brick, other_brick) {
                let other_height = other_brick.1.2;
                if other_height > highest_hit {
                    highest_hit = other_height;
                    hit_bricks = vec![other_brick];
                } else if other_height == highest_hit {
                    hit_bricks.push(other_brick);
                }
            }
        }

        brick.0.2 = highest_hit + 1;
        brick.1.2 = brick.0.2 + height;
        hit_bricks.into_iter().for_each(|hit_brick| {
            supports
                .entry(*hit_brick)
                .and_modify(|other: &mut Vec<Brick>| other.push(brick))
                .or_insert(vec![brick]);
            supported_by
                .entry(brick)
                .and_modify(|others: &mut Vec<Brick>| others.push(*hit_brick))
                .or_insert(vec![*hit_brick]);
        });
        supports.entry(brick).or_insert(vec![]);
        on_ground.push(brick);
    }
    (supports, supported_by)
}

fn bricks_to_disintegrate(bricks: &[Brick]) -> usize {
    find_safe_bricks(bricks).len()
}

fn find_safe_bricks(bricks: &[Brick]) -> Vec<Brick> {
    let (supports, supported_by) = fall_bricks(bricks);
    filter_safe(&supports, &supported_by)
}

fn filter_safe(
    supports: &HashMap<Brick, Vec<Brick>>,
    supported_by: &HashMap<Brick, Vec<Brick>>,
) -> Vec<Brick> {
    supports
        .iter()
        .filter_map(|(b, list)| {
            if list.is_empty()
                || list
                    .iter()
                    .all(|other| supported_by.get(other).unwrap().len() > 1)
            {
                Some(*b)
            } else {
                None
            }
        })
        .collect()
}

fn bricks_falling_after_desintegration(bricks: &[Brick]) -> usize {
    let (supports, supported_by) = fall_bricks(bricks);
    let safe_bricks = filter_safe(&supports, &supported_by);

    let unsafe_bricks: Vec<_> = supports
        .iter()
        .filter_map(|(b, _)| {
            if !safe_bricks.contains(b) {
                Some(*b)
            } else {
                None
            }
        })
        .collect();
    calculate_falling(&supports, &supported_by, &unsafe_bricks)
}

fn calculate_falling(
    supports: &HashMap<Brick, Vec<Brick>>,
    supported_by: &HashMap<Brick, Vec<Brick>>,
    unsafe_bricks: &[Brick],
) -> usize {
    let mut fallen = HashSet::with_capacity(supports.len());

    unsafe_bricks
        .iter()
        .map(|brick| {
            fallen.clear();
            let mut falling: VecDeque<_> = supports
                .get(brick)
                .unwrap()
                .iter()
                .filter(|supported_brick| supported_by.get(*supported_brick).unwrap().len() == 1)
                .collect();
            while let Some(falling_brick) = falling.pop_front() {
                fallen.insert(falling_brick);
                supports
                    .get(falling_brick)
                    .unwrap()
                    .iter()
                    .for_each(|supported_brick| {
                        if supported_by
                            .get(supported_brick)
                            .unwrap()
                            .iter()
                            .all(|supporting_bricks| fallen.contains(supporting_bricks))
                        {
                            falling.push_back(supported_brick);
                        }
                    });
            }
            fallen.len()
        })
        .sum()
}

fn are_overlaping(a: &Brick, b: &Brick) -> bool {
    let xses = !(a.1.0 < b.0.0 || b.1.0 < a.0.0);
    let yses = !(a.1.1 < b.0.1 || b.1.1 < a.0.1);

    xses && yses
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
    const SAMPLE2: &str = "0,0,1~0,0,2
1,0,1~2,0,1
1,0,2~1,0,2
0,0,3~1,0,3";
    const SAMPLE3: &str = "0,0,5~5,0,5
0,5,5~5,5,5
3,3,6~3,3,9
3,0,11~3,5,11";
    const SAMPLE4: &str = "0,0,1~0,5,1
0,6,1~0,9,1
0,0,2~0,0,2
0,3,2~0,8,2";
    const SAMPLE5: &str = "0,0,1~0,0,1
1,0,1~2,0,1
0,0,2~1,0,2
2,0,2~2,0,2";

    #[test]
    fn test_do_overlap() {
        assert_eq!(
            are_overlaping(&((0, 0, 4), (1, 0, 4)), &((0, 0, 1), (0, 0, 3))),
            true
        );
        assert_eq!(
            are_overlaping(&((0, 0, 1), (0, 0, 3)), &((0, 0, 4), (1, 0, 4))),
            true
        );
    }
    #[test]
    fn test_part_1_sample() {
        let bricks = parse_bricks(SAMPLE);
        let bricks2 = parse_bricks(SAMPLE2);
        let bricks3 = parse_bricks(SAMPLE3);
        let bricks4 = parse_bricks(SAMPLE4);
        let bricks5 = parse_bricks(SAMPLE5);

        assert_eq!(bricks_to_disintegrate(&bricks), 5);
        assert_eq!(bricks_to_disintegrate(&bricks2), 3);
        assert_eq!(bricks_to_disintegrate(&bricks3), 3);
        assert_eq!(bricks_to_disintegrate(&bricks4), 3);
        assert_eq!(bricks_to_disintegrate(&bricks5), 3);
    }

    #[test]
    fn test_part_2_sample() {
        let bricks = parse_bricks(SAMPLE);

        assert_eq!(bricks_falling_after_desintegration(&bricks), 7);
    }
}
