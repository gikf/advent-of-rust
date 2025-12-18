use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

type Update = Vec<usize>;
type Ordering = (usize, usize);
type FromTo = HashMap<usize, HashSet<usize>>;

fn main() {
    let input = Path::new("2024/day05/src/input.txt");
    let (orderings, updates) = parse_input(&fs::read_to_string(input).unwrap());

    println!("Part 1");
    println!(
        "Sum of middle page numbers, from correctly ordered updates: {:?}",
        part_1(&updates, &orderings)
    );

    println!("Part 2");
    println!(
        "Sum of middle page numbers, from corrected invalid updates: {:?}",
        part_2(&updates, &orderings)
    );
}

fn part_1(updates: &[Update], orderings: &[Ordering]) -> usize {
    let valid_updates = filter_valid_updates(updates, orderings);

    sum_of_middle(&valid_updates)
}

fn part_2(updates: &[Update], orderings: &[Ordering]) -> usize {
    let (num_to_before, num_to_after) = from_to_orderings(orderings);

    let corrected_updates: Vec<_> = updates
        .iter()
        .filter(|update| !is_update_valid(update, &num_to_before, &num_to_after))
        .map(|update| correct_update(update, &num_to_after))
        .collect();

    sum_of_middle(&corrected_updates)
}

fn correct_update(update: &Update, num_to_after: &FromTo) -> Vec<usize> {
    let mut corrected = Vec::new();
    let mut num_to_before_count = HashMap::new();

    for num in update.iter() {
        corrected.push(*num);
        num_to_before_count.insert(num, 0);
        for num2 in update.iter() {
            if num == num2 {
                continue;
            }

            if let Some(on_right_of_num) = num_to_after.get(num)
                && on_right_of_num.contains(num2)
            {
                num_to_before_count
                    .entry(num)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }
    }

    corrected.sort_by_key(|num| -num_to_before_count.get(num).unwrap());
    corrected
}

fn filter_valid_updates(updates: &[Update], orderings: &[Ordering]) -> Vec<Update> {
    let (num_to_before, num_to_after) = from_to_orderings(orderings);
    updates
        .iter()
        .filter(|update| is_update_valid(update, &num_to_before, &num_to_after))
        .cloned()
        .collect()
}

fn is_update_valid(update: &Update, num_to_before: &FromTo, num_to_after: &FromTo) -> bool {
    for (index, num_to_check) in update.iter().enumerate() {
        if let Some(right_from_num_to_check) = num_to_after.get(num_to_check) {
            for num_on_right in update[(index + 1)..].iter() {
                if right_from_num_to_check.contains(num_on_right) {
                    continue;
                }
                if let Some(before_num_on_right) = num_to_before.get(num_on_right)
                    && before_num_on_right.contains(num_to_check)
                {
                    return false;
                }
            }
        }

        if let Some(before_num_to_check) = num_to_before.get(num_to_check) {
            for num_on_left in update[(index + 1)..].iter() {
                if before_num_to_check.contains(num_on_left) {
                    return false;
                }
                if let Some(after_num_on_left) = num_to_after.get(num_on_left)
                    && after_num_on_left.contains(num_to_check)
                {
                    continue;
                }
            }
        }
    }

    true
}

fn sum_of_middle(updates: &[Update]) -> usize {
    updates
        .iter()
        .map(|update| {
            let middle = update.len().div_euclid(2);
            update[middle]
        })
        .sum()
}

fn from_to_orderings(orderings: &[Ordering]) -> (FromTo, FromTo) {
    let mut num_to_before: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut num_to_after: HashMap<usize, HashSet<usize>> = HashMap::new();

    for (left, right) in orderings {
        num_to_after
            .entry(*left)
            .and_modify(|set| {
                set.insert(*right);
            })
            .or_insert(HashSet::from([*right]));
        num_to_before
            .entry(*right)
            .and_modify(|set| {
                set.insert(*left);
            })
            .or_insert(HashSet::from([*left]));
    }

    (num_to_before, num_to_after)
}

fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let mut orderings = Vec::new();
    let mut updates = Vec::new();

    let mut lines = input.lines();

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        if let Some((a, b)) = line.split_once('|') {
            orderings.push((a.parse().unwrap(), b.parse().unwrap()));
        }
    }

    for line in lines {
        updates.push(line.split(',').map(|num| num.parse().unwrap()).collect())
    }

    (orderings, updates)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "47|53
97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53
97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13

75,47,61,53,29\n97,61,53,29,13\n75,29,13\n75,97,47,61,53\n61,13,29\n97,13,75,29,47";

    #[test]
    fn test_parse_input() {
        let (orderings, updates) = parse_input(SAMPLE);

        assert_eq!(
            orderings,
            [
                (47, 53),
                (97, 13),
                (97, 61),
                (97, 47),
                (75, 29),
                (61, 13),
                (75, 53),
                (29, 13),
                (97, 29),
                (53, 29),
                (61, 53),
                (97, 53),
                (61, 29),
                (47, 13),
                (75, 47),
                (97, 75),
                (47, 61),
                (75, 61),
                (47, 29),
                (75, 13),
                (53, 13),
            ]
        );
        assert_eq!(
            updates,
            Vec::from([
                Vec::from([75, 47, 61, 53, 29]),
                Vec::from([97, 61, 53, 29, 13]),
                Vec::from([75, 29, 13]),
                Vec::from([75, 97, 47, 61, 53]),
                Vec::from([61, 13, 29]),
                Vec::from([97, 13, 75, 29, 47]),
            ])
        )
    }

    #[test]
    fn test_is_update_valid() {
        let (orderings, updates) = parse_input(SAMPLE);
        let (num_to_before, num_to_after) = from_to_orderings(&orderings);

        assert_eq!(
            is_update_valid(&updates[0], &num_to_before, &num_to_after),
            true
        );
        assert_eq!(
            is_update_valid(&updates[1], &num_to_before, &num_to_after),
            true
        );
        assert_eq!(
            is_update_valid(&updates[2], &num_to_before, &num_to_after),
            true
        );
        assert_eq!(
            is_update_valid(&updates[3], &num_to_before, &num_to_after),
            false
        );
        assert_eq!(
            is_update_valid(&updates[4], &num_to_before, &num_to_after),
            false
        );
        assert_eq!(
            is_update_valid(&updates[5], &num_to_before, &num_to_after),
            false
        );
    }

    #[test]
    fn test_part_1_sample() {
        let (orderings, updates) = parse_input(SAMPLE);

        assert_eq!(part_1(&updates, &orderings), 143)
    }

    #[test]
    fn test_correct_update() {
        let (orderings, updates) = parse_input(SAMPLE);

        let (_, num_to_after) = from_to_orderings(&orderings);

        assert_eq!(
            correct_update(&updates[3], &num_to_after),
            [97, 75, 47, 61, 53]
        );
        assert_eq!(correct_update(&updates[4], &num_to_after), [61, 29, 13]);
        assert_eq!(
            correct_update(&updates[5], &num_to_after),
            [97, 75, 47, 29, 13]
        );
    }

    #[test]
    fn test_part_2_sample() {
        let (orderings, updates) = parse_input(SAMPLE);

        let result = part_2(&updates, &orderings);

        assert_eq!(result, 123);
    }
}
