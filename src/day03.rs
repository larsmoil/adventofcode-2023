use std::str::FromStr;

use crate::problem::Solver;

pub struct Day {}

impl Solver for Day {
    fn pt1(&self, input: &str) -> String {
        format!(
            "{:?}",
            input
                .parse::<GearRatios>()
                .unwrap()
                .part_numbers()
                .iter()
                .sum::<u32>()
        )
    }
    fn pt2(&self, input: &str) -> String {
        format!(
            "{:?}",
            input
                .parse::<GearRatios>()
                .unwrap()
                .gear_ratios()
                .iter()
                .sum::<u32>()
        )
    }
}

pub(crate) fn input() -> &'static str {
    include_str!("day03-input.txt").trim()
}

struct GearRatios(String);

impl FromStr for GearRatios {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(GearRatios(String::from(s)))
    }
}

impl GearRatios {
    fn part_numbers(&self) -> Vec<u32> {
        let numbers = self.numbers(|c| c != '.' && !c.is_numeric());
        numbers
            .into_iter()
            .reduce(|acc, e| [acc, e].concat())
            .unwrap()
            .into_iter()
            .map(|e| e.2)
            .collect()
    }

    fn gear_ratios(&self) -> Vec<u32> {
        let numbers = self.numbers(|c| c == '*');
        numbers
            .into_iter()
            .filter(|a| a.len() > 1)
            .map(|t| t.iter().map(|t| t.2).reduce(|acc, e| acc * e).unwrap())
            .collect()
    }

    fn symbols(&self, predicate: fn(char) -> bool) -> Vec<(usize, usize, char)> {
        self.0
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(x, c)| if predicate(c) { Some((x, y, c)) } else { None })
                    .collect()
            })
            .reduce(|acc, e| [acc, e].concat())
            .unwrap()
    }

    fn numbers(&self, adjacent_to: fn(char) -> bool) -> Vec<Vec<(usize, usize, u32)>> {
        let mut numbers: Vec<Vec<(usize, usize, u32)>> = vec![];
        for adjacent_to in self.symbols(adjacent_to) {
            let mut my_numbers: Vec<(usize, usize, u32)> = vec![];
            let mut y = if adjacent_to.1 > 0 {
                adjacent_to.1 - 1
            } else {
                adjacent_to.1
            };

            let lines = &self.0.lines().collect::<Vec<&str>>();
            while y <= adjacent_to.1 + 1 && y < lines.len() {
                let line = lines[y];
                let mut x = if adjacent_to.0 > 0 {
                    adjacent_to.0 - 1
                } else {
                    adjacent_to.0
                };
                while x <= adjacent_to.0 + 1 && x < line.len() {
                    if line[x..].starts_with(|c: char| c.is_numeric()) {
                        while x > 0 && line[x - 1..].starts_with(|c: char| c.is_numeric()) {
                            x -= 1;
                        }
                        let mut end = x;
                        while line[end..].starts_with(|c: char| c.is_numeric()) {
                            end += 1;
                        }
                        my_numbers.push((x, y, line[x..end].parse().unwrap()));
                        x = end + 1;
                    } else {
                        x += 1;
                    }
                }
                y += 1;
            }
            if !my_numbers.is_empty() {
                numbers.push(my_numbers);
            }
        }
        numbers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static str {
        "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
    }

    #[test]
    fn test_pt1_example() {
        assert_eq!("4361".to_string(), Day {}.pt1(example_input()))
    }

    #[test]
    fn test_pt1() {
        assert_eq!("539590".to_string(), Day {}.pt1(input()))
    }

    #[test]
    fn test_pt2_example() {
        assert_eq!("467835".to_string(), Day {}.pt2(example_input()))
    }

    #[test]
    fn test_pt2() {
        assert_eq!("80703636".to_string(), Day {}.pt2(input()))
    }
}
