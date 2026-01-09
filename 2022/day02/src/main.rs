use std::fs;
use std::io::Error;
use std::path::Path;

#[derive(Debug, PartialEq)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq)]
enum RPSResult {
    Won,
    Lost,
    Tie,
}

impl RPSResult {
    fn score(&self) -> usize {
        match self {
            RPSResult::Lost => 0,
            RPSResult::Tie => 3,
            RPSResult::Won => 6,
        }
    }
}

impl std::str::FromStr for Play {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(Error::other("Invalid play")),
        }
    }
}

impl std::str::FromStr for RPSResult {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lost),
            "Y" => Ok(Self::Tie),
            "Z" => Ok(Self::Won),
            _ => Err(Error::other("Unknown expectation")),
        }
    }
}

impl Play {
    fn result_against(&self, other: &Self) -> RPSResult {
        match (self, other) {
            (Play::Rock, Play::Rock) => RPSResult::Tie,
            (Play::Rock, Play::Paper) => RPSResult::Lost,
            (Play::Paper, Play::Rock) => RPSResult::Won,
            (Play::Paper, Play::Paper) => RPSResult::Tie,
            (Play::Scissors, Play::Scissors) => RPSResult::Tie,
            (Play::Scissors, Play::Rock) => RPSResult::Lost,
            (Play::Rock, Play::Scissors) => RPSResult::Won,
            (Play::Paper, Play::Scissors) => RPSResult::Lost,
            (Play::Scissors, Play::Paper) => RPSResult::Won,
        }
    }

    fn score(&self) -> usize {
        match self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }

    fn need_to(&self, result: &RPSResult) -> Self {
        match result {
            RPSResult::Lost => match self {
                Play::Rock => Play::Scissors,
                Play::Paper => Play::Rock,
                Play::Scissors => Play::Paper,
            },
            RPSResult::Tie => match self {
                Play::Rock => Play::Rock,
                Play::Paper => Play::Paper,
                Play::Scissors => Play::Scissors,
            },
            RPSResult::Won => match self {
                Play::Rock => Play::Paper,
                Play::Paper => Play::Scissors,
                Play::Scissors => Play::Rock,
            },
        }
    }
}

fn main() {
    let input = Path::new("2022/day02/src/input.txt");
    let rounds = parse_strategies(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!("Total score: {:?}", total_score(&rounds));
    println!("In {:?}", part1.elapsed());

    let rounds = parse_strategies_with_result(&fs::read_to_string(input).unwrap());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Total score, after following expected results: {:?}",
        total_expected(&rounds)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_strategies(input: &str) -> Vec<(Play, Play)> {
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(" ").unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect()
}

fn parse_strategies_with_result(input: &str) -> Vec<(Play, RPSResult)> {
    input
        .lines()
        .map(|line| {
            let (play, expected) = line.split_once(" ").unwrap();
            (play.parse().unwrap(), expected.parse().unwrap())
        })
        .collect()
}

fn score_round(round: &(Play, Play)) -> usize {
    let (elfs, yours) = round;
    yours.score() + yours.result_against(elfs).score()
}

fn total_score(rounds: &[(Play, Play)]) -> usize {
    rounds.iter().map(score_round).sum()
}

fn score_expected(round: &(Play, RPSResult)) -> usize {
    let (elfs, expected_result) = round;
    expected_result.score() + elfs.need_to(expected_result).score()
}

fn total_expected(rounds: &[(Play, RPSResult)]) -> usize {
    rounds.iter().map(score_expected).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "A Y
B X
C Z";

    #[test]
    fn test_parse_strategies() {
        let strategies = parse_strategies(SAMPLE);

        assert_eq!(
            strategies,
            [
                (Play::Rock, Play::Paper),
                (Play::Paper, Play::Rock),
                (Play::Scissors, Play::Scissors)
            ]
        );
    }

    #[test]
    fn test_parse_strategies_with_result() {
        let strategies = parse_strategies_with_result(SAMPLE);

        assert_eq!(
            strategies,
            [
                (Play::Rock, RPSResult::Tie),
                (Play::Paper, RPSResult::Lost),
                (Play::Scissors, RPSResult::Won),
            ]
        );
    }

    #[test]
    fn test_score_round() {
        let strategies = parse_strategies(SAMPLE);

        assert_eq!(score_round(&strategies[0]), 8);
        assert_eq!(score_round(&strategies[1]), 1);
        assert_eq!(score_round(&strategies[2]), 6);
    }

    #[test]
    fn test_total_score() {
        let strategies = parse_strategies(SAMPLE);

        assert_eq!(total_score(&strategies), 15);
    }

    #[test]
    fn test_expected_round() {
        let strategies = parse_strategies_with_result(SAMPLE);

        assert_eq!(score_expected(&strategies[0]), 4);
        assert_eq!(score_expected(&strategies[1]), 1);
        assert_eq!(score_expected(&strategies[2]), 7);
    }

    #[test]
    fn test_total_expected() {
        let strategies = parse_strategies_with_result(SAMPLE);

        assert_eq!(total_expected(&strategies), 12);
    }
}
