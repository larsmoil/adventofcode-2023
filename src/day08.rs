use num::integer::lcm;
use std::str::FromStr;

use crate::problem::Solver;

pub struct Day {}

impl Solver for Day {
    fn pt1(&self, input: &str) -> String {
        format!(
            "{}",
            input
                .parse::<Map>()
                .unwrap()
                .path(|name| name == START, |name| name == END)
        )
    }
    fn pt2(&self, input: &str) -> String {
        format!(
            "{}",
            input
                .parse::<Map>()
                .unwrap()
                .path(|name| name.ends_with('A'), |name| name.ends_with('Z'))
        )
    }
}

pub(crate) fn input() -> &'static str {
    include_str!("day08-input.txt").trim()
}

const START: &str = "AAA";
const END: &str = "ZZZ";

impl Map {
    fn path(&self, start: fn(&str) -> bool, end: fn(&str) -> bool) -> usize {
        let instructions: Vec<char> = self.instructions.chars().collect();
        let elements: Vec<&(String, usize, usize)> = self
            .elements
            .iter()
            .filter(|(name, _, _)| start(name))
            .collect();

        elements
            .into_iter()
            .map(|e| {
                let mut element = e;
                for i in 0.. {
                    let instruction = instructions[i % instructions.len()];
                    element = &self.elements[match instruction {
                        'L' => element.1,
                        'R' => element.2,
                        _ => panic!("Unknown instruction: {instruction}"),
                    }];
                    if end(&element.0) {
                        return i + 1;
                    }
                }
                panic!("foo");
            })
            .reduce(lcm)
            .unwrap()
    }
}

#[derive(Debug, PartialEq)]
struct Map {
    instructions: String,
    elements: Vec<(String, usize, usize)>,
}
impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instructions, elements) = s.split_once("\n\n").unwrap();
        let instructions = String::from(instructions);
        let elements: Vec<(String, String, String)> = elements
            .lines()
            .map(|line| {
                let (name, left_right) = line.split_once(" = ").unwrap();
                let left_right = left_right.replace(['(', ')'], "");
                let (left, right) = left_right.split_once(", ").unwrap();
                (String::from(name), String::from(left), String::from(right))
            })
            .collect();
        let elements: Vec<(String, usize, usize)> = elements
            .iter()
            .map(|(name, left, right)| {
                let left = elements
                    .iter()
                    .enumerate()
                    .find(|(_, (c, _, _))| c == left)
                    .unwrap()
                    .0;
                let right = elements
                    .iter()
                    .enumerate()
                    .find(|(_, (c, _, _))| c == right)
                    .unwrap()
                    .0;
                (name.clone(), left, right)
            })
            .collect();

        Ok(Map {
            instructions,
            elements,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input_pt_1_1() -> &'static str {
        "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
    }

    fn example_input_pt_1_2() -> &'static str {
        "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
    }

    fn example_input_pt_2() -> &'static str {
        "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
    }

    #[test]
    fn test_pt1_example_1() {
        assert_eq!("2".to_string(), Day {}.pt1(example_input_pt_1_1()))
    }

    #[test]
    fn test_pt1_example_2() {
        assert_eq!("6".to_string(), Day {}.pt1(example_input_pt_1_2()))
    }

    #[test]
    fn test_pt1() {
        assert_eq!("11309".to_string(), Day {}.pt1(input()))
    }

    #[test]
    fn test_pt2_example() {
        assert_eq!("6".to_string(), Day {}.pt2(example_input_pt_2()))
    }

    #[test]
    fn test_pt2() {
        assert_eq!("13740108158591".to_string(), Day {}.pt2(input()))
    }
}
