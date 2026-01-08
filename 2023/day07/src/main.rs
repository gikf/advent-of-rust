use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Num(u8),
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Jack,
            'T' => Self::Num(10),
            num @ '2'..='9' => Self::Num(num.to_digit(10).unwrap() as u8),
            _ => unimplemented!(),
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Ace, Self::Ace) => Some(Ordering::Equal),
            (Self::Ace, _) => Some(Ordering::Greater),
            (_, Self::Ace) => Some(Ordering::Less),
            (Self::King, Self::King) => Some(Ordering::Equal),
            (Self::King, _) => Some(Ordering::Greater),
            (_, Self::King) => Some(Ordering::Less),
            (Self::Queen, Self::Queen) => Some(Ordering::Equal),
            (Self::Queen, _) => Some(Ordering::Greater),
            (_, Self::Queen) => Some(Ordering::Less),
            (Self::Jack, Self::Jack) => Some(Ordering::Equal),
            (Self::Jack, _) => Some(Ordering::Greater),
            (_, Self::Jack) => Some(Ordering::Less),
            (Self::Num(card1), Self::Num(card2)) => Some(card1.cmp(card2)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum CardWithJoker {
    Ace,
    King,
    Queen,
    Joker,
    Num(u8),
}

impl From<char> for CardWithJoker {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Joker,
            'T' => Self::Num(10),
            num @ '2'..='9' => Self::Num(num.to_digit(10).unwrap() as u8),
            _ => unimplemented!(),
        }
    }
}

impl PartialOrd for CardWithJoker {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Ace, Self::Ace) => Some(Ordering::Equal),
            (Self::Ace, _) => Some(Ordering::Greater),
            (_, Self::Ace) => Some(Ordering::Less),
            (Self::King, Self::King) => Some(Ordering::Equal),
            (Self::King, _) => Some(Ordering::Greater),
            (_, Self::King) => Some(Ordering::Less),
            (Self::Queen, Self::Queen) => Some(Ordering::Equal),
            (Self::Queen, _) => Some(Ordering::Greater),
            (_, Self::Queen) => Some(Ordering::Less),
            (Self::Num(card1), Self::Num(card2)) => Some(card1.cmp(card2)),
            (Self::Num(_), _) => Some(Ordering::Greater),
            (_, Self::Num(_)) => Some(Ordering::Less),
            (Self::Joker, Self::Joker) => Some(Ordering::Equal),
        }
    }
}

#[derive(Debug, PartialEq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, PartialEq)]
struct Hand {
    cards: Vec<Card>,
    bid: usize,
    hand_type: HandType,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self.hand_type, &other.hand_type) {
            (HandType::FiveOfAKind, HandType::FiveOfAKind) => self._compare_by_cards(other),
            (HandType::FiveOfAKind, _) => Ordering::Greater,
            (_, HandType::FiveOfAKind) => Ordering::Less,
            (HandType::FourOfAKind, HandType::FourOfAKind) => self._compare_by_cards(other),
            (HandType::FourOfAKind, _) => Ordering::Greater,
            (_, HandType::FourOfAKind) => Ordering::Less,
            (HandType::FullHouse, HandType::FullHouse) => self._compare_by_cards(other),
            (HandType::FullHouse, _) => Ordering::Greater,
            (_, HandType::FullHouse) => Ordering::Less,
            (HandType::ThreeOfAKind, HandType::ThreeOfAKind) => self._compare_by_cards(other),
            (HandType::ThreeOfAKind, _) => Ordering::Greater,
            (_, HandType::ThreeOfAKind) => Ordering::Less,
            (HandType::TwoPair, HandType::TwoPair) => self._compare_by_cards(other),
            (HandType::TwoPair, _) => Ordering::Greater,
            (_, HandType::TwoPair) => Ordering::Less,
            (HandType::OnePair, HandType::OnePair) => self._compare_by_cards(other),
            (HandType::OnePair, _) => Ordering::Greater,
            (_, HandType::OnePair) => Ordering::Less,
            (HandType::HighCard, HandType::HighCard) => self._compare_by_cards(other),
        }
    }
}

impl Hand {
    fn _compare_by_cards(&self, other: &Self) -> Ordering {
        for (left, right) in self.cards.iter().zip(other.cards.iter()) {
            if left > right {
                return Ordering::Greater;
            } else if left < right {
                return Ordering::Less;
            }
        }
        Ordering::Equal
    }

    fn _recognize_deck(cards: &[Card]) -> HandType {
        let mut card_to_count = HashMap::new();
        for card in cards {
            card_to_count
                .entry(card)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        match card_to_count.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                let counts: Vec<_> = card_to_count.values().collect();
                match (counts[0], counts[1]) {
                    (4, 1) | (1, 4) => HandType::FourOfAKind,
                    (2, 3) | (3, 2) => HandType::FullHouse,
                    _ => unreachable!(),
                }
            }
            3 => {
                let counts: Vec<_> = card_to_count.values().collect();
                match (counts[0], counts[1], counts[2]) {
                    (1, 1, 3) | (1, 3, 1) | (3, 1, 1) => HandType::ThreeOfAKind,
                    (2, 2, 1) | (2, 1, 2) | (1, 2, 2) => HandType::TwoPair,
                    _ => unreachable!(),
                }
            }
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct HandWithJoker {
    cards: Vec<CardWithJoker>,
    bid: usize,
    hand_type: HandType,
}

impl PartialOrd for HandWithJoker {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for HandWithJoker {}

impl Ord for HandWithJoker {
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self.hand_type, &other.hand_type) {
            (HandType::FiveOfAKind, HandType::FiveOfAKind) => self._compare_by_cards(other),
            (HandType::FiveOfAKind, _) => Ordering::Greater,
            (_, HandType::FiveOfAKind) => Ordering::Less,
            (HandType::FourOfAKind, HandType::FourOfAKind) => self._compare_by_cards(other),
            (HandType::FourOfAKind, _) => Ordering::Greater,
            (_, HandType::FourOfAKind) => Ordering::Less,
            (HandType::FullHouse, HandType::FullHouse) => self._compare_by_cards(other),
            (HandType::FullHouse, _) => Ordering::Greater,
            (_, HandType::FullHouse) => Ordering::Less,
            (HandType::ThreeOfAKind, HandType::ThreeOfAKind) => self._compare_by_cards(other),
            (HandType::ThreeOfAKind, _) => Ordering::Greater,
            (_, HandType::ThreeOfAKind) => Ordering::Less,
            (HandType::TwoPair, HandType::TwoPair) => self._compare_by_cards(other),
            (HandType::TwoPair, _) => Ordering::Greater,
            (_, HandType::TwoPair) => Ordering::Less,
            (HandType::OnePair, HandType::OnePair) => self._compare_by_cards(other),
            (HandType::OnePair, _) => Ordering::Greater,
            (_, HandType::OnePair) => Ordering::Less,
            (HandType::HighCard, HandType::HighCard) => self._compare_by_cards(other),
        }
    }
}

trait NewFromCards<T> {
    fn new(cards: Vec<T>, bid: usize) -> Self;
}

impl NewFromCards<CardWithJoker> for HandWithJoker {
    fn new(cards: Vec<CardWithJoker>, bid: usize) -> Self {
        let hand_type = Self::_recognize_deck(&cards);
        Self {
            cards,
            bid,
            hand_type,
        }
    }
}

impl NewFromCards<Card> for Hand {
    fn new(cards: Vec<Card>, bid: usize) -> Self {
        let hand_type = Self::_recognize_deck(&cards);
        Self {
            cards,
            bid,
            hand_type,
        }
    }
}

trait HasBid {
    fn get_bid(&self) -> usize;
}

impl HasBid for Hand {
    fn get_bid(&self) -> usize {
        self.bid
    }
}

impl HasBid for HandWithJoker {
    fn get_bid(&self) -> usize {
        self.bid
    }
}

impl HandWithJoker {
    fn _compare_by_cards(&self, other: &Self) -> Ordering {
        for (left, right) in self.cards.iter().zip(other.cards.iter()) {
            if left > right {
                return Ordering::Greater;
            } else if left < right {
                return Ordering::Less;
            }
        }
        Ordering::Equal
    }

    fn _recognize_deck(cards: &[CardWithJoker]) -> HandType {
        let mut card_to_count = HashMap::new();
        for card in cards {
            card_to_count
                .entry(card)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        let has_joker = card_to_count.contains_key(&CardWithJoker::Joker);
        match card_to_count.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                let counts: Vec<_> = card_to_count.values().collect();
                match (counts[0], counts[1]) {
                    (4, 1) | (1, 4) => {
                        if has_joker {
                            HandType::FiveOfAKind
                        } else {
                            HandType::FourOfAKind
                        }
                    }
                    (2, 3) | (3, 2) => {
                        if has_joker {
                            HandType::FiveOfAKind
                        } else {
                            HandType::FullHouse
                        }
                    }
                    _ => unimplemented!(),
                }
            }
            3 => {
                let counts: Vec<_> = card_to_count.values().collect();
                match (counts[0], counts[1], counts[2]) {
                    (1, 1, 3) | (1, 3, 1) | (3, 1, 1) => {
                        if has_joker {
                            HandType::FourOfAKind
                        } else {
                            HandType::ThreeOfAKind
                        }
                    }
                    (2, 2, 1) | (2, 1, 2) | (1, 2, 2) => {
                        match card_to_count.get(&CardWithJoker::Joker) {
                            Some(2) => HandType::FourOfAKind,
                            Some(1) => HandType::FullHouse,
                            None => HandType::TwoPair,
                            _ => unimplemented!(),
                        }
                    }
                    _ => unimplemented!(),
                }
            }
            4 => {
                if has_joker {
                    HandType::ThreeOfAKind
                } else {
                    HandType::OnePair
                }
            }
            5 => {
                if has_joker {
                    HandType::OnePair
                } else {
                    HandType::HighCard
                }
            }
            _ => unimplemented!(),
        }
    }
}

fn main() {
    let input = Path::new("2023/day07/src/input.txt");
    let hands = parse_cards(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!("Total winnings: {:?}", total_winnings::<Card, Hand>(&hands));
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    let hands_with_jokers = parse_cards(&fs::read_to_string(input).unwrap());
    println!("Part 2");
    println!(
        "Total winnings with jokers: {:?}",
        total_winnings::<CardWithJoker, HandWithJoker>(&hands_with_jokers)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_cards<CardType: From<char>>(input: &str) -> Vec<(Vec<CardType>, usize)> {
    input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(" ").unwrap();

            let cards: Vec<_> = cards.chars().map(CardType::from).collect();

            (cards, bid.parse().unwrap())
        })
        .collect()
}

fn total_winnings<CardKind: Clone, HandKind: NewFromCards<CardKind> + Ord + HasBid>(
    hands: &[(Vec<CardKind>, usize)],
) -> usize {
    order_by_rank(hands)
        .iter()
        .enumerate()
        .map(|(index, hand): (usize, &HandKind)| hand.get_bid() * (index + 1))
        .sum()
}

fn order_by_rank<CardKind: Clone, HandKind: NewFromCards<CardKind> + Ord>(
    hands: &[(Vec<CardKind>, usize)],
) -> Vec<HandKind> {
    let mut hands: Vec<HandKind> = hands
        .iter()
        .map(|(cards, bid)| HandKind::new(cards.clone(), *bid))
        .collect();
    hands.sort();
    hands
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_parse_hands() {
        let hands = parse_cards(SAMPLE);

        assert_eq!(
            hands[0],
            (
                Vec::from([
                    Card::Num(3),
                    Card::Num(2),
                    Card::Num(10),
                    Card::Num(3),
                    Card::King
                ]),
                765
            )
        );
        assert_eq!(
            hands[1],
            (
                Vec::from([
                    Card::Num(10),
                    Card::Num(5),
                    Card::Num(5),
                    Card::Jack,
                    Card::Num(5),
                ]),
                684
            )
        );
        assert_eq!(
            hands[2],
            (
                Vec::from([
                    Card::King,
                    Card::King,
                    Card::Num(6),
                    Card::Num(7),
                    Card::Num(7),
                ]),
                28
            )
        );
        assert_eq!(
            hands[3],
            (
                Vec::from([
                    Card::King,
                    Card::Num(10),
                    Card::Jack,
                    Card::Jack,
                    Card::Num(10)
                ]),
                220
            )
        );
        assert_eq!(
            hands[4],
            (
                Vec::from([Card::Queen, Card::Queen, Card::Queen, Card::Jack, Card::Ace]),
                483
            )
        );
    }

    #[test]
    fn test_recognize_hand() {
        let hands = parse_cards(SAMPLE);

        let hand1 = Hand::_recognize_deck(&hands[0].0);
        assert_eq!(hand1, HandType::OnePair);
        let hand2 = Hand::_recognize_deck(&hands[1].0);
        assert_eq!(hand2, HandType::ThreeOfAKind);
        let hand3 = Hand::_recognize_deck(&hands[2].0);
        assert_eq!(hand3, HandType::TwoPair);
        let hand4 = Hand::_recognize_deck(&hands[3].0);
        assert_eq!(hand4, HandType::TwoPair);
        let hand5 = Hand::_recognize_deck(&hands[4].0);
        assert_eq!(hand5, HandType::ThreeOfAKind);

        assert_eq!(
            HandType::FiveOfAKind,
            Hand::_recognize_deck(&[
                Card::Num(2),
                Card::Num(2),
                Card::Num(2),
                Card::Num(2),
                Card::Num(2)
            ])
        );
        assert_eq!(
            HandType::FourOfAKind,
            Hand::_recognize_deck(&[
                Card::Num(2),
                Card::Num(2),
                Card::Ace,
                Card::Num(2),
                Card::Num(2)
            ])
        );
        assert_eq!(
            HandType::FullHouse,
            Hand::_recognize_deck(&[
                Card::Num(2),
                Card::Queen,
                Card::Num(2),
                Card::Num(2),
                Card::Queen
            ])
        );
        assert_eq!(
            HandType::HighCard,
            Hand::_recognize_deck(&[
                Card::Num(2),
                Card::Num(3),
                Card::Num(6),
                Card::Num(7),
                Card::Num(10)
            ])
        );
    }

    #[test]
    fn test_total_winnings() {
        let hands: Vec<(Vec<Card>, usize)> = parse_cards(SAMPLE);

        let winnings = total_winnings::<Card, Hand>(&hands);
        assert_eq!(winnings, 6440);
    }

    #[test]
    fn test_total_winnings_with_jokers() {
        let hands: Vec<(Vec<CardWithJoker>, usize)> = parse_cards(SAMPLE);

        let winnings = total_winnings::<CardWithJoker, HandWithJoker>(&hands);
        assert_eq!(winnings, 5905);
    }
}
