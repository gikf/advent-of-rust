use std::fs;
use std::path::Path;

type Pair = ((usize, usize), (usize, usize));

fn main() {
    let input = Path::new("2022/day04/src/input.txt");
    let pairs = parse_pairs(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Pairs with fully contained assignments: {:?}",
        count_fully_contained(&pairs)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Pairs with overlapping assignments: {:?}",
        count_overlapping(&pairs)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_pairs(input: &str) -> Vec<Pair> {
    input
        .lines()
        .map(|line| {
            let mut pair = line
                .split(",")
                .map(|part| part.split('-').map(|num| num.parse::<usize>().unwrap()));
            let (mut left, mut right) = (pair.next().unwrap(), pair.next().unwrap());

            (
                (left.next().unwrap(), left.next().unwrap()),
                (right.next().unwrap(), right.next().unwrap()),
            )
        })
        .collect()
}

fn count_fully_contained(pairs: &[Pair]) -> usize {
    pairs
        .iter()
        .filter(|(a, b)| (a.0 <= b.0 && a.1 >= b.1) || (a.0 >= b.0 && a.1 <= b.1))
        .count()
}

fn count_overlapping(pairs: &[Pair]) -> usize {
    pairs
        .iter()
        .filter(|(a, b)| !(a.1 < b.0 || a.0 > b.1))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_parse_pairs() {
        let pairs = parse_pairs(SAMPLE);

        assert_eq!(
            pairs,
            [
                ((2, 4), (6, 8)),
                ((2, 3), (4, 5)),
                ((5, 7), (7, 9)),
                ((2, 8), (3, 7)),
                ((6, 6), (4, 6)),
                ((2, 6), (4, 8)),
            ]
        )
    }

    #[test]
    fn test_part_1_sample() {
        let pairs = parse_pairs(SAMPLE);

        assert_eq!(count_fully_contained(&pairs), 2);
    }

    #[test]
    fn test_part_2_sample() {
        let pairs = parse_pairs(SAMPLE);

        assert_eq!(count_overlapping(&pairs), 4);
    }
}
