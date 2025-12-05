use std::fs;
use std::path::Path;

#[derive(Clone, Debug, PartialEq)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn has(&self, num: usize) -> bool {
        self.start <= num && num <= self.end
    }

    fn len(&self) -> usize {
        self.end + 1 - self.start
    }

    fn overlap(&self, other: &Range) -> bool {
        !(self.end < other.start || self.start > other.end)
    }

    fn join(&self, other: &Range) -> Option<Range> {
        if self.overlap(other) {
            Some(Range {
                start: self.start.min(other.start),
                end: self.end.max(other.end),
            })
        } else {
            None
        }
    }
}

fn main() {
    let input = Path::new("2025/day05/src/input.txt");
    let (ranges, ingredients) = parse_input(&fs::read_to_string(input).unwrap());

    println!("Step 1");
    println!(
        "Number of fresh ingredients: {}",
        count_fresh_ingredients(&ingredients, &ranges)
    );

    println!("Step 2");
    println!(
        "Total fresh ingredients in ranges: {}",
        total_fresh_in_ranges(&ranges)
    );
}

fn parse_input(input: &str) -> (Vec<Range>, Vec<usize>) {
    let (input_ranges, input_ingredients) = input.split_once("\n\n").unwrap();

    let ranges: Vec<Range> = input_ranges
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            Range {
                start: start.parse().unwrap(),
                end: end.parse().unwrap(),
            }
        })
        .collect();

    let ingredients: Vec<usize> = input_ingredients
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    (ranges, ingredients)
}

fn count_fresh_ingredients(ingredients: &[usize], ranges: &[Range]) -> usize {
    ingredients
        .iter()
        .filter(|ingredient| ranges.iter().any(|range| range.has(**ingredient)))
        .count()
}

fn join_ranges(ranges: &[Range]) -> (Vec<Range>, bool) {
    let mut joined: Vec<Range> = Vec::new();
    let mut was_joined = false;
    for range in ranges.iter() {
        if joined.is_empty() {
            joined.push(range.clone());
            continue;
        }

        let mut was_range_added = false;
        let mut next_joined = Vec::new();
        for to_join in joined.drain(..) {
            match to_join.join(range) {
                Some(new_range) => {
                    next_joined.push(new_range);
                    was_range_added = true;
                    was_joined = true;
                }
                None => {
                    next_joined.push(to_join);
                }
            }
        }

        if !was_range_added {
            next_joined.push(range.clone());
        }
        joined = next_joined;
    }
    (joined, was_joined)
}

fn total_fresh_in_ranges(ranges: &[Range]) -> usize {
    let (mut joined, mut was_joined) = join_ranges(ranges);
    while was_joined {
        (joined, was_joined) = join_ranges(&joined);
    }
    joined.iter().map(|range| range.len()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";

    #[test]
    fn test_range_contains() {
        let range = Range {
            start: 12345,
            end: 20001,
        };

        assert_eq!(range.has(0), false);
        assert_eq!(range.has(1234), false);
        assert_eq!(range.has(12344), false);
        assert_eq!(range.has(12345), true);
        assert_eq!(range.has(12346), true);
        assert_eq!(range.has(15000), true);
        assert_eq!(range.has(20000), true);
        assert_eq!(range.has(20001), true);
        assert_eq!(range.has(20002), false);
        assert_eq!(range.has(34220), false);
    }

    #[test]
    fn test_range_length() {
        assert_eq!((Range { start: 3, end: 5 }).len(), 3);
        assert_eq!((Range { start: 10, end: 14 }).len(), 5);
        assert_eq!((Range { start: 16, end: 20 }).len(), 5);
        assert_eq!((Range { start: 12, end: 18 }).len(), 7);
    }

    #[test]
    fn test_range_overlap_bottom() {
        assert_eq!(
            (Range { start: 3, end: 5 }).overlap(&Range { start: 5, end: 8 }),
            true
        );

        assert_eq!(
            (Range { start: 3, end: 5 }).overlap(&Range { start: 4, end: 8 }),
            true
        );
    }

    #[test]
    fn test_range_overlap_top() {
        assert_eq!(
            (Range { start: 10, end: 14 }).overlap(&Range { start: 5, end: 11 }),
            true
        );
        assert_eq!(
            (Range { start: 10, end: 14 }).overlap(&Range { start: 5, end: 10 }),
            true
        );
    }

    #[test]
    fn test_range_overlap_no_overlap() {
        assert_eq!(
            (Range { start: 10, end: 14 }).overlap(&Range { start: 15, end: 18 }),
            false
        );
        assert_eq!(
            (Range { start: 1, end: 4 }).overlap(&Range { start: 5, end: 10 }),
            false
        );
    }

    #[test]
    fn test_range_join_when_overlaping() {
        assert_eq!(
            (Range { start: 3, end: 5 }).join(&Range { start: 5, end: 14 }),
            Some(Range { start: 3, end: 14 })
        );
        assert_eq!(
            (Range { start: 3, end: 5 }).join(&Range { start: 4, end: 14 }),
            Some(Range { start: 3, end: 14 })
        );

        assert_eq!(
            (Range { start: 13, end: 15 }).join(&Range { start: 5, end: 13 }),
            Some(Range { start: 5, end: 15 })
        );
        assert_eq!(
            (Range { start: 13, end: 15 }).join(&Range { start: 5, end: 14 }),
            Some(Range { start: 5, end: 15 })
        );

        assert_eq!(
            (Range { start: 3, end: 15 }).join(&Range { start: 5, end: 8 }),
            Some(Range { start: 3, end: 15 })
        );
        assert_eq!(
            (Range { start: 13, end: 14 }).join(&Range { start: 5, end: 15 }),
            Some(Range { start: 5, end: 15 })
        );
    }

    #[test]
    fn test_range_join_when_not_overlaping() {
        assert_eq!(
            (Range { start: 3, end: 5 }).join(&Range { start: 10, end: 14 }),
            None
        );
        assert_eq!(
            (Range { start: 3, end: 5 }).join(&Range { start: 6, end: 14 }),
            None
        );
        assert_eq!(
            (Range { start: 20, end: 25 }).join(&Range { start: 10, end: 14 }),
            None
        );
        assert_eq!(
            (Range { start: 20, end: 25 }).join(&Range { start: 10, end: 19 }),
            None
        );
    }

    #[test]
    fn test_parse_input_sample() {
        let (ranges, ingredients) = parse_input(SAMPLE);

        assert_eq!(
            ranges,
            [
                Range { start: 3, end: 5 },
                Range { start: 10, end: 14 },
                Range { start: 16, end: 20 },
                Range { start: 12, end: 18 },
            ]
        );
        assert_eq!(ingredients, [1, 5, 8, 11, 17, 32]);
    }

    #[test]
    fn test_fresh_ingredients_step_1() {
        let (ranges, ingredients) = parse_input(SAMPLE);

        let result = count_fresh_ingredients(&ingredients, &ranges);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_total_fresh_in_ranges_step_2() {
        let (ranges, _) = parse_input(SAMPLE);

        let total_fresh = total_fresh_in_ranges(&ranges);
        assert_eq!(total_fresh, 14);
    }
}
