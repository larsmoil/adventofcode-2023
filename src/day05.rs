use std::{ops::Range, str::FromStr};

use crate::problem::Solver;

pub struct Day {}

impl Solver for Day {
    fn pt1(&self, input: &str) -> String {
        let soil_location = input.parse::<Almanac>().unwrap().map(false);
        format!("{soil_location}")
    }
    fn pt2(&self, input: &str) -> String {
        let soil_location = input.parse::<Almanac>().unwrap().map(true);
        format!("{soil_location}")
    }
}

pub(crate) fn input() -> &'static str {
    include_str!("day05-input.txt").trim()
}

#[derive(Debug, PartialEq)]
struct Map {
    name: String,
    ranges: Vec<(Range<i64>, i64)>,
}

impl Map {
    fn process_seeds(&self, seeds: &[Range<i64>]) -> Vec<Range<i64>> {
        seeds
            .iter()
            .map(|seed| self.process_seed(seed))
            .reduce(|acc, e| [acc, e].concat())
            .unwrap()
    }
    fn process_seed(&self, seed: &Range<i64>) -> Vec<Range<i64>> {
        let mut seeds: Vec<(Range<i64>, i64)> = vec![];
        let mut ranges = self
            .ranges
            .iter()
            .filter(|range| {
                range.0.contains(&seed.start) && !(range.0.start..seed.start).is_empty()
                    || range.0.contains(&seed.end) && !(range.0.start..seed.end).is_empty()
                    || seed.contains(&range.0.start) && !(seed.start..range.0.start).is_empty()
                    || seed.contains(&range.0.end) && !(range.0.end..seed.start).is_empty()
            })
            .collect::<Vec<&(Range<i64>, i64)>>();
        ranges.sort_by(|a, b| a.0.start.cmp(&b.0.start));
        if ranges.is_empty() {
            seeds.push((seed.clone(), 0));
        } else {
            for range in &ranges {
                let intersection = seed.start.max(range.0.start)..seed.end.min(range.0.end);
                seeds.push((intersection, range.1));
            }
            if seed.start < ranges.first().unwrap().0.start {
                let before = seed.start..ranges.first().unwrap().0.start;
                seeds.push((before, 0));
            }
            if seed.end > seeds.last().unwrap().0.end {
                let after = seeds.last().unwrap().0.end..seed.end;
                seeds.push((after, 0));
            }
        }
        seeds
            .into_iter()
            .map(|(range, offset)| (range.start + offset)..(range.end + offset))
            .collect()
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<i64>,
    maps: Vec<Map>,
}

impl Almanac {
    fn seeds(&self, seeds_as_range: bool) -> Vec<Range<i64>> {
        if seeds_as_range {
            self.seeds
                .chunks(2)
                .map(|seed| seed[0]..(seed[0] + seed[1]))
                .collect()
        } else {
            self.seeds.iter().map(|&seed| seed..(seed + 1)).collect()
        }
    }
    fn map(&self, seeds_as_range: bool) -> i64 {
        self.maps
            .iter()
            .fold(self.seeds(seeds_as_range), |seeds, map| {
                map.process_seeds(&seeds)
            })
            .iter()
            .map(|range| range.start)
            .min()
            .unwrap()
    }
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<&str>>();
        let (name, map) = lines.split_first().unwrap();
        let name = name.replace(" map:", "");
        let ranges: Vec<(Range<i64>, i64)> = map
            .iter()
            .map(|m| {
                let numbers: Vec<i64> = m
                    .split_ascii_whitespace()
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect();
                let (destination, source_start, source_len) = (numbers[0], numbers[1], numbers[2]);
                let source_end = source_start + source_len;
                let offset: i64 = destination - source_start;
                (source_start..source_end, offset)
            })
            .collect();
        Ok(Map { name, ranges })
    }
}

impl FromStr for Almanac {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let maps: Vec<&str> = s.split("\n\n").collect();
        let (seeds, maps) = maps.split_first().unwrap();
        let seeds = seeds
            .replace("seeds: ", "")
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let maps: Vec<Map> = maps.iter().map(|map| map.parse::<Map>().unwrap()).collect();
        Ok(Almanac { seeds, maps })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static str {
        "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
    }

    #[test]
    fn test_map_from_str() {
        assert_eq!(
            Map {
                name: String::from("seed-to-soil"),
                ranges: vec![((98..100), -48), ((50..98), 2)]
            },
            "seed-to-soil map:
50 98 2
52 50 48"
                .parse()
                .unwrap()
        )
    }

    #[test]
    fn test_map_process_seed() {
        assert_eq!(
            vec![81..82],
            Map {
                name: String::from("seed-to-soil"),
                ranges: vec![((98..100), -48), ((50..98), 2)]
            }
            .process_seed(&(79..80))
        );
    }

    #[test]
    fn test_pt1_example() {
        assert_eq!("35".to_string(), Day {}.pt1(example_input()))
    }

    #[test]
    fn test_pt1() {
        assert_eq!("825516882".to_string(), Day {}.pt1(input()))
    }

    #[test]
    fn test_pt2_example() {
        assert_eq!("46".to_string(), Day {}.pt2(example_input()))
    }

    #[test]
    fn test_pt2() {
        assert_eq!("136096660".to_string(), Day {}.pt2(input()))
    }
}
