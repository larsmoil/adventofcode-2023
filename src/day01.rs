use crate::problem::Solver;

pub struct Day {}

const NUMBER_MAPPING_PT_1: &[(&str, u32)] = &[
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];
const NUMBER_MAPPING_PT_2: &[(&str, u32)] = &[
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

impl Solver for Day {
    fn pt1(&self, input: &str) -> String {
        solve(input, NUMBER_MAPPING_PT_1)
    }
    fn pt2(&self, input: &str) -> String {
        solve(input, NUMBER_MAPPING_PT_2)
    }
}

pub(crate) fn input() -> &'static str {
    include_str!("day01-input.txt").trim()
}

fn solve(input: &str, string_numbers: &[(&str, u32)]) -> String {
    input
        .lines()
        .map(|line| {
            let mut numbers: Vec<u32> = vec![];
            for i in 0..line.len() {
                let slice = &line[i..];
                if let Some(&(_k, v)) = string_numbers.iter().find(|(k, _v)| slice.starts_with(k)) {
                    numbers.push(v);
                }
            }
            numbers.first().unwrap() * 10 + numbers.last().unwrap()
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input_pt1() -> &'static str {
        "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
    }
    fn example_input_pt2() -> &'static str {
        "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
    }

    #[test]
    fn test_pt1_example() {
        assert_eq!("142".to_string(), Day {}.pt1(example_input_pt1()))
    }

    #[test]
    fn test_pt1() {
        assert_eq!("55172".to_string(), Day {}.pt1(input()))
    }

    #[test]
    fn test_pt2_example() {
        assert_eq!("281".to_string(), Day {}.pt2(example_input_pt2()))
    }

    #[test]
    fn test_pt2() {
        assert_eq!("54925".to_string(), Day {}.pt2(input()))
    }
}
