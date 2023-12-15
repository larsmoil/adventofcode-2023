use crate::problem::Solver;

pub struct Day {}

impl Solver for Day {
    fn pt1(&self, input: &str) -> String {
        let maps: Maps = Maps::from(input);
        let reflections = maps.reflections(0);
        format!(
            "{}",
            reflections
                .iter()
                .map(|reflection| {
                    match reflection {
                        Reflection::Horizontal(n) => *n * 100,
                        Reflection::Vertical(n) => *n,
                    }
                })
                .sum::<usize>()
        )
    }
    fn pt2(&self, input: &str) -> String {
        let maps: Maps = Maps::from(input);
        let reflections = maps.reflections(1);
        format!(
            "{}",
            reflections
                .iter()
                .map(|reflection| {
                    match reflection {
                        Reflection::Horizontal(n) => *n * 100,
                        Reflection::Vertical(n) => *n,
                    }
                })
                .sum::<usize>()
        )
    }
}

pub(crate) fn input() -> &'static str {
    include_str!("day13-input.txt").trim()
}

#[derive(Debug, PartialEq)]
enum Reflection {
    Horizontal(usize),
    Vertical(usize),
}

struct Map<'a>(&'a str);
impl<'a> From<&'a str> for Map<'a> {
    fn from(value: &'a str) -> Self {
        Self(value)
    }
}
impl<'a> Map<'a> {
    fn reflection(&self, required_smudges: usize) -> Reflection {
        fn map_top((_n, p): (usize, String)) -> usize {
            p.lines().collect::<Vec<_>>().len() / 2
        }
        fn map_botton((n, p): (usize, String)) -> usize {
            n + p.lines().collect::<Vec<_>>().len() / 2
        }

        let bottom = || Self::from_bottom(self.0, required_smudges).map(map_botton);
        let top = || Self::from_top(self.0, required_smudges).map(map_top);
        bottom().or_else(top).map_or_else(
            || {
                let rotated = Self::rotate(self.0);
                let right = || Self::from_bottom(&rotated, required_smudges).map(map_botton);
                let left = || Self::from_top(&rotated, required_smudges).map(map_top);
                Reflection::Vertical(right().or_else(left).unwrap())
            },
            Reflection::Horizontal,
        )
    }

    fn rotate(pattern: &str) -> String {
        let lines: Vec<&str> = pattern.lines().collect();
        let num_columns = lines.first().unwrap().len();
        let columns: Vec<String> = (0..num_columns)
            .map(|c| {
                lines
                    .iter()
                    .rev()
                    .map(|line| line.chars().nth(c).unwrap())
                    .collect::<String>()
            })
            .collect();
        columns.join("\n")
    }

    fn is_reflection(p: &str, required_smudges: usize) -> (bool, usize) {
        let lines: Vec<&str> = p.lines().collect();
        let mirrored: Vec<&str> = p.lines().rev().take(lines.len() / 2).collect();
        let start: Vec<&str> = p.lines().take(lines.len() / 2).collect::<Vec<&str>>();
        let diffs: usize = mirrored
            .into_iter()
            .zip(start)
            .map(|(a, b)| {
                a.chars()
                    .zip(b.chars())
                    .filter(|(c1, c2)| c1 != c2)
                    .collect::<Vec<_>>()
                    .len()
            })
            .sum();
        let is_reflection = lines.len() % 2 == 0 && diffs == required_smudges;

        (is_reflection, diffs)
    }

    fn from_top(pattern: &str, required_smudges: usize) -> Option<(usize, String)> {
        let lines: Vec<&str> = pattern.lines().collect();
        (1..lines.len() - 1)
            .map(|i| {
                (
                    i,
                    pattern
                        .lines()
                        .take(lines.len() - i)
                        .collect::<Vec<&str>>()
                        .join("\n"),
                )
            })
            .find(|(_, p)| Self::is_reflection(p, required_smudges).0)
    }
    fn from_bottom(pattern: &str, required_smudges: usize) -> Option<(usize, String)> {
        let lines: Vec<&str> = pattern.lines().collect();
        (0..lines.len() - 1)
            .map(|i| (i, pattern.lines().skip(i).collect::<Vec<&str>>().join("\n")))
            .find(|(_, p)| Self::is_reflection(p, required_smudges).0)
    }
}

struct Maps<'a>(Vec<Map<'a>>);
impl<'a> From<&'a str> for Maps<'a> {
    fn from(value: &'a str) -> Self {
        Maps(value.split("\n\n").map(Map::from).collect())
    }
}

impl<'a> Maps<'a> {
    fn reflections(&self, required_smudges: usize) -> Vec<Reflection> {
        self.0
            .iter()
            .map(|map| map.reflection(required_smudges))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static str {
        "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
    }

    #[test]
    fn test_pt1_example() {
        assert_eq!("405".to_string(), Day {}.pt1(example_input()));
    }

    #[test]
    fn test_pt1() {
        let actual = Day {}.pt1(input());
        assert_eq!("29165".to_string(), actual);
    }

    #[test]
    fn test_pt2_example() {
        assert_eq!("400".to_string(), Day {}.pt2(example_input()))
    }

    #[test]
    fn test_pt2() {
        assert_eq!("32192".to_string(), Day {}.pt2(input()))
    }
}
