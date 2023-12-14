use std::str::FromStr;

use crate::problem::Solver;

pub struct Day {}

impl Solver for Day {
    fn pt1(&self, input: &str) -> String {
        let star_map: StarMap = input.parse().unwrap();
        let distances: Vec<usize> = star_map.distances(2);
        format!("{}", distances.iter().sum::<usize>())
    }
    fn pt2(&self, input: &str) -> String {
        let star_map: StarMap = input.parse().unwrap();
        let distances: Vec<usize> = star_map.distances(1_000_000);
        format!("{}", distances.iter().sum::<usize>())
    }
}

pub(crate) fn input() -> &'static str {
    include_str!("day11-input.txt").trim()
}

#[derive(Debug, PartialEq)]
struct StarMap(String);
impl FromStr for StarMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(StarMap(String::from(s)))
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Coordinate(usize, usize);
impl Coordinate {
    fn distance(&self, other: &Coordinate) -> usize {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

impl StarMap {
    fn galaxies(&self, void_multiplier: usize) -> Vec<Coordinate> {
        let xs: Vec<usize> = (0..self.0.lines().collect::<Vec<_>>().len())
            .map(|x| {
                let col: Vec<char> = self
                    .0
                    .lines()
                    .map(|line| line.chars().nth(x).unwrap())
                    .collect();
                if col.clone().into_iter().all(|c| c == '.') {
                    void_multiplier
                } else {
                    1
                }
            })
            .collect();
        let ys: Vec<usize> = self
            .0
            .lines()
            .map(|line| {
                if line.chars().all(|c| c == '.') {
                    void_multiplier
                } else {
                    1
                }
            })
            .collect();
        self.0
            .lines()
            .enumerate()
            .map(|(y, l)| {
                let galaxies: Vec<Coordinate> = l
                    .chars()
                    .enumerate()
                    .filter_map(|(x, c)| match c {
                        '#' => {
                            let compensated_x = (0..x).map(|x| xs[x]).sum::<usize>();
                            let compensated_y = (0..y).map(|y| ys[y]).sum::<usize>();
                            Some(Coordinate(compensated_x, compensated_y))
                        }
                        _ => None,
                    })
                    .collect();
                galaxies
            })
            .reduce(|acc, e| [acc, e].concat())
            .unwrap()
    }
    fn distances(&self, void_multiplier: usize) -> Vec<usize> {
        let mut galaxies = self.galaxies(void_multiplier);
        let mut distances: Vec<usize> = vec![];
        while let Some(galaxy) = galaxies.pop() {
            for other in &galaxies {
                let distance = galaxy.distance(other);
                distances.push(distance);
            }
        }
        distances
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static str {
        "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
    }

    #[test]
    fn test_pt1_example() {
        let star_map: StarMap = example_input().parse().unwrap();
        let galaxies = star_map.galaxies(2);
        assert_eq!(9, galaxies.len());
        assert_eq!(vec![Coordinate(4, 0), Coordinate(9, 1)], galaxies[..2]);
        assert_eq!("374".to_string(), Day {}.pt1(example_input()));
    }

    #[test]
    fn test_pt1() {
        assert_eq!("9565386".to_string(), Day {}.pt1(input()))
    }

    #[test]
    fn test_pt2_example_times_10() {
        let star_map: StarMap = example_input().parse().unwrap();
        let distances = star_map.distances(10);
        assert_eq!(1030, distances.iter().sum::<usize>())
    }

    #[test]
    fn test_pt2_example_times_100() {
        let star_map: StarMap = example_input().parse().unwrap();
        let distances = star_map.distances(100);
        assert_eq!(8410, distances.iter().sum::<usize>())
    }

    #[test]
    fn test_pt2() {
        assert_eq!("857986849428".to_string(), Day {}.pt2(input()))
    }
}
