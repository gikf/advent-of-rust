use std::collections::HashMap;
use std::fs;
use std::path::Path;

const STEPS_PART1: usize = 10;
const STEPS_PART2: usize = 40;

fn main() {
    let input = Path::new("2021/day14/src/input.txt");
    let contents = fs::read_to_string(input).unwrap();
    let (template, insertions) = parse_formula(&contents);

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Quantity of the most common element substracted from quantity of the least common element after 10 steps: {:?}",
        step(&template, &insertions, STEPS_PART1)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Quantity of the most common element substracted from quantity of the least common element after 40 steps: {:?}",
        step_pairs(&template, &insertions, STEPS_PART2)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_formula(input: &str) -> (Vec<char>, HashMap<(char, char), char>) {
    let mut lines = input.lines();

    let template = lines.next().unwrap().chars().collect();

    lines.next().unwrap();

    let mut insertions = HashMap::new();
    for line in lines {
        let (pair, insertion) = line.split_once(" -> ").unwrap();
        let mut chars = pair.chars();
        insertions.insert(
            (chars.next().unwrap(), chars.next().unwrap()),
            insertion.chars().next().unwrap(),
        );
    }

    (template, insertions)
}

fn step(template: &[char], insertions: &HashMap<(char, char), char>, steps: usize) -> usize {
    let mut polymer = template.to_vec();

    for _ in 0..steps {
        let mut next_polymer = Vec::with_capacity(polymer.len() * 2);
        for (index, pair) in polymer.windows(2).enumerate() {
            if let &[a, b] = pair {
                if index == 0 {
                    next_polymer.push(a);
                }
                if let Some(ch) = insertions.get(&(a, b)) {
                    next_polymer.push(*ch);
                }
                next_polymer.push(b);
            }
        }
        polymer = next_polymer;
    }

    let mut occurrences = HashMap::new();
    for c in polymer {
        occurrences
            .entry(c)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut ordered: Vec<_> = occurrences.iter().collect();
    ordered.sort_by_key(|(_, count)| **count);

    ordered.last().unwrap().1 - ordered.first().unwrap().1
}

fn step_pairs(template: &[char], insertions: &HashMap<(char, char), char>, steps: usize) -> usize {
    let mut pairs = HashMap::new();
    let mut ps = template.windows(2);
    let mut f = ps.next().unwrap().iter();
    let mut first: (char, char) = (*f.next().unwrap(), *f.next().unwrap());
    for window in ps {
        if let &[a, b] = window {
            pairs
                .entry((a, b))
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }

    for _ in 0..steps {
        let mut next_pairs = HashMap::new();
        for ((a, b), pair_count) in pairs.drain() {
            if let Some(ch) = insertions.get(&(a, b)) {
                next_pairs
                    .entry((a, *ch))
                    .and_modify(|count| *count += pair_count)
                    .or_insert(pair_count);
                next_pairs
                    .entry((*ch, b))
                    .and_modify(|count| *count += pair_count)
                    .or_insert(pair_count);
            } else {
                next_pairs
                    .entry((a, b))
                    .and_modify(|count| *count += pair_count)
                    .or_insert(pair_count);
            }
        }

        let (a, b) = first;
        if let Some(ch) = insertions.get(&(a, b)) {
            first = (a, *ch);
            next_pairs
                .entry((*ch, b))
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        pairs = next_pairs;
    }

    let mut occurrences = HashMap::from([(first.0, 1), (first.1, 1)]);
    for ((_, b), pair_count) in pairs {
        occurrences
            .entry(b)
            .and_modify(|count| *count += pair_count)
            .or_insert(pair_count);
    }

    let mut ordered: Vec<_> = occurrences.iter().collect();
    ordered.sort_by_key(|(_, count)| **count);

    ordered.last().unwrap().1 - ordered.first().unwrap().1
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn test_part_1_sample() {
        let (template, insertions) = parse_formula(SAMPLE);

        assert_eq!(step(&template, &insertions, STEPS_PART1), 1588);
    }

    #[test]
    fn test_part_2_sample() {
        let (template, insertions) = parse_formula(SAMPLE);

        assert_eq!(step_pairs(&template, &insertions, STEPS_PART1), 1588);
        assert_eq!(
            step_pairs(&template, &insertions, STEPS_PART2),
            2188189693529
        );
    }
}
