use num::integer::lcm;
use std::fmt::Debug;

use crate::problem::Solver;
pub struct Day {}

impl Solver for Day {
    fn pt1(&self, input: &str) -> String {
        let mut machines = Machines::from(input);
        let signals: (usize, usize) = (0..1000).fold((0_usize, 0_usize), |mut acc, _| {
            let signals = machines.broadcast(PulseType::Low, String::from("broadcaster"));
            for (_, signal, _) in signals {
                acc = match signal {
                    PulseType::High => (acc.0, acc.1 + 1),
                    PulseType::Low => (acc.0 + 1, acc.1),
                }
            }
            acc
        });

        let signals = signals.0 * signals.1;
        format!("{signals}")
    }
    fn pt2(&self, input: &str) -> String {
        let mut machines = Machines::from(input);
        let vr_inputs = [
            String::from("pq"),
            String::from("fg"),
            String::from("dk"),
            String::from("fm"),
        ];
        let mut cycles: Vec<usize> = vec![];

        for i in 1.. {
            let signals = machines.broadcast(PulseType::Low, String::from("broadcaster"));

            for (from, pulse_type, _to) in signals {
                if vr_inputs.contains(&from) && pulse_type == PulseType::High {
                    cycles.push(i);
                }
            }

            if cycles.len() == vr_inputs.len() {
                let lcm = cycles.into_iter().reduce(lcm).unwrap();
                let cycles = lcm;
                return format!("{cycles}");
            }

            if i > 10_000 {
                break;
            }
        }
        panic!("out of loop!");
    }
}

pub(crate) fn input() -> &'static str {
    include_str!("day20-input.txt").trim()
}

#[derive(Debug)]
struct Machines(Vec<Box<dyn Module>>);
impl Machines {
    fn module(&mut self, label: &str) -> Option<&mut Box<dyn Module>> {
        self.0.iter_mut().find(|m| m.label() == label)
    }
    fn broadcast(&mut self, pulse_type: PulseType, to: String) -> Vec<(String, PulseType, String)> {
        let mut sent: Vec<(String, PulseType, String)> = vec![];
        let signal: (String, PulseType, Vec<String>) =
            (String::from("button"), pulse_type, vec![to]);
        let mut all_signals: Vec<Vec<(String, PulseType, Vec<String>)>> = vec![vec![signal]];

        while !all_signals.is_empty() {
            let signals = all_signals.remove(0);
            let mut to_send: Vec<(String, PulseType, Vec<String>)> = vec![];
            for (sender, signal, receivers) in signals {
                for receiver in receivers {
                    sent.push((sender.clone(), signal, receiver.clone()));
                    if let Some(receiver) = self.module(&receiver) {
                        if let Some(output) = receiver.receive(signal, sender.clone()) {
                            to_send.push(output);
                        }
                    }
                }
            }
            if !to_send.is_empty() {
                all_signals.push(to_send);
            }
        }
        sent
    }
}

trait Module: Debug {
    fn conjunction(&self) -> Option<&Conjunction>;
    fn conjunction_mut(&mut self) -> Option<&mut Conjunction>;
    fn label(&self) -> String;
    fn outputs(&self) -> Vec<String>;
    fn receive(
        &mut self,
        signal: PulseType,
        sender: String,
    ) -> Option<(String, PulseType, Vec<String>)>;
}

#[derive(Debug)]
struct Broadcaster {
    label: String,
    outputs: Vec<String>,
}
#[derive(Debug)]
struct FlipFlop {
    label: String,
    outputs: Vec<String>,
    state: bool,
}
#[derive(Debug)]
struct Conjunction {
    label: String,
    outputs: Vec<String>,
    remembered: Vec<(String, PulseType)>,
}

impl Module for Broadcaster {
    fn conjunction(&self) -> Option<&Conjunction> {
        None
    }
    fn conjunction_mut(&mut self) -> Option<&mut Conjunction> {
        None
    }
    fn label(&self) -> String {
        self.label.clone()
    }
    fn outputs(&self) -> Vec<String> {
        self.outputs.clone()
    }
    fn receive(
        &mut self,
        signal: PulseType,
        _sender: String,
    ) -> Option<(String, PulseType, Vec<String>)> {
        Some((
            self.label(),
            signal,
            self.outputs.clone().into_iter().map(String::from).collect(),
        ))
    }
}
impl Module for FlipFlop {
    fn conjunction(&self) -> Option<&Conjunction> {
        None
    }
    fn conjunction_mut(&mut self) -> Option<&mut Conjunction> {
        None
    }
    fn label(&self) -> String {
        self.label.clone()
    }
    fn outputs(&self) -> Vec<String> {
        self.outputs.clone()
    }
    fn receive(
        &mut self,
        signal: PulseType,
        _sender: String,
    ) -> Option<(String, PulseType, Vec<String>)> {
        match signal {
            PulseType::High => None,
            PulseType::Low => {
                let signal = if self.state {
                    PulseType::Low
                } else {
                    PulseType::High
                };
                self.state = !self.state;
                Some((self.label(), signal, self.outputs.clone()))
            }
        }
    }
}
impl Module for Conjunction {
    fn conjunction(&self) -> Option<&Conjunction> {
        Some(self)
    }
    fn conjunction_mut(&mut self) -> Option<&mut Conjunction> {
        Some(self)
    }
    fn label(&self) -> String {
        self.label.clone()
    }
    fn outputs(&self) -> Vec<String> {
        self.outputs.clone()
    }
    fn receive(
        &mut self,
        signal: PulseType,
        sender: String,
    ) -> Option<(String, PulseType, Vec<String>)> {
        let remembered = if let Some(i) = self.remembered.iter_mut().position(|(l, _)| l == &sender)
        {
            &mut self.remembered[i]
        } else {
            self.remembered.insert(0, (sender.clone(), PulseType::Low));
            &mut self.remembered[0]
        };

        remembered.1 = signal;

        Some((
            self.label(),
            if self.remembered.iter().all(|(_, s)| *s == PulseType::High) {
                PulseType::Low
            } else {
                PulseType::High
            },
            self.outputs.clone().into_iter().map(String::from).collect(),
        ))
    }
}

impl Conjunction {
    fn add_input(&mut self, input: String) {
        self.remembered.push((input, PulseType::Low));
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum PulseType {
    High,
    Low,
}

impl From<&str> for Machines {
    fn from(value: &str) -> Self {
        let mut modules: Vec<Box<dyn Module>> = value.lines().map(module_from).collect();
        let mut connections: Vec<(String, String)> = vec![];
        for module in &modules {
            let label = module.label();
            let outputs = module.outputs();
            for output in outputs {
                let output = modules.iter().find(|m| m.label() == output);
                if let Some(output) = output {
                    if let Some(conjunction) = output.conjunction() {
                        connections.push((label.clone(), conjunction.label()));
                    }
                }
            }
        }
        for (from, to) in connections {
            let to = modules.iter_mut().find(|m| m.label() == to).unwrap();
            let conjunction = to.conjunction_mut().expect("should be a conjunction");
            conjunction.add_input(from);
        }
        Self(modules)
    }
}

fn module_from(value: &str) -> Box<dyn Module> {
    let type_index = value
        .chars()
        .position(|c| c == '&' || c == '%')
        .map_or(0, |p| p + 1);
    let (label, outputs) = value[type_index..]
        .split_once(" -> ")
        .unwrap_or_else(|| panic!("Not -> found: {}", &value[type_index..]));
    let label = String::from(label);
    let outputs: Vec<String> = outputs.split(", ").map(String::from).collect();

    let result: Box<dyn Module> = match &value[..type_index] {
        "&" => Box::new(Conjunction {
            label,
            outputs,
            remembered: vec![],
        }),
        "%" => Box::new(FlipFlop {
            label,
            outputs,
            state: false,
        }),
        &_ => Box::new(Broadcaster { label, outputs }),
    };
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt1_example_1() {
        let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
        assert_eq!("32000000".to_string(), Day {}.pt1(input));
    }

    #[test]
    fn test_pt1_example_2() {
        let input = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
        assert_eq!("11687500".to_string(), Day {}.pt1(input));
    }

    #[test]
    fn test_pt1() {
        assert_eq!("712543680".to_string(), Day {}.pt1(input()));
    }

    #[test]
    fn test_pt2() {
        assert_eq!("238920142622879".to_string(), Day {}.pt2(input()))
    }
}
