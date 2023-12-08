use std::{cmp::Ordering, str::FromStr};

use crate::problem::Solver;

pub struct Day {}

#[derive(Debug, PartialEq)]
struct Hands(Vec<Hand>);

#[derive(Clone, Debug, PartialEq)]
struct Hand(String, u64);

#[derive(Debug, PartialEq)]
enum Type {
    FiveOfKind = 6,
    FourOfKind = 5,
    FullHouse = 4,
    ThreeOfKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

impl Solver for Day {
    fn pt1(&self, input: &str) -> String {
        format!(
            "{}",
            input
                .parse::<Hands>()
                .unwrap()
                .ranked(false)
                .iter()
                .enumerate()
                .map(|(i, hand)| (i as u64 + 1) * hand.1)
                .sum::<u64>()
        )
    }
    fn pt2(&self, input: &str) -> String {
        format!(
            "{}",
            input
                .parse::<Hands>()
                .unwrap()
                .ranked(true)
                .iter()
                .enumerate()
                .map(|(i, hand)| (i as u64 + 1) * hand.1)
                .sum::<u64>()
        )
    }
}

const CARDS: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];
const CARDS_WITH_JOKERS: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];
const JOKER: char = 'J';

impl Hands {
    fn ranked(&self, with_jokers: bool) -> Vec<Hand> {
        let cards = if with_jokers {
            CARDS_WITH_JOKERS
        } else {
            CARDS
        };
        let mut ranked = self.0.clone();
        ranked.sort_by(|a_card, b_card| {
            let by_hand_type =
                (a_card.hand_type(with_jokers) as u64).cmp(&(b_card.hand_type(with_jokers) as u64));
            if by_hand_type == Ordering::Equal {
                for (a, b) in a_card.0.chars().zip(b_card.0.chars()) {
                    let a_strength = cards
                        .into_iter()
                        .enumerate()
                        .find(|(_, c)| *c == a)
                        .unwrap()
                        .0;
                    let b_strength = cards
                        .into_iter()
                        .enumerate()
                        .find(|(_, c)| *c == b)
                        .unwrap()
                        .0;
                    match a_strength.cmp(&b_strength) {
                        Ordering::Less => return Ordering::Greater,
                        Ordering::Equal => {}
                        Ordering::Greater => return Ordering::Less,
                    }
                }
                panic!("Couldn't sort hands");
            } else {
                by_hand_type
            }
        });
        ranked
    }
}

impl Hand {
    fn hand_type(&self, with_jokers: bool) -> Type {
        let jokers = if with_jokers {
            self.0.matches(JOKER).count()
        } else {
            0
        };
        let mut num_cards: Vec<usize> = CARDS
            .into_iter()
            .filter(|c| !with_jokers || *c != JOKER)
            .map(|c| self.0.matches(c).count())
            .filter(|&n| n > 0)
            .collect();
        num_cards.sort_unstable();
        num_cards.reverse();
        let first = num_cards.first().unwrap_or(&0);
        match first + jokers {
            5 => Type::FiveOfKind,
            4 => Type::FourOfKind,
            3 => match num_cards[1..].first().unwrap() {
                2 => Type::FullHouse,
                _ => Type::ThreeOfKind,
            },
            2 => match num_cards[1..].first().unwrap() {
                2 => Type::TwoPair,
                _ => Type::OnePair,
            },
            1 => Type::HighCard,
            _ => panic!("Unhandled occurence: {first}"),
        }
    }
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s.split_once(' ').unwrap();
        let bid: u64 = bid.parse().unwrap();
        Ok(Hand(String::from(hand), bid))
    }
}

impl FromStr for Hands {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Hands(
            s.lines()
                .map(|line| line.parse::<Hand>().unwrap())
                .collect(),
        ))
    }
}

pub(crate) fn input() -> &'static str {
    include_str!("day07-input.txt").trim()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static str {
        "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
    }

    #[test]
    fn test_hand_hand_type_without_jokers() {
        let hands: Hands = example_input().parse().unwrap();
        let actual: Vec<Type> = hands
            .0
            .into_iter()
            .map(|hand| hand.hand_type(false))
            .collect();
        let expected: Vec<Type> = vec![
            Type::OnePair,
            Type::ThreeOfKind,
            Type::TwoPair,
            Type::TwoPair,
            Type::ThreeOfKind,
        ];
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_hand_hand_type_with_jokers() {
        let hands: Hands = example_input().parse().unwrap();
        let actual: Vec<Type> = hands
            .0
            .into_iter()
            .map(|hand| hand.hand_type(true))
            .collect();
        let expected: Vec<Type> = vec![
            Type::OnePair,
            Type::FourOfKind,
            Type::TwoPair,
            Type::FourOfKind,
            Type::FourOfKind,
        ];
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_hands_ranked_without_jokers() {
        let hands: Hands = example_input().parse().unwrap();
        let actual: Vec<(String, Type, u64)> = hands
            .ranked(false)
            .iter()
            .map(|hand| (hand.0.clone(), hand.hand_type(false), hand.1))
            .collect();
        let expected: Vec<(String, Type, u64)> = vec![
            (String::from("32T3K"), Type::OnePair, 765),
            (String::from("KTJJT"), Type::TwoPair, 220),
            (String::from("KK677"), Type::TwoPair, 28),
            (String::from("T55J5"), Type::ThreeOfKind, 684),
            (String::from("QQQJA"), Type::ThreeOfKind, 483),
        ];
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_hands_ranked_with_jokers() {
        let hands: Hands = example_input().parse().unwrap();
        let actual: Vec<(String, Type, u64)> = hands
            .ranked(true)
            .iter()
            .map(|hand| (hand.0.clone(), hand.hand_type(true), hand.1))
            .collect();
        let expected: Vec<(String, Type, u64)> = vec![
            (String::from("32T3K"), Type::OnePair, 765),
            (String::from("KK677"), Type::TwoPair, 28),
            (String::from("T55J5"), Type::FourOfKind, 684),
            (String::from("QQQJA"), Type::FourOfKind, 483),
            (String::from("KTJJT"), Type::FourOfKind, 220),
        ];
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_pt1_example() {
        assert_eq!("6440".to_string(), Day {}.pt1(example_input()))
    }

    #[test]
    fn test_pt1() {
        assert_eq!("249390788".to_string(), Day {}.pt1(input()))
    }

    #[test]
    fn test_pt2_example() {
        assert_eq!("5905".to_string(), Day {}.pt2(example_input()))
    }

    #[test]
    fn test_pt2() {
        assert_eq!("248750248".to_string(), Day {}.pt2(input()))
    }
}
