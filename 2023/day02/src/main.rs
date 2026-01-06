use std::collections::HashMap;
use std::fs;
use std::path::Path;

type Game<'a> = Vec<HashMap<&'a str, usize>>;

fn main() {
    let input = Path::new("2023/day02/src/input.txt");
    let contents = fs::read_to_string(input).unwrap();
    let games = parse_games(&contents);

    let part1 = std::time::Instant::now();
    let available_cubes = HashMap::from_iter([("red", 12), ("green", 13), ("blue", 14)]);
    println!("Part 1");
    println!(
        "Sum of IDs of possible games: {:?}",
        sum_possible_ids(&games, &available_cubes)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Sum of the game's power sets: {:?}",
        sum_power_of_game_sets(&games)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_games(input: &str) -> HashMap<usize, Game<'_>> {
    let mut games: HashMap<usize, Game> = HashMap::new();

    for line in input.lines() {
        let (id_part, rest) = line.split_once(": ").unwrap();
        let (_, id) = id_part.split_once(" ").unwrap();
        let sets: Vec<HashMap<&str, usize>> = rest
            .split("; ")
            .map(|set| {
                let mut cubes_in_set = HashMap::new();
                set.split(", ").for_each(|cube| {
                    let (count, color) = cube.split_once(" ").unwrap();
                    cubes_in_set.insert(color, count.parse().unwrap());
                });
                cubes_in_set
            })
            .collect();
        games.insert(id.parse().unwrap(), sets);
    }
    games
}

fn sum_possible_ids(games: &HashMap<usize, Game>, available_cubes: &HashMap<&str, usize>) -> usize {
    games
        .iter()
        .filter_map(|(id, game)| {
            if game.iter().all(|set| {
                set.iter().all(|(color, count)| {
                    available_cubes
                        .get(color)
                        .is_some_and(|value| value >= count)
                })
            }) {
                Some(id)
            } else {
                None
            }
        })
        .sum()
}

fn minimum_cubes_to_play<'a>(game: &'a Game) -> HashMap<&'a str, usize> {
    let mut minimums = HashMap::new();
    for set in game {
        for (color, count) in set {
            minimums
                .entry(*color)
                .and_modify(|minimum: &mut usize| *minimum = (*minimum).max(*count))
                .or_insert(*count);
        }
    }
    minimums
}

fn sum_power_of_game_sets(games: &HashMap<usize, Game>) -> usize {
    games
        .values()
        .map(|game| minimum_cubes_to_play(game).values().product::<usize>())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_count_possible_ids() {
        let games = parse_games(SAMPLE);

        let available_cubes = HashMap::from_iter([("red", 12), ("green", 13), ("blue", 14)]);
        let sum = sum_possible_ids(&games, &available_cubes);
        assert_eq!(sum, 8);
    }

    #[test]
    fn test_minimum_cubes_to_play() {
        let games = parse_games(SAMPLE);

        assert_eq!(
            minimum_cubes_to_play(&games[&1]),
            HashMap::from_iter([("red", 4), ("green", 2), ("blue", 6)])
        );
        assert_eq!(
            minimum_cubes_to_play(&games[&2]),
            HashMap::from_iter([("red", 1), ("green", 3), ("blue", 4)])
        );
        assert_eq!(
            minimum_cubes_to_play(&games[&3]),
            HashMap::from_iter([("red", 20), ("green", 13), ("blue", 6)])
        );
        assert_eq!(
            minimum_cubes_to_play(&games[&4]),
            HashMap::from_iter([("red", 14), ("green", 3), ("blue", 15)])
        );
        assert_eq!(
            minimum_cubes_to_play(&games[&5]),
            HashMap::from_iter([("red", 6), ("green", 3), ("blue", 2)])
        );
    }

    #[test]
    fn test_sum_of_power_sets() {
        let games = parse_games(SAMPLE);

        assert_eq!(sum_power_of_game_sets(&games), 2286);
    }
}
