use crate::{
    libs::{shoelace, Coordinate},
    problem::Solver,
};

pub struct Day {}

impl Solver for Day {
    fn pt1(&self, input: &str) -> String {
        let lavaduct_lagoon = LavaductLagoon::from(input);
        let dug = lavaduct_lagoon.dig(InstructionType::Literal);
        format!("{dug}")
    }
    fn pt2(&self, input: &str) -> String {
        let lavaduct_lagoon = LavaductLagoon::from(input);
        let dug = lavaduct_lagoon.dig(InstructionType::Color);
        format!("{dug}")
    }
}

enum InstructionType {
    Literal,
    Color,
}

struct LavaductLagoon<'a>(Vec<Instruction<'a>>);

impl<'a> LavaductLagoon<'a> {
    fn dig(&self, instruction_type: InstructionType) -> isize {
        let start: Coordinate = Coordinate(0, 0);
        let dig_points: Vec<Coordinate> = self.0.iter().fold(vec![start], |mut acc, dig| {
            let Coordinate(x, y) = *acc.last().unwrap();
            let (direction, steps) = match instruction_type {
                InstructionType::Literal => (dig.0.clone(), dig.1.clone()),
                InstructionType::Color => dig.2.extract_instruction(),
            };
            let to: Coordinate = match direction {
                Direction::North => Coordinate(x, y - steps),
                Direction::East => Coordinate(x + steps, y),
                Direction::South => Coordinate(x, y + steps),
                Direction::West => Coordinate(x - steps, y),
            };
            acc.push(to);
            acc
        });

        shoelace(&dig_points)
    }
}

impl<'a> From<&'a str> for LavaductLagoon<'a> {
    fn from(value: &'a str) -> Self {
        let instructions: Vec<Instruction> = value.lines().map(Instruction::from).collect();
        Self(instructions)
    }
}

#[derive(Clone, Debug)]
struct Instruction<'a>(Direction, isize, Color<'a>);
impl<'a> From<&'a str> for Instruction<'a> {
    fn from(value: &'a str) -> Self {
        let mut direction_length_color = value.split_ascii_whitespace();
        let direction = Direction::from(direction_length_color.next().unwrap());
        let length: isize = direction_length_color.next().unwrap().parse().unwrap();
        let color: Color = Color::from(direction_length_color.next().unwrap());
        Self(direction, length, color)
    }
}

#[derive(Clone, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}
impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "U" => Self::North,
            "R" => Self::East,
            "D" => Self::South,
            "L" => Self::West,
            &_ => todo!("Unknown direction: '{value}'"),
        }
    }
}

#[derive(Clone, Debug)]
struct Color<'a>(&'a str);
impl<'a> From<&'a str> for Color<'a> {
    fn from(value: &'a str) -> Self {
        Self(&value[2..value.len() - 1])
    }
}
impl<'a> Color<'a> {
    fn extract_instruction(&self) -> (Direction, isize) {
        let distance = &self.0[..5];
        let distance: isize = isize::from_str_radix(distance, 16).unwrap();
        let direction = &self.0[5..];
        let direction = match direction {
            "0" => Direction::East,
            "1" => Direction::South,
            "2" => Direction::West,
            "3" => Direction::North,
            &_ => todo!("Unknown direction: {direction}"),
        };

        (direction, distance)
    }
}

pub(crate) fn input() -> &'static str {
    include_str!("day18-input.txt").trim()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static str {
        "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"
    }

    #[test]
    fn test_pt1_example() {
        assert_eq!("62".to_string(), Day {}.pt1(example_input()));
    }

    #[test]
    fn test_pt1() {
        assert_eq!("56923".to_string(), Day {}.pt1(input()));
    }

    #[test]
    fn test_pt2_example() {
        assert_eq!("952408144115".to_string(), Day {}.pt2(example_input()));
    }

    #[test]
    fn test_pt2() {
        assert_eq!("66296566363189".to_string(), Day {}.pt2(input()))
    }
}
