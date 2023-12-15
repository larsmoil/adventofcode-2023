use std::{cmp::Ordering, fmt::Debug};

use crate::problem::Solver;

pub struct Day {}

impl Solver for Day {
    fn pt1(&self, input: &str) -> String {
        let mut dish = ParabolicReflectorDish::from(input.to_owned());
        dish.tilt_north();
        let load = dish.load();
        format!("{load}")
    }
    fn pt2(&self, input: &str) -> String {
        let mut dish = ParabolicReflectorDish::from(input.to_owned());
        let mut patterns: Vec<Vec<Vec<char>>> = vec![];

        let cycles = 1_000_000_000;
        for i in 0..cycles {
            dish.rotate_360();
            patterns.push(dish.0.clone());
            if let Some((index, _)) = patterns
                .iter()
                .take(patterns.len() - 1)
                .enumerate()
                .find(|(_, p)| **p == dish.0.clone())
            {
                let cycle_len = i - index;
                let cycles_to_go = (cycles - i - 1) % cycle_len;

                for _ in 0..cycles_to_go {
                    dish.rotate_360();
                }

                break;
            }
        }

        let load = dish.load();
        format!("{load}")
    }
}

#[derive(Clone, PartialEq)]
struct ParabolicReflectorDish(Vec<Vec<char>>);
impl From<String> for ParabolicReflectorDish {
    fn from(value: String) -> Self {
        Self(value.lines().map(|l| l.chars().collect()).collect())
    }
}
impl Debug for ParabolicReflectorDish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ParabolicReflectorDish")
            .field(&self.0)
            .finish()
    }
}
impl ParabolicReflectorDish {
    fn roll(s: &[char], before: char, after: char) -> Vec<char> {
        let chunks: Vec<Vec<char>> = s
            .split(|c| *c == '#')
            .map(|chunk| {
                let mut chunk = chunk.to_vec();
                chunk.sort_by(|a, b| {
                    if a == b {
                        Ordering::Equal
                    } else if *a == before && *b == after {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                });
                chunk
            })
            .collect::<Vec<Vec<char>>>();

        chunks
            .into_iter()
            .enumerate()
            .fold(vec![], |mut acc, (i, c)| {
                if i != 0 {
                    acc.push('#');
                }
                [acc, c].concat()
            })
    }
    fn roll_end(s: &[char]) -> Vec<char> {
        Self::roll(s, '.', 'O')
    }
    fn roll_start(s: &[char]) -> Vec<char> {
        Self::roll(s, 'O', '.')
    }
    fn rotate_360(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }
    fn tilt_west(&mut self) {
        self.0 = self
            .0
            .iter()
            .map(|l| Self::roll_start(l))
            .collect::<Vec<Vec<char>>>();
    }
    fn tilt_east(&mut self) {
        self.0 = self
            .0
            .iter()
            .map(|l| Self::roll_end(l))
            .collect::<Vec<Vec<char>>>();
    }
    fn tilt_south(&mut self) {
        let tilted_columns: Vec<Vec<char>> = (0..self.0.len())
            .map(|i| self.0.iter().map(|l| l[i]).collect::<Vec<char>>())
            .map(|c| Self::roll_end(&c))
            .collect();

        let lines: Vec<Vec<char>> = (0..self.0.len())
            .map(|i| {
                tilted_columns
                    .iter()
                    .map(|column| column[i])
                    .collect::<Vec<char>>()
            })
            .collect();

        self.0 = lines;
    }

    fn tilt_north(&mut self) {
        let tilted_columns: Vec<Vec<char>> = (0..self.0.len())
            .map(|i| self.0.iter().map(|l| l[i]).collect::<Vec<char>>())
            .map(|c| Self::roll_start(&c))
            .collect();

        let lines: Vec<Vec<char>> = (0..self.0.len())
            .map(|i| {
                tilted_columns
                    .iter()
                    .map(|column| column[i])
                    .collect::<Vec<char>>()
            })
            .collect();

        self.0 = lines;
    }
    fn load(&self) -> usize {
        self.0
            .iter()
            .rev()
            .enumerate()
            .map(|(i, l)| (i + 1) * l.iter().filter(|c| **c == 'O').collect::<Vec<_>>().len())
            .sum()
    }
}

pub(crate) fn input() -> &'static str {
    include_str!("day14-input.txt").trim()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static str {
        "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."
    }

    #[test]
    fn test_load() {
        assert_eq!(
            104,
            ParabolicReflectorDish::from(example_input().to_owned()).load()
        );
    }

    #[test]
    fn test_tilt() {
        let expected = ParabolicReflectorDish::from(
            "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#...."
                .to_owned(),
        );
        let mut actual = ParabolicReflectorDish::from(example_input().to_owned());
        actual.tilt_north();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_pt1_example() {
        assert_eq!("136".to_string(), Day {}.pt1(example_input()));
    }

    #[test]
    fn test_pt1() {
        assert_eq!("109345".to_string(), Day {}.pt1(input()));
    }

    #[test]
    fn test_pt2_example() {
        let expected_1 = ParabolicReflectorDish::from(
            ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#...."
                .to_owned(),
        );
        let expected_2 = ParabolicReflectorDish::from(
            ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O"
                .to_owned(),
        );
        let expected_3 = ParabolicReflectorDish::from(
            ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O"
                .to_owned(),
        );
        let mut actual = ParabolicReflectorDish::from(example_input().to_owned());
        actual.rotate_360();
        assert_eq!(expected_1, actual);
        actual.rotate_360();
        assert_eq!(expected_2, actual);
        actual.rotate_360();
        assert_eq!(expected_3, actual);

        assert_eq!("64".to_string(), Day {}.pt2(example_input()))
    }

    #[test]
    fn test_pt2() {
        assert_eq!("112452".to_string(), Day {}.pt2(input()))
    }
}
