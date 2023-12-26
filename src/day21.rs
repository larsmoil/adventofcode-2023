use std::{collections::HashSet, iter};

use crate::{libs::Coordinate, problem::Solver};
pub struct Day {}

impl Solver for Day {
    fn pt1(&self, input: &str) -> String {
        let map = Map::from(input);
        let plots = map.plots(64, None).unwrap();
        format!("{plots}")
    }
    fn pt2(&self, input: &str) -> String {
        let plots = Map::pt2(&input.bytes().collect::<Vec<_>>(), 26501365);
        format!("{plots}")
    }
}

impl Map {
    fn pt2(input: &[u8], steps: u64) -> u64 {
        let targets = [0, 65, 65 + 131, 65 + 2 * 131];
        let start = (65i16, 65i16);
        let a_b_c = targets
            .windows(2)
            .scan(HashSet::from_iter(iter::once(start)), |queue, w| {
                let (start, end) = (w[0], w[1]);
                for _ in start..end {
                    *queue = queue
                        .iter()
                        .flat_map(|&(y, x)| {
                            [(0, 1), (0, -1), (1, 0), (-1, 0)]
                                .into_iter()
                                .map(move |(dy, dx)| (y + dy, x + dx))
                                .filter(|&(y, x)| {
                                    input[(y.rem_euclid(131) as usize * 132)
                                        + x.rem_euclid(131) as usize]
                                        != b'#'
                                })
                        })
                        .collect::<HashSet<_>>();
                }
                Some(queue.len() as u64)
            })
            .collect::<Vec<_>>();
        let (a, b, c) = (a_b_c[0], a_b_c[1], a_b_c[2]);

        let x = steps / 131;
        a + x * (b - a) + x * (x - 1) / 2 * ((c - b) - (b - a))
    }

    fn plots(&self, steps: u64, visited: Option<HashSet<Coordinate>>) -> Option<u64> {
        if steps == 0 {
            visited.map(|v| u64::try_from(v.len()).unwrap())
        } else {
            let reachable: HashSet<Coordinate> = visited
                .unwrap_or_else(|| HashSet::from_iter([self.start()]))
                .iter()
                .flat_map(|c| self.reachable(c))
                .collect();

            self.plots(steps - 1, Some(reachable))
        }
    }
    fn start(&self) -> Coordinate {
        let s = isize::try_from(
            self.terrain
                .iter()
                .enumerate()
                .find(|(_, t)| t == &&Terrain::Start)
                .unwrap()
                .0,
        )
        .unwrap();
        Coordinate(s % self.width, s / self.width)
    }
    fn reachable(&self, from: &Coordinate) -> HashSet<Coordinate> {
        [
            self.up(from),
            self.right(from),
            self.down(from),
            self.left(from),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
    fn terrain_type(&self, coordinate: &Coordinate) -> Terrain {
        self.terrain
            [usize::try_from(coordinate.0 % self.width + coordinate.1 * self.width).unwrap()]
    }
    fn up(&self, from: &Coordinate) -> Option<Coordinate> {
        if from.1 > 0 {
            Some(Coordinate(from.0, from.1 - 1))
        } else {
            None
        }
        .filter(|c| self.terrain_type(c) != Terrain::Rock)
    }
    fn right(&self, from: &Coordinate) -> Option<Coordinate> {
        if from.0 < self.width - 1 {
            Some(Coordinate(from.0 + 1, from.1))
        } else {
            None
        }
        .filter(|c| self.terrain_type(c) != Terrain::Rock)
    }
    fn down(&self, from: &Coordinate) -> Option<Coordinate> {
        if from.1 < self.width - 1 {
            Some(Coordinate(from.0, from.1 + 1))
        } else {
            None
        }
        .filter(|c| self.terrain_type(c) != Terrain::Rock)
    }
    fn left(&self, from: &Coordinate) -> Option<Coordinate> {
        if from.0 > 0 {
            Some(Coordinate(from.0 - 1, from.1))
        } else {
            None
        }
        .filter(|c| self.terrain_type(c) != Terrain::Rock)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Terrain {
    Start,
    Rock,
    GardenPlot,
}
impl From<char> for Terrain {
    fn from(value: char) -> Self {
        match value {
            'S' => Terrain::Start,
            '.' => Terrain::GardenPlot,
            '#' => Terrain::Rock,
            _ => todo!("Unknown Terrain: {value}"),
        }
    }
}

#[derive(Debug)]
struct Map {
    terrain: Vec<Terrain>,
    width: isize,
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let width: isize = value
            .lines()
            .collect::<Vec<_>>()
            .first()
            .unwrap()
            .len()
            .try_into()
            .unwrap();
        let terrain: Vec<Terrain> = value
            .lines()
            .flat_map(|line| line.chars().map(Terrain::from))
            .collect();
        Self { terrain, width }
    }
}

pub(crate) fn input() -> &'static str {
    include_str!("day21-input.txt").trim()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static str {
        "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."
    }

    #[test]
    fn test_pt1_example() {
        let map = Map::from(example_input());

        let plots = map.plots(0, None);
        assert_eq!(None, plots);

        let plots = map.plots(1, None);
        assert_eq!(Some(2), plots);

        let plots = map.plots(2, None);
        assert_eq!(Some(4), plots);

        let plots = map.plots(3, None);
        assert_eq!(Some(6), plots);

        let plots = map.plots(6, None);
        assert_eq!(Some(16), plots);
    }

    #[test]
    fn test_pt1() {
        assert_eq!("3733".to_string(), Day {}.pt1(input()));
    }

    #[test]
    #[ignore]
    fn test_pt2_example() {
        let plots = Map::pt2(&example_input().bytes().collect::<Vec<_>>(), 5000);
        assert_eq!(16733044, plots);
    }

    #[test]
    fn test_pt2() {
        assert_eq!("617729401414635".to_string(), Day {}.pt2(input()));
    }
}
