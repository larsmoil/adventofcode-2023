use std::{
    collections::{BinaryHeap, HashMap},
    num::TryFromIntError,
    str::FromStr,
};

use crate::problem::Solver;

pub struct Day {}

impl Solver for Day {
    fn pt1(&self, input: &str) -> String {
        let map: Map = input.parse().unwrap();
        let losses = map
            .dijkstra((0, 0), (map.width() - 1, map.height() - 1), |n| {
                map.neighbors_pt1(*n)
            })
            .unwrap();
        format!("{losses}")
    }
    fn pt2(&self, input: &str) -> String {
        let map: Map = input.parse().unwrap();
        let losses = map
            .dijkstra((0, 0), (map.width() - 1, map.height() - 1), |n| {
                map.neighbors_pt2(*n)
            })
            .unwrap();
        format!("{losses}")
    }
}

pub(crate) fn input() -> &'static str {
    include_str!("day17-input.txt").trim()
}

struct Map(Vec<u8>, u8);
impl FromStr for Map {
    type Err = TryFromIntError;

    #[allow(clippy::cast_possible_truncation)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = u8::try_from(s.lines().collect::<Vec<_>>().first().unwrap().len())?;
        let numbers: Vec<u8> = s
            .lines()
            .flat_map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect::<Vec<u8>>()
            })
            .collect();

        Ok(Self(numbers, width))
    }
}

type Direction = u8;
const NORTH: u8 = 0b1000;
const EAST: u8 = 0b0100;
const SOUTH: u8 = 0b0010;
const WEST: u8 = 0b0001;

type DirectionCount = u8;

type Coordinate = (u8, u8);
type Cost = u32;

#[derive(Eq, PartialEq)]
struct Process(Cost, (Coordinate, Direction, DirectionCount));
impl Ord for Process {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0).then_with(|| self.1.cmp(&other.1))
    }
}
impl PartialOrd for Process {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Map {
    fn width(&self) -> u8 {
        self.1
    }

    fn height(&self) -> u8 {
        u8::try_from(self.0.len() / self.1 as usize).unwrap()
    }

    fn index(&self, coordinate: Coordinate) -> usize {
        coordinate.0 as usize + coordinate.1 as usize * self.width() as usize
    }

    fn neighbors_pt1(
        &self,
        of: (Coordinate, Direction, DirectionCount),
    ) -> Vec<(Coordinate, Direction, DirectionCount)> {
        self.neighbors(of).into_iter().filter(|n| n.2 < 3).collect()
    }

    fn neighbors_pt2(
        &self,
        of: (Coordinate, Direction, DirectionCount),
    ) -> Vec<(Coordinate, Direction, DirectionCount)> {
        let (_from_coordinate, from_direction, from_dir_count) = of;
        self.neighbors(of)
            .into_iter()
            .filter(
                |&(_neighbor_coordinate, neighbor_direction, neighbor_dir_count)| {
                    if from_direction == neighbor_direction {
                        neighbor_dir_count < 10
                    } else if from_direction != neighbor_direction {
                        from_dir_count >= 3
                    } else {
                        false
                    }
                },
            )
            .collect()
    }

    fn neighbors(
        &self,
        of: (Coordinate, Direction, DirectionCount),
    ) -> Vec<(Coordinate, Direction, DirectionCount)> {
        let (coordinate, direction, count) = of;
        let north: Option<(Coordinate, Direction)> = if coordinate.1 > 0 && direction != SOUTH {
            Some(((coordinate.0, coordinate.1 - 1), NORTH))
        } else {
            None
        };
        let east: Option<(Coordinate, Direction)> =
            if coordinate.0 < self.width() - 1 && direction != WEST {
                Some(((coordinate.0 + 1, coordinate.1), EAST))
            } else {
                None
            };
        let south: Option<(Coordinate, Direction)> =
            if coordinate.1 < self.height() - 1 && direction != NORTH {
                Some(((coordinate.0, coordinate.1 + 1), SOUTH))
            } else {
                None
            };
        let west: Option<(Coordinate, Direction)> = if coordinate.0 > 0 && direction != EAST {
            Some(((coordinate.0 - 1, coordinate.1), WEST))
        } else {
            None
        };
        vec![north, east, south, west]
            .into_iter()
            .flatten()
            .map(|n| (n.0, n.1, if n.1 == direction { count + 1 } else { 0 }))
            .collect()
    }

    fn dijkstra<F>(&self, start: Coordinate, goal: Coordinate, next_fn: F) -> Option<Cost>
    where
        F: Fn(
            &(Coordinate, Direction, DirectionCount),
        ) -> Vec<(Coordinate, Direction, DirectionCount)>,
    {
        let mut distances: HashMap<(Coordinate, Direction, DirectionCount), Cost> = HashMap::new();
        distances.insert((start, SOUTH, 0), 0);
        distances.insert((start, EAST, 0), 0);

        let mut to_process: BinaryHeap<Process> = BinaryHeap::new();
        to_process.push(Process(0, (start, SOUTH, 0)));
        to_process.push(Process(0, (start, EAST, 0)));

        while let Some(Process(cost, (coordinate, direction, direction_count))) = to_process.pop() {
            if coordinate == goal {
                return Some(cost);
            }

            for neighbor in next_fn(&(coordinate, direction, direction_count)) {
                let neighbor_cost = u32::from(self.0[self.index(neighbor.0)]);
                let new_cost = cost + neighbor_cost;

                if let Some(&best) = distances.get(&neighbor) {
                    if new_cost >= best {
                        continue;
                    }
                }

                distances.insert(neighbor, new_cost);

                let for_process = Process(new_cost, neighbor);
                to_process.push(for_process);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static str {
        "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"
    }

    #[test]
    fn test_pt1_example_neighbors() {
        let map: Map = example_input().parse().unwrap();
        assert_eq!(13, map.width());
        assert_eq!(13, map.height());

        assert_eq!(vec![2, 4, 1], map.0[..3]);
        assert_eq!(
            vec![5, 3, 3],
            map.0[(map.1.pow(2) as usize - 3)..map.1.pow(2) as usize]
        );

        let expected: Vec<(Coordinate, Direction, DirectionCount)> =
            vec![((1, 0), EAST, 0), ((0, 1), SOUTH, 2)];
        assert_eq!(expected, map.neighbors(((0, 0), SOUTH, 1)));

        let expected: Vec<(Coordinate, Direction, DirectionCount)> =
            vec![((1, 0), EAST, 0), ((0, 1), SOUTH, 3)];
        assert_eq!(expected, map.neighbors(((0, 0), SOUTH, 2)));
    }

    #[test]
    fn test_pt1_example() {
        let map: Map = example_input().parse().unwrap();
        assert_eq!(
            Some(4),
            map.dijkstra((0, 0), (1, 0), |n| map.neighbors_pt1(*n))
        );
        assert_eq!(
            Some(5),
            map.dijkstra((0, 0), (2, 0), |n| map.neighbors_pt1(*n))
        );
        assert_eq!(
            Some(102),
            map.dijkstra((0, 0), (map.width() - 1, map.height() - 1), |n| map
                .neighbors_pt1(*n))
        );
        assert_eq!("102".to_string(), Day {}.pt1(example_input()));
    }

    #[test]
    fn test_pt1() {
        assert_eq!("638".to_string(), Day {}.pt1(input()));
    }

    #[test]
    fn test_pt2_example_neighbors() {
        let map: Map = example_input().parse().unwrap();

        let expected: Vec<(Coordinate, Direction, DirectionCount)> = vec![((0, 1), SOUTH, 1)];
        assert_eq!(expected, map.neighbors_pt2(((0, 0), SOUTH, 0)));

        let expected: Vec<(Coordinate, Direction, DirectionCount)> = vec![((0, 2), SOUTH, 2)];
        assert_eq!(expected, map.neighbors_pt2(((0, 1), SOUTH, 1)));

        let expected: Vec<(Coordinate, Direction, DirectionCount)> = vec![((0, 3), SOUTH, 3)];
        assert_eq!(expected, map.neighbors_pt2(((0, 2), SOUTH, 2)));

        let expected: Vec<(Coordinate, Direction, DirectionCount)> =
            vec![((1, 3), EAST, 0), ((0, 4), SOUTH, 4)];
        assert_eq!(expected, map.neighbors_pt2(((0, 3), SOUTH, 3)));

        let expected: Vec<(Coordinate, Direction, DirectionCount)> =
            vec![((1, 4), EAST, 0), ((0, 5), SOUTH, 5)];
        assert_eq!(expected, map.neighbors_pt2(((0, 4), SOUTH, 4)));

        let expected: Vec<(Coordinate, Direction, DirectionCount)> =
            vec![((1, 5), EAST, 0), ((0, 6), SOUTH, 6)];
        assert_eq!(expected, map.neighbors_pt2(((0, 5), SOUTH, 5)));

        let expected: Vec<(Coordinate, Direction, DirectionCount)> =
            vec![((1, 6), EAST, 0), ((0, 7), SOUTH, 7)];
        assert_eq!(expected, map.neighbors_pt2(((0, 6), SOUTH, 6)));

        let expected: Vec<(Coordinate, Direction, DirectionCount)> =
            vec![((1, 7), EAST, 0), ((0, 8), SOUTH, 8)];
        assert_eq!(expected, map.neighbors_pt2(((0, 7), SOUTH, 7)));

        let expected: Vec<(Coordinate, Direction, DirectionCount)> =
            vec![((1, 8), EAST, 0), ((0, 9), SOUTH, 9)];
        assert_eq!(expected, map.neighbors_pt2(((0, 8), SOUTH, 8)));

        let expected: Vec<(Coordinate, Direction, DirectionCount)> = vec![((1, 9), EAST, 0)];
        assert_eq!(expected, map.neighbors_pt2(((0, 9), SOUTH, 9)));

        let expected: Vec<(Coordinate, Direction, DirectionCount)> = vec![((1, 9), EAST, 0)];
        assert_eq!(expected, map.neighbors_pt2(((0, 9), SOUTH, 10)));
    }

    #[test]
    fn test_pt2_example() {
        assert_eq!("94".to_string(), Day {}.pt2(example_input()));
    }

    #[test]
    #[ignore]
    fn test_pt2_example_2() {
        let map: Map = "111111111111
999999999991
999999999991
999999999991
999999999991"
            .parse()
            .unwrap();
        assert_eq!(12, map.width());
        assert_eq!(5, map.height());

        assert_eq!(
            Some(9),
            map.dijkstra((0, 0), (9, 0), |n| map.neighbors_pt2(*n))
        );
        assert_eq!(
            Some(71),
            map.dijkstra((0, 0), (map.width() - 1, map.height() - 1), |n| map
                .neighbors_pt2(*n))
        );
    }

    #[test]
    fn test_pt2() {
        assert_eq!("748".to_string(), Day {}.pt2(input()))
    }
}
