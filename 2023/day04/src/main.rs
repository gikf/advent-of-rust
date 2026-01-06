use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

type Card = (usize, HashSet<usize>, HashSet<usize>);

fn main() {
    let input = Path::new("2023/day04/src/input.txt");
    let cards = parse_cards(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!("Total score of cards: {:?}", score_cards(&cards));
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!("Total cards won: {:?}", cards_won(&cards));
    println!("In {:?}", part2.elapsed());
}

fn parse_cards(input: &str) -> Vec<(usize, HashSet<usize>, HashSet<usize>)> {
    input
        .lines()
        .map(|line| {
            let (id_part, rest) = line.split_once(": ").unwrap();
            let (winning, owned) = rest.split_once(" | ").unwrap();

            let id = id_part.split_ascii_whitespace().last().unwrap();
            let winning_numbers = HashSet::from_iter(
                winning
                    .split_ascii_whitespace()
                    .map(|number| number.parse().unwrap()),
            );
            let numbers_scratched = HashSet::from_iter(
                owned
                    .split_ascii_whitespace()
                    .map(|number| number.parse().unwrap()),
            );

            (id.parse().unwrap(), winning_numbers, numbers_scratched)
        })
        .collect()
}

fn score_cards(cards: &[Card]) -> usize {
    cards
        .iter()
        .map(|(_, winning, scratched)| {
            let matching = matching_numbers_on_card(winning, scratched);
            if matching != 0 {
                2_usize.pow(matching as u32 - 1)
            } else {
                0
            }
        })
        .sum()
}

fn matching_numbers_on_card(winning: &HashSet<usize>, scratched: &HashSet<usize>) -> usize {
    winning
        .iter()
        .filter(|number| scratched.contains(*number))
        .count()
}

fn cards_won(cards: &[Card]) -> usize {
    let mut card_with_matching_numbers = Vec::new();
    cards.iter().for_each(|(id, winning, scratched)| {
        card_with_matching_numbers.push((*id, matching_numbers_on_card(winning, scratched)));
    });

    let mut card_copies = HashMap::new();
    card_with_matching_numbers
        .iter()
        .for_each(|(id, matching_numbers)| {
            let copies_of_winning_card = card_copies.entry(*id).or_insert(1);
            let won_of_next_card = *copies_of_winning_card;
            ((*id + 1)..=(*id + *matching_numbers)).for_each(|next_card_id| {
                card_copies
                    .entry(next_card_id)
                    .and_modify(|count| *count += won_of_next_card)
                    .or_insert(1 + won_of_next_card);
            })
        });
    card_copies.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_parse_cards() {
        let cards = parse_cards(SAMPLE);

        assert_eq!(
            cards[0],
            (
                1,
                HashSet::from([41, 48, 83, 86, 17]),
                HashSet::from([83, 86, 6, 31, 17, 9, 48, 53])
            )
        );
        assert_eq!(
            cards[1],
            (
                2,
                HashSet::from([13, 32, 20, 16, 61]),
                HashSet::from([61, 30, 68, 82, 17, 32, 24, 19])
            )
        );
        assert_eq!(
            cards[2],
            (
                3,
                HashSet::from([1, 21, 53, 59, 44]),
                HashSet::from([69, 82, 63, 72, 16, 21, 14, 1])
            )
        );
        assert_eq!(
            cards[3],
            (
                4,
                HashSet::from([41, 92, 73, 84, 69]),
                HashSet::from([59, 84, 76, 51, 58, 5, 54, 83])
            )
        );
        assert_eq!(
            cards[4],
            (
                5,
                HashSet::from([87, 83, 26, 28, 32]),
                HashSet::from([88, 30, 70, 12, 93, 22, 82, 36])
            )
        );
        assert_eq!(
            cards[5],
            (
                6,
                HashSet::from([31, 18, 13, 56, 72]),
                HashSet::from([74, 77, 10, 23, 35, 67, 36, 11])
            )
        );
    }

    #[test]
    fn test_matching_numbers() {
        let cards = parse_cards(SAMPLE);

        assert_eq!(matching_numbers_on_card(&cards[0].1, &cards[0].2), 4);
        assert_eq!(matching_numbers_on_card(&cards[1].1, &cards[1].2), 2);
        assert_eq!(matching_numbers_on_card(&cards[2].1, &cards[2].2), 2);
        assert_eq!(matching_numbers_on_card(&cards[3].1, &cards[3].2), 1);
        assert_eq!(matching_numbers_on_card(&cards[4].1, &cards[4].2), 0);
        assert_eq!(matching_numbers_on_card(&cards[5].1, &cards[5].2), 0);
    }

    #[test]
    fn test_score_cards() {
        let cards = parse_cards(SAMPLE);

        assert_eq!(score_cards(&cards), 13);
    }

    #[test]
    fn test_cards_won() {
        let cards = parse_cards(SAMPLE);

        assert_eq!(cards_won(&cards), 30);
    }
}
