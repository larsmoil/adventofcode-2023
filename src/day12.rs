use std::fmt::Write;

use crate::problem::Solver;

pub struct Day {}

impl Solver for Day {
    fn pt1(&self, input: &str) -> String {
        format!(
            "{}",
            ConditionRecords::from(input)
                .0
                .iter()
                .map(ConditionRecord::permutations)
                .sum::<usize>()
        )
    }
    fn pt2(&self, input: &str) -> String {
        let multiplied = &mut String::new();
        input.lines().for_each(|line| {
            let (conditions, groups) = line.split_once(' ').unwrap();
            let conditions = std::iter::once(conditions)
                .cycle()
                .take(5)
                .collect::<Vec<&str>>()
                .join("?");
            let groups = std::iter::once(groups)
                .cycle()
                .take(5)
                .collect::<Vec<&str>>()
                .join(",");
            writeln!(multiplied, "{conditions} {groups}").unwrap();
        });
        self.pt1(multiplied)
    }
}

#[derive(Debug, PartialEq)]
struct ConditionRecords<'a>(Vec<ConditionRecord<'a>>);
impl<'a> From<&'a str> for ConditionRecords<'a> {
    fn from(value: &'a str) -> Self {
        Self(value.lines().map(ConditionRecord::from).collect())
    }
}

fn permutations(springs: &str, groups: &[usize]) -> usize {
    let springs = format!(".{}", springs.trim_end_matches('.'));
    let springs: Vec<char> = springs.chars().collect();
    let mut count = vec![0_usize; springs.len() + 1];
    count[0] = 1;

    for (i, _) in springs.iter().take_while(|&&c| c != '#').enumerate() {
        count[i + 1] = 1;
    }

    for group in groups {
        let mut n_c = vec![0; springs.len() + 1];
        let mut chunk = 0;

        for (i, &c) in springs.iter().enumerate() {
            if c == '.' {
                chunk = 0;
            } else {
                chunk += 1;
            }

            if c != '#' {
                n_c[i + 1] += n_c[i];
            }

            if chunk >= *group && springs[i - group] != '#' {
                n_c[i + 1] += count[i - group];
            }
        }

        count = n_c;
    }

    *count.last().unwrap()
}

#[derive(Clone, Debug, PartialEq)]
struct ConditionRecord<'a>(&'a str, Vec<usize>);
impl<'a> ConditionRecord<'a> {
    fn permutations(&self) -> usize {
        permutations(self.0, &self.1)
    }
}
impl<'a> From<&'a str> for ConditionRecord<'a> {
    fn from(value: &'a str) -> Self {
        let (spring_statuses, groups) = value.split_once(' ').unwrap();
        let groups: Vec<usize> = groups.split(',').map(|g| g.parse().unwrap()).collect();
        Self(spring_statuses, groups)
    }
}

pub(crate) fn input() -> &'static str {
    include_str!("day12-input.txt").trim()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static str {
        "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
    }

    #[test]
    fn test_pt1_example() {
        assert_eq!(1, ConditionRecord::from("???.### 1,1,3").permutations());
        assert_eq!(
            4,
            ConditionRecord::from(".??..??...?##. 1,1,3").permutations()
        );
        assert_eq!(
            1,
            ConditionRecord::from("?#?#?#?#?#?#?#? 1,3,1,6").permutations()
        );
        assert_eq!(
            1,
            ConditionRecord::from("????.#...#... 4,1,1").permutations()
        );
        assert_eq!(
            4,
            ConditionRecord::from("????.######..#####. 1,6,5").permutations()
        );
        assert_eq!(
            10,
            ConditionRecord::from("?###???????? 3,2,1").permutations()
        );
        assert_eq!("21".to_string(), Day {}.pt1(example_input()));
    }

    #[test]
    fn test_pt1() {
        assert_eq!("7792".to_string(), Day {}.pt1(input()))
    }

    #[test]
    fn test_pt2_example() {
        assert_eq!("525152".to_string(), Day {}.pt2(example_input()))
    }

    #[test]
    fn test_pt2() {
        assert_eq!("13012052341533".to_string(), Day {}.pt2(input()))
    }
}
