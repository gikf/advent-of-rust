use std::collections::HashSet;
use std::fs;
use std::path::Path;

fn main() {
    let input = Path::new("2024/day19/src/input.txt");
    let contents = &fs::read_to_string(input).unwrap();
    let (designs, patterns) = parse_input(contents);

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Number of possible designs: {:?}",
        count_possible_designs(&designs, &patterns)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Sum of number of different ways to make design: {:?}",
        count_all_possible_designs(&designs, &patterns)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_input(input: &str) -> (Vec<&str>, HashSet<&str>) {
    let mut lines = input.lines();

    let patterns: HashSet<&str> = lines.next().unwrap().split(", ").collect();

    lines.next();

    let designs = lines.collect();

    (designs, patterns)
}

fn find_possible_designs<'a>(designs: &'a [&str], patterns: &HashSet<&str>) -> Vec<&'a &'a str> {
    designs
        .iter()
        .filter(move |design| is_design_possible(design, patterns))
        .collect()
}

fn count_possible_designs(designs: &[&str], patterns: &HashSet<&str>) -> usize {
    find_possible_designs(designs, patterns).len()
}

fn count_all_possible_designs(designs: &[&str], patterns: &HashSet<&str>) -> usize {
    designs
        .iter()
        .map(move |design| count_unique_ways(design, patterns))
        .sum()
}

fn is_design_possible(design: &str, patterns: &HashSet<&str>) -> bool {
    if design.is_empty() {
        return true;
    }

    let relevant_patterns: Vec<_> = patterns
        .iter()
        .filter(|pattern| design.contains(*pattern))
        .collect();
    let mut seen = HashSet::new();

    let mut stack = vec![(design, vec![])];

    while let Some((left, past)) = stack.pop() {
        if left.is_empty() {
            return true;
        }

        if seen.contains(left) {
            continue;
        }

        seen.insert(left);

        if (relevant_patterns)
            .iter()
            .all(|pattern| !pattern.starts_with(&design[0..1]))
        {
            continue;
        }

        for pattern in &relevant_patterns {
            if **pattern == left {
                return true;
            }
            if left.starts_with(**pattern) {
                let mut new_past = past.clone();
                new_past.push(pattern);
                stack.push((&left[pattern.len()..], new_past));
            }
        }
    }
    false
}

#[allow(unused)]
fn is_design_possible2(design: &str, patterns: &HashSet<&str>) -> bool {
    let longest_pattern = patterns.iter().max_by_key(|p| p.len()).unwrap().len();

    let mut counts = vec![0_usize; design.len() + 1];
    counts[0] = 1;

    for end_index in 1..=design.len() {
        for size in 1..=longest_pattern {
            if size > end_index {
                continue;
            }
            let start_index = end_index - size;
            if patterns.contains(&design[start_index..end_index]) {
                counts[end_index] += counts[start_index];
                if end_index == design.len() - 1 && counts[design.len()] > 0 {
                    return true;
                }
            }
        }
    }
    counts[counts.len() - 1] > 0
}

fn count_unique_ways(design: &str, patterns: &HashSet<&str>) -> usize {
    let longest_pattern = patterns.iter().max_by_key(|p| p.len()).unwrap().len();

    let mut counts = vec![0; design.len() + 1];
    counts[0] = 1;

    for end_index in 1..=design.len() {
        for size in 1..=longest_pattern {
            if size > end_index {
                continue;
            }
            let start_index = end_index - size;
            if patterns.contains(&design[start_index..end_index]) {
                counts[end_index] += counts[start_index];
            }
        }
    }
    counts[counts.len() - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test_parse_input() {
        let (designs, patterns) = parse_input(SAMPLE);

        assert_eq!(
            designs,
            [
                "brwrr", "bggr", "gbbr", "rrbgbr", "ubwu", "bwurrg", "brgr", "bbrgwb"
            ]
        );
        assert_eq!(
            patterns,
            HashSet::from(["r", "wr", "b", "g", "bwu", "rb", "gb", "br"])
        );
    }

    #[test]
    fn test_is_design_possible() {
        let (_, patterns) = parse_input(SAMPLE);
        assert_eq!(is_design_possible("", &patterns), true);
        assert_eq!(is_design_possible("r", &patterns), true);
        assert_eq!(is_design_possible("bwu", &patterns), true);
        let input = Path::new("src/input.txt");
        let contents = &fs::read_to_string(input).unwrap();
        let (_, patterns) = parse_input(contents);

        assert_eq!(
            is_design_possible(
                "gwwbbrugrggrwuuugggwgurbrurbrrggwwbgbwwbbrwwwurwwuu",
                &patterns
            ),
            true
        );
    }

    #[test]
    fn test_count_unique_ways() {
        let (_, patterns) = parse_input(SAMPLE);
        assert_eq!(count_unique_ways("brwrr", &patterns), 2);
        assert_eq!(count_unique_ways("bgrr", &patterns), 1);
        assert_eq!(count_unique_ways("gbbr", &patterns), 4);
        assert_eq!(count_unique_ways("rrbgbr", &patterns), 6);
        assert_eq!(count_unique_ways("bwurrg", &patterns), 1);
        assert_eq!(count_unique_ways("brgr", &patterns), 2);
        let input = Path::new("src/input.txt");
        let contents = &fs::read_to_string(input).unwrap();
        let (_, patterns) = parse_input(contents);

        assert_eq!(
            count_unique_ways(
                "gwwbbrugrggrwuuugggwgurbrurbrrggwwbgbwwbbrwwwurwwuu",
                &patterns
            ),
            112051105380
        );
    }

    #[test]
    fn test_part_1_sample() {
        let (designs, patterns) = parse_input(SAMPLE);
        let possible_designs = count_possible_designs(&designs, &patterns);
        assert_eq!(possible_designs, 6);
    }

    #[test]
    fn test_part_2_sample() {
        let (designs, patterns) = parse_input(SAMPLE);
        let all_designs = count_all_possible_designs(&designs, &patterns);
        assert_eq!(all_designs, 16);
    }
}
