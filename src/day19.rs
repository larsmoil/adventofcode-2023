use std::{collections::HashMap, ops::RangeInclusive};

use crate::problem::Solver;
pub struct Day {}

impl Solver for Day {
    fn pt1(&self, input: &str) -> String {
        let workflows = Workflows::from(input);
        let accepted = workflows.sort();
        format!("{accepted}")
    }
    fn pt2(&self, input: &str) -> String {
        let workflows = Workflows::from(input);
        let processed = workflows.process_range(
            Part {
                x: 1..=4000,
                m: 1..=4000,
                a: 1..=4000,
                s: 1..=4000,
            },
            &ProcessResult::Next("in"),
            |part| part.a.len() * part.m.len() * part.s.len() * part.x.len(),
        );
        format!("{processed}")
    }
}

impl Part {
    fn is_empty(&self) -> bool {
        self.a.is_empty() || self.m.is_empty() || self.s.is_empty() || self.x.is_empty()
    }
    fn with_attr(&self, attr: &Attr, value: RangeInclusive<u16>) -> Part {
        match attr {
            Attr::A => Part {
                a: value,
                m: self.m.clone(),
                s: self.s.clone(),
                x: self.x.clone(),
            },
            Attr::M => Part {
                a: self.a.clone(),
                m: value,
                s: self.s.clone(),
                x: self.x.clone(),
            },
            Attr::S => Part {
                a: self.a.clone(),
                m: self.m.clone(),
                s: value,
                x: self.x.clone(),
            },
            Attr::X => Part {
                a: self.a.clone(),
                m: self.m.clone(),
                s: self.s.clone(),
                x: value,
            },
        }
    }
}

impl<'a> Workflows<'a> {
    fn sort(&self) -> usize {
        self.1
            .clone()
            .into_iter()
            .map(|part| {
                self.process_range(part, &ProcessResult::Next("in"), |p| {
                    let score =
                        if p.a.is_empty() || p.m.is_empty() || p.s.is_empty() || p.x.is_empty() {
                            0
                        } else {
                            p.a.start() + p.m.start() + p.s.start() + p.x.start()
                        };
                    usize::try_from(score).unwrap()
                })
            })
            .sum::<usize>()
    }

    fn process_range(&self, part: Part, next: &ProcessResult, score: fn(&Part) -> usize) -> usize {
        match next {
            ProcessResult::Accept => score(&part),
            ProcessResult::Reject => 0,
            ProcessResult::Next(next) => {
                let workflow = self.0.get(next).unwrap();
                let mut sum = 0;
                let mut part = part;
                for rule in &workflow.1 {
                    match rule {
                        Rule::Pass(target) => {
                            sum += self.process_range(part, target, score);
                            break;
                        }
                        Rule::Conditional(attr, operator, threshold, to) => {
                            let value = match attr {
                                Attr::A => &part.a,
                                Attr::M => &part.m,
                                Attr::S => &part.s,
                                Attr::X => &part.x,
                            };
                            let adjusted_threshold: u16 = match operator {
                                Operator::LessThan => *threshold - 1,
                                Operator::GreaterThan => *threshold + 1,
                            };
                            if value.contains(&adjusted_threshold) {
                                let matching = match operator {
                                    Operator::LessThan => *value.start()..=adjusted_threshold,
                                    Operator::GreaterThan => adjusted_threshold..=*value.end(),
                                };
                                if !matching.is_empty() {
                                    let matching = part.with_attr(attr, matching);
                                    sum += self.process_range(matching, to, score);

                                    let non_matching = match operator {
                                        Operator::LessThan => *threshold..=*value.end(),
                                        Operator::GreaterThan => *value.start()..=*threshold,
                                    };
                                    let non_matching = part.with_attr(attr, non_matching);
                                    if non_matching.is_empty() {
                                        break;
                                    }

                                    part = non_matching;
                                }
                            } else {
                                match operator {
                                    Operator::LessThan => {
                                        if value.end() < threshold {
                                            sum += self.process_range(part.clone(), to, score);
                                            break;
                                        }
                                    }
                                    Operator::GreaterThan => {
                                        if value.start() > threshold {
                                            sum += self.process_range(part.clone(), to, score);
                                            break;
                                        }
                                    }
                                };
                            }
                        }
                    }
                }

                sum
            }
        }
    }
}

#[derive(Clone, Debug)]
enum Rule<'a> {
    Conditional(Attr, Operator, u16, ProcessResult<'a>),
    Pass(ProcessResult<'a>),
}

#[derive(Clone, Debug)]
enum ProcessResult<'a> {
    Accept,
    Reject,
    Next(&'a str),
}
impl<'a> From<&'a str> for ProcessResult<'a> {
    fn from(value: &'a str) -> Self {
        match value {
            "A" => Self::Accept,
            "R" => Self::Reject,
            v => Self::Next(v),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Attr {
    A,
    M,
    S,
    X,
}
impl From<&str> for Attr {
    fn from(value: &str) -> Self {
        match value {
            "a" => Attr::A,
            "m" => Attr::M,
            "s" => Attr::S,
            "x" => Attr::X,
            &_ => todo!("Unknown operator: {}", value),
        }
    }
}

pub(crate) fn input() -> &'static str {
    include_str!("day19-input.txt").trim()
}

struct Workflows<'a>(HashMap<&'a str, Workflow<'a>>, Vec<Part>);
impl<'a> From<&'a str> for Workflows<'a> {
    fn from(value: &'a str) -> Self {
        let (workflows, parts) = value.split_once("\n\n").unwrap();
        let workflows: Vec<Workflow> = workflows.lines().map(Workflow::from).collect();
        let workflows: HashMap<&'a str, Workflow> =
            workflows.into_iter().map(|w| (w.0, w)).collect();
        Self(workflows, parts.lines().map(Part::from).collect())
    }
}

#[derive(Clone, Debug)]
struct Workflow<'a>(&'a str, Vec<Rule<'a>>);
impl<'a> From<&'a str> for Workflow<'a> {
    fn from(value: &'a str) -> Self {
        let (name, rules) =
            value[..value.len() - 1].split_at(value.chars().position(|c| c == '{').unwrap());
        let rules = rules[1..].split(',').map(Rule::from).collect();
        Self(name, rules)
    }
}

#[derive(Clone, Debug)]
enum Operator {
    LessThan,
    GreaterThan,
}
impl From<&str> for Operator {
    fn from(value: &str) -> Self {
        match value {
            ">" => Operator::GreaterThan,
            "<" => Operator::LessThan,
            &_ => todo!("Unknown operator: {}", value),
        }
    }
}

impl<'a> From<&'a str> for Rule<'a> {
    fn from(value: &'a str) -> Self {
        if value.contains(':') {
            let attr = &value[0..1];
            let attr = Attr::from(attr);
            let operator = &value[1..2];
            let operator = Operator::from(operator);
            let (value, target) = value[2..].split_once(':').unwrap();
            let value = value.parse().unwrap();
            let target: ProcessResult = ProcessResult::from(target);
            Self::Conditional(attr, operator, value, target)
        } else {
            let target: ProcessResult = ProcessResult::from(value);
            Self::Pass(target)
        }
    }
}

#[derive(Clone, Debug)]
struct Part {
    x: RangeInclusive<u16>,
    m: RangeInclusive<u16>,
    a: RangeInclusive<u16>,
    s: RangeInclusive<u16>,
}
impl From<&str> for Part {
    fn from(value: &str) -> Self {
        let attrs: HashMap<Attr, u16> = value[1..value.len() - 1]
            .split(',')
            .map(|a| a.split_once('=').unwrap())
            .map(|(attr, value)| (Attr::from(attr), value.parse::<u16>().unwrap()))
            .collect();
        Self {
            x: *attrs.get(&Attr::X).unwrap()..=*attrs.get(&Attr::X).unwrap(),
            m: *attrs.get(&Attr::M).unwrap()..=*attrs.get(&Attr::M).unwrap(),
            a: *attrs.get(&Attr::A).unwrap()..=*attrs.get(&Attr::A).unwrap(),
            s: *attrs.get(&Attr::S).unwrap()..=*attrs.get(&Attr::S).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static str {
        "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"
    }

    #[test]
    fn test_pt1_example() {
        assert_eq!("19114".to_string(), Day {}.pt1(example_input()));
    }

    #[test]
    fn test_pt1() {
        assert_eq!("409898".to_string(), Day {}.pt1(input()));
    }

    #[test]
    fn test_pt2_example() {
        assert_eq!("167409079868000".to_string(), Day {}.pt2(example_input()));
    }

    #[test]
    fn test_pt2() {
        assert_eq!("113057405770956".to_string(), Day {}.pt2(input()))
    }
}
