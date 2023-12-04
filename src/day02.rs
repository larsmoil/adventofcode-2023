use std::str::FromStr;

use crate::problem::Solver;

pub struct Day {}

impl Solver for Day {
    fn pt1(&self, input: &str) -> String {
        solve(input)
            .iter()
            .filter(|game| game.possible())
            .map(|game| game.name)
            .sum::<u32>()
            .to_string()
    }
    fn pt2(&self, input: &str) -> String {
        solve(input)
            .iter()
            .map(Game::minimum)
            .map(|minimums| minimums.0 * minimums.1 * minimums.2)
            .sum::<u32>()
            .to_string()
    }
}

pub(crate) fn input() -> &'static str {
    include_str!("day02-input.txt").trim()
}

struct Game {
    name: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn possible(&self) -> bool {
        self.rounds.iter().all(Round::possible)
    }
    fn minimum(&self) -> Round {
        self.rounds.iter().fold(Round(0, 0, 0), |acc, e| {
            Round(e.0.max(acc.0), e.1.max(acc.1), e.2.max(acc.2))
        })
    }
}

#[derive(Debug, PartialEq)]
struct Round(u32, u32, u32);

impl FromStr for Round {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.split(',').fold(Round(0, 0, 0), |acc, e| {
            let (num, color) = e.trim().split_once(' ').unwrap();
            let num: u32 = num.parse().unwrap();
            match color {
                "red" => acc.add_red(num),
                "green" => acc.add_green(num),
                "blue" => acc.add_blue(num),
                &_ => panic!("Unknown color {color}"),
            }
        }))
    }
}

impl Round {
    fn add_red(&self, red: u32) -> Self {
        Round(self.0 + red, self.1, self.2)
    }
    fn add_green(&self, green: u32) -> Self {
        Round(self.0, self.1 + green, self.2)
    }
    fn add_blue(&self, blue: u32) -> Self {
        Round(self.0, self.1, self.2 + blue)
    }
    fn possible(&self) -> bool {
        self.0 <= 12 && self.1 <= 13 && self.2 <= 14
    }
}

fn solve(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let (name, rounds) = line.split_once(": ").unwrap();
            let rounds = rounds.split("; ").map(|r| r.parse::<Round>().unwrap());
            Game {
                name: name.replace("Game ", "").parse::<u32>().unwrap(),
                rounds: rounds.collect(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static str {
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
    }

    #[test]
    fn test_round() {
        assert_eq!(Round(4, 0, 3), " 3 blue, 4 red".parse().unwrap())
    }

    #[test]
    fn test_pt1_example() {
        assert_eq!("8".to_string(), Day {}.pt1(example_input()))
    }

    #[test]
    fn test_pt1() {
        assert_eq!("2545".to_string(), Day {}.pt1(input()))
    }

    #[test]
    fn test_pt2_example() {
        assert_eq!("2286".to_string(), Day {}.pt2(example_input()))
    }

    #[test]
    fn test_pt2() {
        assert_eq!("78111".to_string(), Day {}.pt2(input()))
    }
}
