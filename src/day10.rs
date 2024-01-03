use std::str::FromStr;

use crate::libs::{shoelace, Coordinate};
use crate::problem::Solver;

pub struct Day {}

impl Solver for Day {
    fn pt1(&self, input: &str) -> String {
        let pipe_maze = input.parse::<PipeMaze>().unwrap();
        let cycle = pipe_maze.cycle();
        let furthest = cycle.len() / 2;
        format!("{furthest}")
    }
    fn pt2(&self, input: &str) -> String {
        let pipe_maze = input.parse::<PipeMaze>().unwrap();
        let enclosed = pipe_maze.enclosed();
        format!("{enclosed}")
    }
}

pub(crate) fn input() -> &'static str {
    include_str!("day10-input.txt").trim()
}

impl PipeMaze {
    fn cycle(&self) -> Vec<Coordinate<isize>> {
        let start = self.start();
        let mut connections = self.connections(&start);

        let mut cycle: Vec<Coordinate<isize>> = vec![start.clone()];
        let mut from = start.clone();
        let mut to = connections.next().unwrap().clone();
        while to != start {
            let new_tos: Vec<Coordinate<isize>> =
                self.connections(&to).filter(|c| c != &from).collect();
            assert!(
                new_tos.len() == 1,
                "expected single connection, got: {new_tos:?} (from: {from}, to: {to})"
            );

            let new_to = new_tos.first().unwrap().clone();
            from = to;
            to = new_to;
            cycle.push(to.clone());
        }
        cycle
    }

    fn enclosed(&self) -> isize {
        shoelace(&self.cycle()) - isize::try_from(self.cycle().len()).unwrap()
    }

    fn start(&self) -> Coordinate<isize> {
        let (i, _e) = self
            .pipes
            .iter()
            .enumerate()
            .find(|(_, pipe_type)| **pipe_type == PipeType::Start)
            .unwrap();
        Coordinate(
            isize::try_from(i % self.width).unwrap(),
            isize::try_from(i / self.width).unwrap(),
        )
    }

    fn connections(&self, position: &Coordinate<isize>) -> impl Iterator<Item = Coordinate<isize>> {
        [
            self.north(position),
            self.east(position),
            self.south(position),
            self.west(position),
        ]
        .into_iter()
        .flatten()
    }

    fn pipe_type(&self, position: &Coordinate<isize>) -> &PipeType {
        &self.pipes[usize::try_from(position.0).unwrap()
            + usize::try_from(position.1).unwrap() * self.width]
    }

    fn connection(
        &self,
        from: &Coordinate<isize>,
        direction: &Direction,
    ) -> Option<Coordinate<isize>> {
        let Coordinate(from_x, from_y) = *from;

        let to: Option<Coordinate<isize>> = match direction {
            Direction::North => {
                if from_y > 0 {
                    Some(Coordinate(from.0, from.1 - 1))
                } else {
                    None
                }
            }
            Direction::East => {
                if from_x < isize::try_from(self.width).unwrap() - 1 {
                    Some(Coordinate(from.0 + 1, from.1))
                } else {
                    None
                }
            }
            Direction::South => {
                if from_y
                    < (isize::try_from(self.pipes.len()).unwrap()
                        / isize::try_from(self.width).unwrap())
                        - 1
                {
                    Some(Coordinate(from.0, from.1 + 1))
                } else {
                    None
                }
            }
            Direction::West => {
                if from_x > 0 {
                    Some(Coordinate(from.0 - 1, from.1))
                } else {
                    None
                }
            }
        };

        if let Some(to) = to {
            let type_from = self.pipe_type(from);
            let type_to = self.pipe_type(&to);

            if type_from.connects(type_to, direction) {
                Some(to)
            } else {
                None
            }
        } else {
            None
        }
    }
    fn north(&self, position: &Coordinate<isize>) -> Option<Coordinate<isize>> {
        self.connection(position, &Direction::North)
    }
    fn east(&self, position: &Coordinate<isize>) -> Option<Coordinate<isize>> {
        self.connection(position, &Direction::East)
    }
    fn south(&self, position: &Coordinate<isize>) -> Option<Coordinate<isize>> {
        self.connection(position, &Direction::South)
    }
    fn west(&self, position: &Coordinate<isize>) -> Option<Coordinate<isize>> {
        self.connection(position, &Direction::West)
    }
}

impl FromStr for PipeMaze {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let width = lines.first().unwrap().len();
        let pipes: Vec<PipeType> = lines
            .iter()
            .flat_map(|line| {
                line.chars()
                    .map(|c| format!("{c}").parse::<PipeType>().unwrap())
                    .collect::<Vec<PipeType>>()
            })
            .collect();
        Ok(PipeMaze { pipes, width })
    }
}

#[derive(Debug, PartialEq)]
struct PipeMaze {
    pipes: Vec<PipeType>,
    width: usize,
}

#[derive(Clone, Debug, PartialEq)]
enum PipeType {
    NorthSouth,
    EastWest,
    Ground,
    Start,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    fn invert(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

impl PipeType {
    fn to(&self) -> Vec<Direction> {
        match self {
            PipeType::NorthSouth => vec![Direction::North, Direction::South],
            PipeType::EastWest => vec![Direction::East, Direction::West],
            PipeType::Ground => vec![],
            PipeType::Start => vec![
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ],
            PipeType::NorthEast => vec![Direction::North, Direction::East],
            PipeType::NorthWest => vec![Direction::North, Direction::West],
            PipeType::SouthWest => vec![Direction::South, Direction::West],
            PipeType::SouthEast => vec![Direction::South, Direction::East],
        }
    }
    fn connects(&self, other: &PipeType, direction: &Direction) -> bool {
        self.to().contains(direction) && other.to().contains(&direction.invert())
    }
}

impl ToString for PipeType {
    fn to_string(&self) -> String {
        String::from(match self {
            PipeType::NorthSouth => "|",
            PipeType::EastWest => "-",
            PipeType::NorthEast => "L",
            PipeType::NorthWest => "J",
            PipeType::SouthWest => "7",
            PipeType::SouthEast => "F",
            PipeType::Ground => ".",
            PipeType::Start => "S",
        })
    }
}
impl FromStr for PipeType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert!(
            s.len() == 1,
            "Unparsable, expected a single character string. Got: '{s}'"
        );

        Ok(match s {
            "|" => PipeType::NorthSouth,
            "-" => PipeType::EastWest,
            "L" => PipeType::NorthEast,
            "J" => PipeType::NorthWest,
            "7" => PipeType::SouthWest,
            "F" => PipeType::SouthEast,
            "." => PipeType::Ground,
            "S" => PipeType::Start,
            &_ => todo!("Unknown pipe type '{s}'"),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt1_example_1() {
        assert_eq!(
            "4".to_string(),
            Day {}.pt1(
                ".....
.S-7.
.|.|.
.L-J.
....."
            )
        )
    }

    #[test]
    fn test_pt1_example_2() {
        assert_eq!(
            "8".to_string(),
            Day {}.pt1(
                "..F7.
.FJ|.
SJ.L7
|F--J
LJ..."
            )
        )
    }

    #[test]
    fn test_pt1() {
        assert_eq!("6701".to_string(), Day {}.pt1(input()))
    }

    #[test]
    fn test_pt2_example_1() {
        assert_eq!(
            "4".to_string(),
            Day {}.pt2(
                "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."
            )
        )
    }

    #[test]
    fn test_pt2_example_2() {
        assert_eq!(
            "4".to_string(),
            Day {}.pt2(
                "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
.........."
            )
        )
    }

    #[test]
    fn test_pt2_example_3() {
        assert_eq!(
            "8".to_string(),
            Day {}.pt2(
                ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."
            )
        )
    }

    #[test]
    fn test_pt2_example_4() {
        assert_eq!(
            "10".to_string(),
            Day {}.pt2(
                "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"
            )
        )
    }

    #[test]
    fn test_pt2() {
        assert_eq!("303".to_string(), Day {}.pt2(input()))
    }
}
