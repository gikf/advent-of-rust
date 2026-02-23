use std::collections::HashMap;
use std::fs;
use std::path::Path;

const PART1_LIMIT: usize = 1000;
const PART2_LIMIT: usize = 21;
const ADDONS: [(usize, usize); 7] = [(1, 3), (3, 4), (6, 5), (7, 6), (6, 7), (3, 8), (1, 9)];

fn main() {
    let input = Path::new("2021/day21/src/input.txt");
    let players = parse_players(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Score of losing player multiplied by number of rolls: {:?}",
        play_until(players, PART1_LIMIT)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Most number of universes, in which one of the players won: {:?}",
        play_dirac(players, PART2_LIMIT)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_players(input: &str) -> [usize; 2] {
    let mut players = input
        .lines()
        .map(|line| line.split_ascii_whitespace().last().unwrap());
    [
        players.next().unwrap().parse().unwrap(),
        players.next().unwrap().parse().unwrap(),
    ]
}

fn play_until(players: [usize; 2], limit: usize) -> usize {
    let mut dice = (1..=100).cycle();
    let mut rolls = 0;
    let mut positions = players;
    let mut scores = [0, 0];
    let mut cur_player = 0;

    while scores.iter().all(|s| *s < limit) {
        let rolled_total = (0..3).map(|_| dice.next().unwrap()).sum::<usize>();
        rolls += 3;

        let next_position = positions[cur_player] + rolled_total;
        positions[cur_player] = if next_position.is_multiple_of(10) {
            10
        } else {
            next_position % 10
        };

        scores[cur_player] += positions[cur_player];
        cur_player = (cur_player + 1) % 2;
    }

    rolls * (*scores.iter().min().unwrap())
}

fn play_dirac(players: [usize; 2], limit: usize) -> usize {
    let [wins1, wins2] = dirac(players, [0, 0], limit, &mut HashMap::new());
    wins1.max(wins2)
}

fn dirac(
    positions: [usize; 2],
    scores: [usize; 2],
    limit: usize,
    results: &mut HashMap<[[usize; 2]; 2], [usize; 2]>,
) -> [usize; 2] {
    if let Some(&result) = results.get(&[positions, scores]) {
        return result;
    }

    let mut wins = [0, 0];

    for (count, score) in ADDONS {
        let position = positions[0] + score;
        let new_position = if position.is_multiple_of(10) {
            10
        } else {
            position % 10
        };
        let new_score = scores[0] + new_position;
        if new_score >= limit {
            wins[0] += count;
        } else {
            let [next_wins2, next_wins1] = dirac(
                [positions[1], new_position],
                [scores[1], new_score],
                limit,
                results,
            );
            wins[0] += count * next_wins1;
            wins[1] += count * next_wins2;
        }
    }

    results.insert([positions, scores], wins);
    wins
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Player 1 starting position: 4
Player 2 starting position: 8";

    #[test]
    fn test_part_1_sample() {
        let players = parse_players(SAMPLE);

        assert_eq!(play_until(players, PART1_LIMIT), 739785);
    }

    #[test]
    fn test_part_2_sample() {
        let players = parse_players(SAMPLE);

        assert_eq!(play_dirac(players, PART2_LIMIT), 444356092776315);
    }
}
