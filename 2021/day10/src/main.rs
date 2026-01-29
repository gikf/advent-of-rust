use std::fs;
use std::path::Path;

enum Line {
    Valid,
    Corrupt(char),
    Incomplete(Vec<char>),
}

fn main() {
    let input = Path::new("2021/day10/src/input.txt");
    let contents = fs::read_to_string(input).unwrap();
    let lines = parse_lines(&contents);

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Total syntax error score for corrupted lines: {:?}",
        syntax_error_score(&lines)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Middle score of the incomplete lines: {:?}",
        incomplete_middle_score(&lines)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_lines(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn score_corrupt(c: &char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn score_incomplete(brackets: &[char]) -> usize {
    brackets.iter().rev().fold(0, |acc, bracket| {
        acc * 5
            + match bracket {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => unimplemented!(),
            }
    })
}

fn validate_line(line: &str) -> Line {
    let mut stack = Vec::new();
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => {
                stack.push(c);
            }
            ')' | ']' | '}' | '>' => match (stack.pop().unwrap(), c) {
                ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => {}
                (_, _) => return Line::Corrupt(c),
            },
            _ => unimplemented!(),
        }
    }
    if !stack.is_empty() {
        Line::Incomplete(stack)
    } else {
        Line::Valid
    }
}

fn syntax_error_score(lines: &[&str]) -> usize {
    lines
        .iter()
        .map(|line| match validate_line(line) {
            Line::Corrupt(c) => score_corrupt(&c),
            _ => 0,
        })
        .sum()
}

fn incomplete_middle_score(lines: &[&str]) -> usize {
    let mut scores: Vec<_> = lines
        .iter()
        .filter_map(|line| match validate_line(line) {
            Line::Incomplete(brackets) => Some(score_incomplete(&brackets)),
            _ => None,
        })
        .collect();
    scores.sort();

    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_score_incomplete() {
        assert_eq!(
            score_incomplete(&Vec::from_iter("[({([[{{".chars())),
            288957
        );
        assert_eq!(
            score_incomplete(&Vec::from_iter("({<[{(".chars().rev())),
            5566
        );
        assert_eq!(
            score_incomplete(&Vec::from_iter("{{<{<((((".chars().rev())),
            1480781
        );
        assert_eq!(
            score_incomplete(&Vec::from_iter("[[{{[{[{<".chars().rev())),
            995444
        );
        assert_eq!(score_incomplete(&Vec::from_iter("<{([".chars())), 294);
    }

    #[test]
    fn test_part_1_sample() {
        let lines = parse_lines(SAMPLE);

        assert_eq!(syntax_error_score(&lines), 26397);
    }

    #[test]
    fn test_part_2_sample() {
        let lines = parse_lines(SAMPLE);

        assert_eq!(incomplete_middle_score(&lines), 288957)
    }
}
