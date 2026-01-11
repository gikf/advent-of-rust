use std::collections::HashSet;
use std::fs;
use std::path::Path;

fn main() {
    let input = Path::new("2022/day03/src/input.txt");
    let content = fs::read_to_string(input).unwrap();
    let rucksacks = parse_rucksacks(&content);

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Sum of repeated item priorities: {:?}",
        sum_item_priorities_in_rucksack(&rucksacks)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Sum of repeated item priorities in 3 Elves groups: {:?}",
        sum_item_priorities_in_rucksack_groups(&rucksacks)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_rucksacks(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn repeated_items_in_rucksack(rucksack: &str) -> HashSet<char> {
    let (items_a, items_b) = rucksack.split_at(rucksack.len() / 2);
    let items_a: HashSet<_> = HashSet::from_iter(items_a.chars());

    items_a
        .intersection(&HashSet::from_iter(items_b.chars()))
        .cloned()
        .collect()
}

fn repeated_items_in_rucksack_group(rucksacks: &[&str]) -> HashSet<char> {
    let mut repeated = HashSet::from_iter(rucksacks[0].chars());
    for rucksack in rucksacks.iter().skip(1) {
        repeated = repeated
            .intersection(&HashSet::from_iter(rucksack.chars()))
            .cloned()
            .collect();
    }
    repeated
}

fn item_priority(item: &char) -> usize {
    if let Some(position) = ('a'..='z').chain('A'..='Z').position(|char| char == *item) {
        position + 1
    } else {
        0
    }
}

fn sum_item_priorities_in_rucksack(rucksacks: &[&str]) -> usize {
    rucksacks
        .iter()
        .map(|rucksack| {
            repeated_items_in_rucksack(rucksack)
                .iter()
                .map(item_priority)
                .sum::<usize>()
        })
        .sum()
}

fn sum_item_priorities_in_rucksack_groups(rucksacks: &[&str]) -> usize {
    rucksacks
        .chunks(3)
        .map(|three_rucksacks| {
            repeated_items_in_rucksack_group(three_rucksacks)
                .iter()
                .map(item_priority)
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_parse_rucksacks() {
        let rucksacks = parse_rucksacks(SAMPLE);

        assert_eq!(
            rucksacks,
            [
                ("vJrwpWtwJgWrhcsFMMfFFhFp"),
                ("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
                ("PmmdzqPrVvPwwTWBwg"),
                ("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
                ("ttgJtRGJQctTZtZT"),
                ("CrZsJsPPZsGzwwsLwLmpwMDw")
            ]
        )
    }

    #[test]
    fn test_find_repeated_items() {
        let rucksacks = parse_rucksacks(SAMPLE);

        assert_eq!(
            repeated_items_in_rucksack(&rucksacks[0]),
            HashSet::from(['p'])
        );
        assert_eq!(
            repeated_items_in_rucksack(&rucksacks[1]),
            HashSet::from(['L'])
        );
        assert_eq!(
            repeated_items_in_rucksack(&rucksacks[2]),
            HashSet::from(['P'])
        );
        assert_eq!(
            repeated_items_in_rucksack(&rucksacks[3]),
            HashSet::from(['v'])
        );
        assert_eq!(
            repeated_items_in_rucksack(&rucksacks[4]),
            HashSet::from(['t'])
        );
        assert_eq!(
            repeated_items_in_rucksack(&rucksacks[5]),
            HashSet::from(['s'])
        );
    }

    #[test]
    fn test_item_priority() {
        assert_eq!(item_priority(&'p'), 16);
        assert_eq!(item_priority(&'L'), 38);
        assert_eq!(item_priority(&'P'), 42);
        assert_eq!(item_priority(&'v'), 22);
        assert_eq!(item_priority(&'t'), 20);
        assert_eq!(item_priority(&'s'), 19);
    }

    #[test]
    fn test_sum_repeated_item_priorities() {
        let rucksacks = parse_rucksacks(SAMPLE);

        assert_eq!(sum_item_priorities_in_rucksack(&rucksacks), 157);
    }

    #[test]
    fn test_find_repeated_items_across_rucksacks() {
        let rucksacks = parse_rucksacks(SAMPLE);

        assert_eq!(
            repeated_items_in_rucksack_group(&rucksacks[0..=2]),
            HashSet::from(['r'])
        );
        assert_eq!(
            repeated_items_in_rucksack_group(&rucksacks[3..=5]),
            HashSet::from(['Z'])
        );
    }

    #[test]
    fn test_sum_repeated_item_priorities_from_groups() {
        let rucksacks = parse_rucksacks(SAMPLE);

        assert_eq!(sum_item_priorities_in_rucksack_groups(&rucksacks), 70);
    }
}
