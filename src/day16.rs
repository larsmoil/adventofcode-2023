use std::str::FromStr;

use num::integer::Roots;

use crate::problem::Solver;

pub struct Day {}

impl Solver for Day {
    fn pt1(&self, input: &str) -> String {
        let lava_production_facility: LavaProductionFacility = input.parse().unwrap();
        let energized = lava_production_facility.energized(((0, 0), EAST));

        format!("{energized}")
    }
    fn pt2(&self, input: &str) -> String {
        let lava_production_facility: LavaProductionFacility = input.parse().unwrap();
        let energized: usize = (0..lava_production_facility.1)
            .flat_map(|i| {
                let from_north = ((i, 0), SOUTH);
                let from_east = ((lava_production_facility.1, i), WEST);
                let from_south = ((i, lava_production_facility.1 - 1), NORTH);
                let from_west = ((0, i), EAST);
                vec![from_north, from_east, from_south, from_west]
            })
            .map(|c| lava_production_facility.energized(c.clone()))
            .max()
            .unwrap();

        format!("{}", energized)
    }
}

pub(crate) fn input() -> &'static str {
    include_str!("day16-input.txt").trim()
}

struct LavaProductionFacility(Vec<char>, usize);
impl FromStr for LavaProductionFacility {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.lines().flat_map(|l| l.chars()).collect();
        let width_height = chars.len().sqrt();
        Ok(Self(chars, width_height))
    }
}

type Beam = (Coordinate, Direction);
type Coordinate = (usize, usize);

type Direction = u8;
const NORTH: u8 = 0b1000;
const EAST: u8 = 0b0100;
const SOUTH: u8 = 0b0010;
const WEST: u8 = 0b0001;

impl LavaProductionFacility {
    fn energized(&self, current: Beam) -> usize {
        let mut state: Vec<Direction> = vec![0; self.1.pow(2)];
        let mut currents: Vec<Beam> = vec![current];

        while let Some(current) = currents.pop() {
            if state[self.index(&current.0)] & current.1 != 0 {
                continue;
            }
            state[self.index(&current.0)] |= current.1;
            let next = self.pass_beam(current);
            for n in next {
                currents.push(n.clone());
            }
        }
        state.into_iter().filter(|s| *s != 0).count()
    }
    fn pass_beam<'a>(&self, current: Beam) -> impl Iterator<Item = Beam> + 'a {
        let (coordinate, direction) = (current.0, current.1);
        let current_type = self.0[self.index(&coordinate)];
        match (current_type, direction) {
            ('.', _) | ('-', EAST | WEST) | ('|', NORTH | SOUTH) => {
                vec![self.go(&coordinate, direction)]
            }
            ('-', NORTH | SOUTH) => {
                vec![self.go(&coordinate, EAST), self.go(&coordinate, WEST)]
            }
            ('|', EAST | WEST) => vec![self.go(&coordinate, NORTH), self.go(&coordinate, SOUTH)],
            ('\\', SOUTH) | ('/', NORTH) => {
                vec![self.go(&coordinate, EAST)]
            }
            ('\\', WEST) | ('/', EAST) => {
                vec![self.go(&coordinate, NORTH)]
            }
            ('\\', NORTH) | ('/', SOUTH) => {
                vec![self.go(&coordinate, WEST)]
            }
            ('\\', EAST) | ('/', WEST) => {
                vec![self.go(&coordinate, SOUTH)]
            }
            _ => todo!("Handle type {current_type} {direction:?}"),
        }
        .into_iter()
        .flatten()
    }
    fn index(&self, coordinate: &Coordinate) -> usize {
        coordinate.0 % self.1 + coordinate.1 * self.1
    }
    fn go(&self, from: &Coordinate, to: Direction) -> Option<Beam> {
        match to {
            NORTH => {
                if from.1 > 0 {
                    Some(((from.0, from.1 - 1), to.clone()))
                } else {
                    None
                }
            }
            EAST => {
                if from.0 < self.1 - 1 {
                    Some(((from.0 + 1, from.1), to.clone()))
                } else {
                    None
                }
            }
            SOUTH => {
                if from.1 < self.1 - 1 {
                    Some(((from.0, from.1 + 1), to.clone()))
                } else {
                    None
                }
            }
            WEST => {
                if from.0 > 0 {
                    Some(((from.0 - 1, from.1), to.clone()))
                } else {
                    None
                }
            }
            _ => todo!("Unknown direction: {to}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static str {
        ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|...."
    }

    #[test]
    fn test_pt1_example() {
        assert_eq!("46".to_string(), Day {}.pt1(example_input()));
    }

    #[test]
    fn test_pt1() {
        assert_eq!("7482".to_string(), Day {}.pt1(input()));
    }

    #[test]
    fn test_pt2_example() {
        assert_eq!("51".to_string(), Day {}.pt2(example_input()))
    }

    #[test]
    fn test_pt2() {
        assert_eq!("7896".to_string(), Day {}.pt2(input()))
    }
}
