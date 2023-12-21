use num::integer::lcm;
use std::{collections::HashMap, fmt::Debug};

use crate::problem::Solver;
pub struct Day {}

impl Solver for Day {
    fn pt1(&self, input: &str) -> String {
        let mut machines = Machines::from(input);
        let signals: (usize, usize) = (0..1000).fold((0_usize, 0_usize), |acc, _| {
            let signals = machines.broadcast(PulseType::Low, "broadcaster");
            signals
                .iter()
                .fold(acc, |acc, (_sender, pulse_type)| match pulse_type {
                    PulseType::High => (acc.0, acc.1 + 1),
                    PulseType::Low => (acc.0 + 1, acc.1),
                })
        });

        let signals = signals.0 * signals.1;
        format!("{signals}")
    }
    fn pt2(&self, input: &str) -> String {
        let mut machines = Machines::from(input);
        let vr_inputs = ["pq", "fg", "dk", "fm"];
        let vr_inputs: [usize; 4] =
            vr_inputs.map(|i| machines.0.iter().position(|m| m.label() == i).unwrap());
        let mut cycles: Vec<usize> = vec![];

        for i in 1.. {
            let signals = machines.broadcast(PulseType::Low, "broadcaster");

            for (from, pulse_type) in signals {
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

type Sent = (usize, PulseType);
type Signal = (usize, String, PulseType, Vec<Option<usize>>);

#[derive(Debug)]
struct Machines(Vec<Box<dyn Module>>);
impl Machines {
    fn init_conjunctions(&mut self) {
        let mut input_output: Vec<(usize, String, usize)> = self
            .0
            .iter()
            .enumerate()
            .flat_map(|(i, m)| {
                m.outputs()
                    .into_iter()
                    .flatten()
                    .map(|output| (i, m.label(), output))
                    .collect::<Vec<(usize, String, usize)>>()
            })
            .collect();
        input_output.sort_unstable();
        input_output.dedup();

        for (_sender_index, sender_label, receiver_index) in input_output {
            let receiver = self.0.get_mut(receiver_index).unwrap();
            receiver.add_input(&sender_label);
        }
    }
    fn module(&mut self, label: &str) -> Option<usize> {
        self.0.iter_mut().position(|m| m.label() == label)
    }
    fn broadcast(&mut self, pulse_type: PulseType, to: &str) -> Vec<Sent> {
        let to = self.module(to).unwrap();
        let mut sent: Vec<Sent> = vec![];
        let signal: Signal = (1337, String::from("button"), pulse_type, vec![Some(to)]);
        let mut all_signals: Vec<Vec<Signal>> = vec![vec![signal]];

        while !all_signals.is_empty() {
            let signals = all_signals.remove(0);
            let mut to_send: Vec<Signal> = vec![];
            for (sender_index, sender_label, signal, receivers) in signals {
                for receiver in receivers {
                    if let Some(receiver) = receiver {
                        let receiver_module = self.0.get_mut(receiver).unwrap();
                        if let Some(output) = receiver_module.receive(signal, &sender_label) {
                            to_send.push((receiver, output.0, output.1, output.2));
                        }
                    }
                    sent.push((sender_index, signal));
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
    fn add_input(&mut self, input: &str);
    fn label(&self) -> String;
    fn outputs(&self) -> Vec<Option<usize>>;
    fn receive(
        &mut self,
        signal: PulseType,
        sender: &str,
    ) -> Option<(String, PulseType, Vec<Option<usize>>)>;
}

#[derive(Debug)]
struct Broadcaster {
    label: String,
    outputs: Vec<Option<usize>>,
}
#[derive(Debug)]
struct FlipFlop {
    label: String,
    outputs: Vec<Option<usize>>,
    state: bool,
}
#[derive(Debug)]
struct Conjunction {
    label: String,
    outputs: Vec<Option<usize>>,
    remembered: Vec<(String, PulseType)>,
}

impl Module for Broadcaster {
    fn add_input(&mut self, _input: &str) {}
    fn label(&self) -> String {
        self.label.clone()
    }
    fn outputs(&self) -> Vec<Option<usize>> {
        self.outputs.clone()
    }
    fn receive(
        &mut self,
        signal: PulseType,
        _sender: &str,
    ) -> Option<(String, PulseType, Vec<Option<usize>>)> {
        Some((self.label(), signal, self.outputs.clone()))
    }
}
impl Module for FlipFlop {
    fn add_input(&mut self, _input: &str) {}
    fn label(&self) -> String {
        self.label.clone()
    }
    fn outputs(&self) -> Vec<Option<usize>> {
        self.outputs.clone()
    }
    fn receive(
        &mut self,
        signal: PulseType,
        _sender: &str,
    ) -> Option<(String, PulseType, Vec<Option<usize>>)> {
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
    fn add_input(&mut self, input: &str) {
        self.remembered.push((String::from(input), PulseType::Low));
    }
    fn label(&self) -> String {
        self.label.clone()
    }
    fn outputs(&self) -> Vec<Option<usize>> {
        self.outputs.clone()
    }
    fn receive(
        &mut self,
        signal: PulseType,
        sender: &str,
    ) -> Option<(String, PulseType, Vec<Option<usize>>)> {
        let remembered = self
            .remembered
            .iter_mut()
            .find(|(l, _)| l == sender)
            .unwrap();

        remembered.1 = signal;

        Some((
            self.label(),
            if self.remembered.iter().all(|(_, s)| *s == PulseType::High) {
                PulseType::Low
            } else {
                PulseType::High
            },
            self.outputs.clone(),
        ))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum PulseType {
    High,
    Low,
}

impl From<&str> for Machines {
    fn from(value: &str) -> Self {
        let labels_to_indices: HashMap<&str, usize> = value
            .lines()
            .enumerate()
            .map(|(i, l)| {
                let type_index = l
                    .chars()
                    .position(|c| c == '&' || c == '%')
                    .map_or(0, |p| p + 1);
                let (label, _outputs) = l[type_index..]
                    .split_once(" -> ")
                    .unwrap_or_else(|| panic!("Not -> found: {}", &l[type_index..]));
                (label, i)
            })
            .collect();

        let modules: Vec<Box<dyn Module>> = value
            .lines()
            .map(|l| module_from(l, &labels_to_indices))
            .collect();

        let mut result = Self(modules);
        result.init_conjunctions();
        result
    }
}

fn module_from(value: &str, labels_to_indices: &HashMap<&str, usize>) -> Box<dyn Module> {
    let type_index = value
        .chars()
        .position(|c| c == '&' || c == '%')
        .map_or(0, |p| p + 1);
    let (label, outputs) = value[type_index..]
        .split_once(" -> ")
        .unwrap_or_else(|| panic!("Not -> found: {}", &value[type_index..]));
    let label = String::from(label);
    let outputs: Vec<Option<usize>> = outputs
        .split(", ")
        .map(|label| labels_to_indices.get(label).copied())
        .collect();

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
