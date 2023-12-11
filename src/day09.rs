use std::str::FromStr;

use crate::problem::Solver;

pub struct Day {}

impl Solver for Day {
    fn pt1(&self, input: &str) -> String {
        format!("{}", input.parse::<Oasis>().unwrap().next_history())
    }
    fn pt2(&self, input: &str) -> String {
        format!("{}", input.parse::<Oasis>().unwrap().prev_history())
    }
}

pub(crate) fn input() -> &'static str {
    include_str!("day09-input.txt").trim()
}

#[derive(Debug)]
struct Measurements(Vec<i64>);
impl FromStr for Measurements {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Measurements(
            s.split_ascii_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect(),
        ))
    }
}
impl Measurements {
    fn next_history(&self) -> i64 {
        let next: Vec<i64> = self
            .0
            .windows(2)
            .map(|a| a.last().unwrap() - a.first().unwrap())
            .collect();

        return if next.iter().all(|&n| n == 0) {
            self.0.last().unwrap() + next.last().unwrap()
        } else {
            self.0.last().unwrap() + Measurements(next).next_history()
        };
    }
    fn prev_history(&self) -> i64 {
        let mut reversed = self.0.clone();
        reversed.reverse();
        Measurements(reversed).next_history()
    }
}

struct Oasis(Vec<Measurements>);
impl FromStr for Oasis {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Oasis(s.lines().map(|line| line.parse().unwrap()).collect()))
    }
}

impl Oasis {
    fn prev_history(&self) -> i64 {
        self.0.iter().map(Measurements::prev_history).sum()
    }
    fn next_history(&self) -> i64 {
        self.0.iter().map(Measurements::next_history).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static str {
        "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
    }

    #[test]
    fn test_pt1_example() {
        assert_eq!("114".to_string(), Day {}.pt1(example_input()))
    }

    #[test]
    fn test_pt1() {
        assert_eq!("1666172641".to_string(), Day {}.pt1(input()))
    }

    #[test]
    fn test_pt2_example() {
        assert_eq!("2".to_string(), Day {}.pt2(example_input()))
    }

    #[test]
    fn test_pt2() {
        assert_eq!("933".to_string(), Day {}.pt2(input()))
    }
}
