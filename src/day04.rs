use crate::problem::Solver;

pub struct Day {}

impl Solver for Day {
    fn pt1(&self, input: &str) -> String {
        format!("{}", Game::from(input).score())
    }
    fn pt2(&self, input: &str) -> String {
        format!("{}", Game::from(input).cards())
    }
}

struct Game<'a>(&'a str);
impl<'a> From<&'a str> for Game<'a> {
    fn from(value: &'a str) -> Self {
        Self(value)
    }
}
impl<'a> Game<'a> {
    fn score(&self) -> u32 {
        self.0
            .lines()
            .map(ScratchCard::from)
            .map(|g| g.score())
            .sum()
    }
    fn cards(&self) -> u32 {
        let mut cards: Vec<(u32, ScratchCard)> = self
            .0
            .lines()
            .map(ScratchCard::from)
            .map(|o| (1, o))
            .collect();
        (0..cards.len()).for_each(|i| {
            let card = &cards[i];
            let score = card.1.matching_numbers();

            for _ in 0..card.0 {
                ((i + 1)..=((i + score as usize).min(cards.len()))).for_each(|j| {
                    cards[j].0 += 1;
                });
            }
        });
        cards.iter().map(|t| t.0).sum::<u32>()
    }
}

#[derive(Debug)]
struct ScratchCard<'a>(&'a str);
impl<'a> From<&'a str> for ScratchCard<'a> {
    fn from(value: &'a str) -> Self {
        Self(value)
    }
}

impl<'a> ScratchCard<'a> {
    fn matching_numbers(&self) -> u32 {
        let (winning, my) = self.numbers();
        let num_matches: u32 = my
            .iter()
            .filter(|my_number| winning.contains(my_number))
            .count()
            .try_into()
            .unwrap();
        num_matches
    }
    fn score(&self) -> u32 {
        match self.matching_numbers() {
            0 => 0,
            n => 2_u32.pow(n - 1),
        }
    }
    fn numbers(&self) -> (Vec<u32>, Vec<u32>) {
        let values = self.0.split_once(':').unwrap().1.trim();
        let (winning_numbers, my_numbers) = values.split_once('|').unwrap();
        let winning_numbers: Vec<u32> = winning_numbers
            .split_ascii_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        let my_numbers: Vec<u32> = my_numbers
            .split_ascii_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        (winning_numbers, my_numbers)
    }
}

pub(crate) fn input() -> &'static str {
    include_str!("day04-input.txt").trim()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static str {
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
    }

    #[test]
    fn test_pt1_example() {
        assert_eq!("13".to_string(), Day {}.pt1(example_input()))
    }

    #[test]
    fn test_pt1() {
        assert_eq!("22897".to_string(), Day {}.pt1(input()))
    }

    #[test]
    fn test_pt2_example() {
        assert_eq!("30".to_string(), Day {}.pt2(example_input()))
    }

    #[test]
    fn test_pt2() {
        assert_eq!("5095824".to_string(), Day {}.pt2(input()))
    }
}
