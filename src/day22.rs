use std::{
    collections::HashSet,
    fmt::{Debug, Display},
    ops::RangeInclusive,
};

use crate::problem::Solver;
pub struct Day {}

impl Solver for Day {
    fn pt1(&self, input: &str) -> String {
        let mut bricks = Bricks::from(input);
        bricks.settle();
        let removable = bricks.removable().len();
        format!("{removable}")
    }
    fn pt2(&self, input: &str) -> String {
        let mut bricks = Bricks::from(input);
        bricks.settle();
        let disintegrated = bricks.disintegrate();
        format!("{}", disintegrated.iter().sum::<usize>())
    }
}

pub(crate) fn input() -> &'static str {
    include_str!("day22-input.txt").trim()
}

impl Bricks {
    fn disintegrate(&self) -> Vec<usize> {
        self.bricks
            .iter()
            .map(|brick| {
                let mut is_falling = vec![false; self.bricks.len()];
                let mut queue: Vec<BrickId> = vec![];

                is_falling[brick.id] = true;
                queue.push(brick.id);

                while !queue.is_empty() {
                    let b = &self.bricks[queue.remove(0)];
                    for b in &b.above {
                        if !is_falling[*b] && self.bricks[*b].below.iter().all(|&b| is_falling[b]) {
                            is_falling[*b] = true;
                            queue.push(*b);
                        }
                    }
                }

                is_falling.into_iter().filter(|f| *f).count() - 1
            })
            .collect()
    }
    fn removable(&self) -> Vec<&Brick> {
        self.bricks
            .iter()
            .filter(|brick| !brick.above.iter().any(|b| self.bricks[*b].below.len() == 1))
            .collect()
    }
    fn settle(&mut self) {
        let mut height_grid: [[BrickId; 10]; 10] = [[NO_BRICK; 10]; 10];

        for i in 0..self.bricks.len() {
            let mut brick = self.bricks[i].clone();

            let mut below = HashSet::new();
            let mut high: usize = 0;

            for x in brick.x.clone() {
                for y in brick.y.clone() {
                    if height_grid[x][y] != NO_BRICK {
                        high = high.max(*self.bricks[height_grid[x][y]].z.end());
                        below.insert(height_grid[x][y]);
                    }
                    height_grid[x][y] = brick.id;
                }
            }
            for id in below {
                if *self.bricks[id].z.end() == high {
                    brick.below.push(id);
                    self.bricks[id].above.push(brick.id);
                }
            }
            let d = brick.z.end() - brick.z.start();
            brick.z = high + 1..=high + 1 + d;
            self.bricks[i] = brick;
        }
    }
}

#[derive(PartialEq)]
struct Bricks {
    bricks: Vec<Brick>,
    is_falling: Vec<bool>,
}

type BrickId = usize;
const NO_BRICK: BrickId = usize::MAX;

#[derive(Clone, PartialEq)]
struct Brick {
    x: RangeInclusive<usize>,
    y: RangeInclusive<usize>,
    z: RangeInclusive<usize>,
    id: BrickId,
    below: Vec<BrickId>,
    above: Vec<BrickId>,
}

impl From<&str> for Bricks {
    fn from(value: &str) -> Self {
        let mut bricks: Vec<Brick> = value.lines().map(Brick::from).collect();
        bricks.sort_by_key(|b| *b.z.start());
        bricks.iter_mut().enumerate().for_each(|(i, b)| b.id = i);
        Self {
            bricks: bricks.clone(),
            is_falling: vec![false; bricks.len()],
        }
    }
}

impl From<&str> for Brick {
    fn from(value: &str) -> Self {
        let (a, b) = value.split_once('~').unwrap();
        let start: Vec<usize> = a.split(',').map(|n| n.parse::<usize>().unwrap()).collect();
        let end: Vec<usize> = b.split(',').map(|n| n.parse::<usize>().unwrap()).collect();

        Self {
            x: start[0].min(end[0])..=start[0].max(end[0]),
            y: start[1].min(end[1])..=start[1].max(end[1]),
            z: start[2].min(end[2])..=start[2].max(end[2]),
            id: 0,
            below: vec![],
            above: vec![],
        }
    }
}

impl Debug for Bricks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", &self))
    }
}
impl Display for Bricks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.bricks
            .iter()
            .fold(Ok(()), |_, brick| f.write_fmt(format_args!("{}\n", &brick)))
    }
}
impl Debug for Brick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", &self))
    }
}
impl Display for Brick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{},{},{}~{},{},{}",
            &self.x.start(),
            &self.y.start(),
            &self.z.start(),
            &self.x.end(),
            &self.y.end(),
            &self.z.end(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static str {
        "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"
    }

    #[test]
    fn test_pt1_example_settle() {
        let mut bricks = Bricks::from(example_input());
        bricks.settle();

        let expected = String::from(
            "1,0,1~1,2,1
0,0,2~2,0,2
0,2,2~2,2,2
0,0,3~0,2,3
2,0,3~2,2,3
0,1,4~2,1,4
1,1,5~1,1,6
",
        );
        assert_eq!(expected, bricks.to_string());
    }

    #[test]
    fn test_pt1_example() {
        assert_eq!("5".to_string(), Day {}.pt1(example_input()));
    }

    #[test]
    fn test_pt1() {
        let actual = Day {}.pt1(input());
        assert_eq!("507".to_string(), actual);
    }

    #[test]
    fn test_pt2_example() {
        assert_eq!("7".to_string(), Day {}.pt2(example_input()));
    }

    #[test]
    fn test_pt2() {
        assert_eq!("51733".to_string(), Day {}.pt2(input()));
    }
}
