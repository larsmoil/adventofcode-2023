use std::str::FromStr;

use crate::problem::Solver;

pub struct Day {}

impl Solver for Day {
    fn pt1(&self, input: &str) -> String {
        format!("{}", input.parse::<Races>().unwrap().margin_of_error())
    }
    fn pt2(&self, input: &str) -> String {
        format!(
            "{}",
            input
                .parse::<Race>()
                .unwrap()
                .winning_button_presses()
                .len()
        )
    }
}

pub(crate) fn input() -> &'static str {
    include_str!("day06-input.txt").trim()
}

#[derive(Debug, PartialEq)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn winning_button_presses(&self) -> Vec<u64> {
        (1..self.time)
            .map(|time_pushed| {
                let time_remaining = self.time - time_pushed;
                let speed = time_pushed;
                let distance = speed * time_remaining;
                (distance, time_pushed)
            })
            .filter(|&(distance, _time_pushed)| distance > self.distance)
            .map(|(_, time_pushed)| time_pushed)
            .collect()
    }
}

#[derive(Debug, PartialEq)]
struct Races(Vec<Race>);

impl Races {
    fn margin_of_error(&self) -> usize {
        self.0
            .iter()
            .map(|race| race.winning_button_presses().len())
            .reduce(|acc, e| acc * e)
            .unwrap()
    }
}

impl FromStr for Races {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s
            .lines()
            .map(|line| {
                let values = line
                    .split_ascii_whitespace()
                    .skip(1)
                    .map(|v| v.parse::<u64>().unwrap())
                    .collect();
                values
            })
            .collect::<Vec<Vec<u64>>>();
        let (times, distances) = (lines[0].clone(), lines[1].clone());
        if times.len() != distances.len() || times.is_empty() {
            Err(String::from("Could not parse!"))
        } else {
            Ok(Races(
                times
                    .into_iter()
                    .zip(distances)
                    .map(|(time, distance)| Race { time, distance })
                    .collect(),
            ))
        }
    }
}

impl FromStr for Race {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<u64> = s
            .lines()
            .map(|line| line.replace(|c: char| !c.is_numeric(), ""))
            .map(|line| line.parse().unwrap())
            .collect();
        let time = numbers[0];
        let distance = numbers[1];
        Ok(Race { time, distance })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static str {
        "Time:      7  15   30
Distance:  9  40  200"
    }

    #[test]
    fn test_races_from_str() {
        assert_eq!(
            Races(vec![
                Race {
                    time: 7,
                    distance: 9
                },
                Race {
                    time: 15,
                    distance: 40
                },
                Race {
                    time: 30,
                    distance: 200
                },
            ]),
            example_input().parse().unwrap()
        )
    }

    #[test]
    fn test_race_from_str() {
        assert_eq!(
            Race {
                time: 71530,
                distance: 940200,
            },
            example_input().parse().unwrap()
        )
    }

    #[test]
    fn test_race_fwinning_button_presses() {
        assert_eq!(
            vec![2, 3, 4, 5],
            Race {
                time: 7,
                distance: 9
            }
            .winning_button_presses()
        )
    }

    #[test]
    fn test_pt1_example() {
        assert_eq!("288".to_string(), Day {}.pt1(example_input()))
    }

    #[test]
    fn test_pt1() {
        assert_eq!("861300".to_string(), Day {}.pt1(input()))
    }

    #[test]
    fn test_pt2_example() {
        assert_eq!("71503".to_string(), Day {}.pt2(example_input()))
    }

    #[test]
    fn test_pt2() {
        assert_eq!("28101347".to_string(), Day {}.pt2(input()))
    }
}
