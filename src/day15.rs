use crate::problem::Solver;

pub struct Day {}

impl Solver for Day {
    fn pt1(&self, input: &str) -> String {
        let lens_library = LensLibrary::from(input);
        let hashes = lens_library.hashes();
        format!("{}", hashes.iter().sum::<usize>())
    }
    fn pt2(&self, input: &str) -> String {
        let lens_library = LensLibrary::from(input);
        let focusing_powers: Vec<usize> = lens_library.focusing_powers();
        format!("{}", focusing_powers.iter().sum::<usize>())
    }
}

struct LensLibrary<'a>(&'a str);
impl<'a> From<&'a str> for LensLibrary<'a> {
    fn from(value: &'a str) -> Self {
        Self(value)
    }
}

impl<'a> LensLibrary<'a> {
    fn hashes(&self) -> Vec<usize> {
        self.0.split(',').map(Self::hash).collect()
    }
    fn hash(s: &str) -> usize {
        s.chars().fold(0_usize, |acc, c| (acc + c as usize) * 17) % 256
    }
    fn focusing_powers(&self) -> Vec<usize> {
        let mut boxes: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];
        self.0.split(',').for_each(|lens| {
            let split_index = lens.chars().position(|c| c == '=' || c == '-').unwrap();
            let (lens, operation) = lens.split_at(split_index);
            let the_box = boxes.get_mut(Self::hash(lens)).unwrap();
            match operation {
                "-" => {
                    the_box.retain(|(l, _)| l != &lens);
                }
                &_ => {
                    if let Some(focal_length) = operation.strip_prefix('=') {
                        let focal_length: usize = focal_length.parse().unwrap();
                        if let Some((i, _)) =
                            the_box.iter().enumerate().find(|(_, (l, _))| l == &lens)
                        {
                            let _ = std::mem::replace(&mut the_box[i], (lens, focal_length));
                        } else {
                            the_box.push((lens, focal_length));
                        }
                    } else {
                        panic!("Unknown operation: {operation}")
                    }
                }
            }
        });
        boxes
            .iter()
            .enumerate()
            .map(|(bi, b)| {
                (bi + 1)
                    * b.iter()
                        .enumerate()
                        .map(|(li, t)| (li + 1) * t.1)
                        .sum::<usize>()
            })
            .collect()
    }
}

pub(crate) fn input() -> &'static str {
    include_str!("day15-input.txt").trim()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static str {
        "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"
    }

    #[test]
    fn test_pt1_example() {
        let lens_library = LensLibrary::from(example_input());
        let hashes = lens_library.hashes();
        assert_eq!(vec![30, 253, 97, 47, 14, 180, 9, 197, 48, 214, 231], hashes);
        assert_eq!("1320".to_string(), Day {}.pt1(example_input()));
    }

    #[test]
    fn test_pt1() {
        assert_eq!("510273".to_string(), Day {}.pt1(input()));
    }

    #[test]
    fn test_pt2_example() {
        assert_eq!("145".to_string(), Day {}.pt2(example_input()))
    }

    #[test]
    fn test_pt2() {
        assert_eq!("212449".to_string(), Day {}.pt2(input()))
    }
}
