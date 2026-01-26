use std::fs;
use std::path::Path;

type Pattern = Vec<char>;

fn main() {
    let input = Path::new("2021/day08/src/input.txt");
    let contents = fs::read_to_string(input).unwrap();
    let signals = parse_input(&contents);

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Number of 1, 4, 7, and 8 in output: {:?}",
        count_basic_digits(&signals)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Sum of the output values: {:?}",
        sum_of_decoded_values(&signals)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_input(input: &str) -> Vec<(Vec<Pattern>, Vec<Pattern>)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(" | ").map(|part| {
                part.split_ascii_whitespace()
                    .map(|s| {
                        let mut pattern: Vec<_> = s.chars().collect();
                        pattern.sort();
                        pattern
                    })
                    .collect()
            });

            (parts.next().unwrap(), parts.next().unwrap())
        })
        .collect()
}

fn count_basic_digits(signals: &[(Vec<Pattern>, Vec<Pattern>)]) -> usize {
    signals
        .iter()
        .map(|(_, output)| {
            output
                .iter()
                .map(|pattern| match pattern.len() {
                    2 | 3 | 4 | 7 => 1,
                    _ => 0,
                })
                .sum::<usize>()
        })
        .sum()
}

fn sum_of_decoded_values(signals: &[(Vec<Pattern>, Vec<Pattern>)]) -> usize {
    signals.iter().map(output_value).sum()
}

fn output_value(signal: &(Vec<Pattern>, Vec<Pattern>)) -> usize {
    let mut numbers = vec![vec![]; 10];
    let (patterns, output) = signal;

    let mut length_to_patterns = vec![vec![]; 8];

    for pattern in patterns {
        length_to_patterns[pattern.len()].push(pattern.clone());
    }

    for length in [2, 3, 4, 7] {
        let pattern = length_to_patterns[length].pop().unwrap();
        match pattern.len() {
            2 => {
                numbers[1] = pattern;
            }
            3 => {
                numbers[7] = pattern;
            }
            4 => {
                numbers[4] = pattern;
            }
            7 => {
                numbers[8] = pattern;
            }
            _ => {}
        }
    }

    for pattern in length_to_patterns[5].drain(..) {
        if numbers[1].iter().all(|c| pattern.contains(c)) {
            numbers[3] = pattern;
        } else if numbers[4].iter().filter(|c| pattern.contains(c)).count() == 3 {
            numbers[5] = pattern;
        } else {
            numbers[2] = pattern;
        }
    }
    for pattern in length_to_patterns[6].drain(..) {
        if numbers[1].iter().filter(|c| pattern.contains(c)).count() == 1 {
            numbers[6] = pattern;
        } else if numbers[5].iter().all(|c| pattern.contains(c)) {
            numbers[9] = pattern;
        } else {
            numbers[0] = pattern;
        }
    }

    output
        .iter()
        .rev()
        .enumerate()
        .map(|(index, pattern)| {
            let number = numbers
                .iter()
                .enumerate()
                .find_map(|(digit, p)| if p == pattern { Some(digit) } else { None })
                .unwrap();
            number * 10_usize.pow(index as u32)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
    const SAMPLE2: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_part_1_sample() {
        let signals1 = parse_input(SAMPLE1);

        assert_eq!(count_basic_digits(&signals1), 0);

        let signals2 = parse_input(SAMPLE2);

        assert_eq!(count_basic_digits(&signals2), 26);
    }

    #[test]
    fn test_part_2_sample() {
        let signals1 = parse_input(SAMPLE1);

        assert_eq!(sum_of_decoded_values(&signals1), 5353);

        let signals2 = parse_input(SAMPLE2);

        assert_eq!(sum_of_decoded_values(&signals2), 61229);
    }
}
