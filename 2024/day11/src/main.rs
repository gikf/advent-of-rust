use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Hash, PartialEq)]
enum StoneCount {
    Num(usize),
    Ref(usize),
    NthRef(usize, usize),
}

fn main() {
    let input = Path::new("2024/day11/src/input.txt");
    let stones = parse_stones(&fs::read_to_string(input).unwrap());

    let part1_stones = stones.clone();
    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Number of stones after 25 blinks: {:?}",
        stones_after_blinks(part1_stones, 25)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Number of stones after 75 blinks: {:?}",
        blink_stones_by(stones, 75)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_stones(input: &str) -> Vec<usize> {
    input
        .split_ascii_whitespace()
        .map(|part| part.parse().unwrap())
        .collect()
}

fn blink_stones_by(stones: Vec<usize>, count: usize) -> usize {
    let stone_to_count = get_stone_to_count();
    let mut _calculated: HashMap<(usize, usize), usize> = HashMap::new();
    stones
        .iter()
        .map(|stone| blink_stone_by(*stone, count, &stone_to_count, &mut _calculated))
        .sum()
}

fn blink_stone_by(
    stone: usize,
    blinks: usize,
    stone_to_count: &HashMap<usize, Vec<Vec<StoneCount>>>,
    _calculated: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if blinks == 0 && stone > 9 {
        return 1;
    }

    if let Some(result) = _calculated.get(&(stone, blinks)) {
        return *result;
    }

    match (stone, num_of_digits(stone)) {
        (num, _) if num < 10 => {
            let stone_stages = stone_to_count.get(&num).unwrap();
            let (stage, to_count, ref_count_offset) = if stone_stages.len() > blinks {
                (blinks, 1, 0)
            } else {
                (
                    stone_stages.len() - 1,
                    blinks - stone_stages.len() + 1,
                    blinks - stone_stages.len(),
                )
            };

            let res = stone_stages
                .get(stage)
                .unwrap()
                .iter()
                .map(|stone_count| match *stone_count {
                    StoneCount::Num(c) => c,
                    StoneCount::Ref(stone_ref) => {
                        blink_stone_by(stone_ref, to_count, stone_to_count, _calculated)
                    }
                    StoneCount::NthRef(stone_ref, ref_count) => blink_stone_by(
                        stone_ref,
                        ref_count + ref_count_offset,
                        stone_to_count,
                        _calculated,
                    ),
                })
                .sum();

            _calculated.insert((stone, blinks), res);
            res
        }
        (num, digits) if digits.is_multiple_of(2) => {
            let left = num / 10usize.pow(digits / 2);
            let right = num % 10usize.pow(digits / 2);

            let res = blink_stone_by(left, blinks - 1, stone_to_count, _calculated)
                + blink_stone_by(right, blinks - 1, stone_to_count, _calculated);

            _calculated.insert((stone, blinks), res);
            res
        }
        (num, _) => {
            let res = blink_stone_by(num * 2024, blinks - 1, stone_to_count, _calculated);

            _calculated.insert((stone, blinks), res);
            res
        }
    }
}

fn blink(stones: Vec<usize>) -> Vec<usize> {
    let mut new_stones = Vec::new();

    for stone in stones {
        let new = match (stone, num_of_digits(stone)) {
            (0, _) => vec![1],
            (num, digits) if digits.is_multiple_of(2) => {
                let left = num / 10usize.pow(digits / 2);
                let right = num % 10usize.pow(digits / 2);
                vec![left, right]
            }
            (num, _) => {
                vec![num * 2024]
            }
        };
        for s in new.iter() {
            new_stones.push(*s);
        }
    }
    new_stones
}

fn stones_after_blinks(stones: Vec<usize>, blinks: usize) -> usize {
    let mut stones = stones;
    for _ in 0..blinks {
        stones = blink(stones);
    }
    stones.len()
}

fn num_of_digits(num: usize) -> u32 {
    let mut digits = 1;
    while num >= 10usize.pow(digits) {
        digits += 1;
    }
    digits
}

fn get_stone_to_count() -> HashMap<usize, Vec<Vec<StoneCount>>> {
    HashMap::from_iter(Vec::from([
        (
            0,
            vec![
                vec![StoneCount::Num(1)],
                vec![StoneCount::Num(1)],
                vec![StoneCount::Num(1)],
                vec![StoneCount::Num(2)],
                vec![
                    StoneCount::Ref(2),
                    StoneCount::Ref(0),
                    StoneCount::Ref(2),
                    StoneCount::Ref(4),
                ],
            ],
        ),
        (
            1,
            vec![
                vec![StoneCount::Num(1)],
                vec![StoneCount::Num(1)],
                vec![StoneCount::Num(2)],
                vec![
                    StoneCount::Ref(2),
                    StoneCount::Ref(0),
                    StoneCount::Ref(2),
                    StoneCount::Ref(4),
                ],
            ],
        ),
        (
            2,
            vec![
                vec![StoneCount::Num(1)],
                vec![StoneCount::Num(1)],
                vec![StoneCount::Num(2)],
                vec![
                    StoneCount::Ref(4),
                    StoneCount::Ref(0),
                    StoneCount::Ref(4),
                    StoneCount::Ref(8),
                ],
            ],
        ),
        (
            3,
            vec![
                vec![StoneCount::Num(1)],
                vec![StoneCount::Num(1)],
                vec![StoneCount::Num(2)],
                vec![
                    StoneCount::Ref(6),
                    StoneCount::Ref(0),
                    StoneCount::Ref(7),
                    StoneCount::Ref(2),
                ],
            ],
        ),
        (
            4,
            vec![
                vec![StoneCount::Num(1)],
                vec![StoneCount::Num(1)],
                vec![StoneCount::Num(2)],
                vec![
                    StoneCount::Ref(8),
                    StoneCount::Ref(0),
                    StoneCount::Ref(9),
                    StoneCount::Ref(6),
                ],
            ],
        ),
        (
            5,
            vec![
                vec![StoneCount::Num(1)],
                vec![StoneCount::Num(1)],
                vec![StoneCount::Num(1)],
                vec![StoneCount::Num(2)],
                vec![StoneCount::Num(4)],
                vec![
                    StoneCount::Ref(2),
                    StoneCount::Ref(0),
                    StoneCount::Ref(4),
                    StoneCount::Ref(8),
                    StoneCount::Ref(2),
                    StoneCount::Ref(8),
                    StoneCount::Ref(8),
                    StoneCount::Ref(0),
                ],
            ],
        ),
        (
            6,
            vec![
                vec![StoneCount::Num(1)],
                vec![StoneCount::Num(1)],
                vec![StoneCount::Num(1)],
                vec![StoneCount::Num(2)],
                vec![StoneCount::Num(4)],
                vec![
                    StoneCount::Ref(2),
                    StoneCount::Ref(4),
                    StoneCount::Ref(5),
                    StoneCount::Ref(7),
                    StoneCount::Ref(9),
                    StoneCount::Ref(4),
                    StoneCount::Ref(5),
                    StoneCount::Ref(6),
                ],
            ],
        ),
        (
            7,
            vec![
                vec![StoneCount::Num(1)],
                vec![StoneCount::Num(1)],
                vec![StoneCount::Num(1)],
                vec![StoneCount::Num(2)],
                vec![StoneCount::Num(4)],
                vec![
                    StoneCount::Ref(2),
                    StoneCount::Ref(8),
                    StoneCount::Ref(6),
                    StoneCount::Ref(7),
                    StoneCount::Ref(6),
                    StoneCount::Ref(0),
                    StoneCount::Ref(3),
                    StoneCount::Ref(2),
                ],
            ],
        ),
        (
            8,
            vec![
                vec![StoneCount::Num(1)],
                vec![StoneCount::Num(1)],
                vec![StoneCount::Num(1)],
                vec![StoneCount::Num(2)],
                vec![StoneCount::Num(4)],
                vec![
                    StoneCount::Ref(3),
                    StoneCount::Ref(2),
                    StoneCount::Ref(7),
                    StoneCount::Ref(7),
                    StoneCount::Ref(2),
                    StoneCount::Ref(6),
                    StoneCount::NthRef(8, 2),
                ],
            ],
        ),
        (
            9,
            vec![
                vec![StoneCount::Num(1)],
                vec![StoneCount::Num(1)],
                vec![StoneCount::Num(1)],
                vec![StoneCount::Num(2)],
                vec![StoneCount::Num(4)],
                vec![
                    StoneCount::Ref(3),
                    StoneCount::Ref(6),
                    StoneCount::Ref(8),
                    StoneCount::Ref(6),
                    StoneCount::Ref(9),
                    StoneCount::Ref(1),
                    StoneCount::Ref(8),
                    StoneCount::Ref(4),
                ],
            ],
        ),
    ]))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "125 17";
    const SAMPLE2: &str = "0 1 10 99 999";

    #[test]
    fn test_parse_stones() {
        assert_eq!(parse_stones(SAMPLE), [125, 17]);
        assert_eq!(parse_stones(SAMPLE2), [0, 1, 10, 99, 999]);
    }

    #[test]
    fn test_num_of_digits() {
        assert_eq!(num_of_digits(99), 2);
        assert_eq!(num_of_digits(999), 3);
    }

    #[test]
    fn test_blink() {
        let stones = parse_stones(SAMPLE2);
        assert_eq!(blink(stones), [1, 2024, 1, 0, 9, 9, 2021976]);

        assert_eq!(blink(vec![253000, 1, 7]), [253, 0, 2024, 14168]);
        assert_eq!(
            blink(vec![253, 0, 2024, 14168]),
            [512072, 1, 20, 24, 28676032]
        );
        assert_eq!(
            blink(vec![512072, 1, 20, 24, 28676032],),
            [512, 72, 2024, 2, 0, 2, 4, 2867, 6032]
        );
        assert_eq!(
            blink(vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032],),
            [1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32]
        );
        assert_eq!(
            blink(vec![
                1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32
            ],),
            [
                2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6,
                0, 3, 2
            ]
        );
    }

    #[test]
    fn test_blink_stones_by() {
        let stones = parse_stones(SAMPLE);

        let result = blink_stones_by(stones, 25);
        assert_eq!(result, 55312);
    }

    #[test]
    fn test_blink_stone_by() {
        let stone_to_count = get_stone_to_count();
        assert_eq!(
            blink_stone_by(0, 1, &stone_to_count, &mut HashMap::new()),
            1
        );
        assert_eq!(
            blink_stone_by(5, 5, &stone_to_count, &mut HashMap::new()),
            8
        );
        assert_eq!(
            blink_stone_by(10, 1, &stone_to_count, &mut HashMap::new()),
            2
        );
        assert_eq!(
            blink_stone_by(111, 1, &stone_to_count, &mut HashMap::new()),
            1
        );

        assert_eq!(
            blink_stone_by(222, 1, &stone_to_count, &mut HashMap::new()),
            1
        );
        assert_eq!(
            blink_stone_by(222, 2, &stone_to_count, &mut HashMap::new()),
            2
        );
        assert_eq!(
            blink_stone_by(222, 3, &stone_to_count, &mut HashMap::new()),
            2
        );
        assert_eq!(
            blink_stone_by(222, 4, &stone_to_count, &mut HashMap::new()),
            4
        );
        assert_eq!(
            blink_stone_by(222, 7, &stone_to_count, &mut HashMap::new()),
            8
        );
    }

    #[test]
    fn test_part_1_sample() {
        let stones = parse_stones(SAMPLE);

        let after_25 = stones_after_blinks(stones.clone(), 25);
        assert_eq!(after_25, 55312);
    }

    #[test]
    fn test_part_2_sample() {
        let stones = parse_stones(SAMPLE);

        let after_75 = blink_stones_by(stones, 75);
        assert_eq!(after_75, 65601038650482);
    }
}
