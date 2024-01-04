use rustworkx_core::connectivity::stoer_wagner_min_cut;
use rustworkx_core::petgraph::graphmap::UnGraphMap;
use std::collections::HashSet;

use crate::problem::Solver;
pub struct Day {}

impl Solver for Day {
    fn pt1(&self, input: &str) -> String {
        let wiring_diagram = WiringDiagram::from(input);
        let divided = wiring_diagram.divide();
        format!("{}", divided.0 * divided.1)
    }
    fn pt2(&self, _input: &str) -> String {
        String::from("Merry Christmas!")
    }
}

#[derive(Debug)]
struct WiringDiagram<'a>(&'a str);
impl<'a> WiringDiagram<'a> {
    fn divide(&self) -> (usize, usize) {
        let mut hashset = HashSet::new();
        for line in self.0.lines() {
            let (from, to) = line.split_once(": ").unwrap();
            to.split(' ').for_each(|t| {
                hashset.insert((from, t));
            });
        }

        let graph = UnGraphMap::<&str, ()>::from_edges(hashset);
        let (_min_cut, partition) = stoer_wagner_min_cut(&graph, |_| Result::Ok::<_, ()>(1))
            .unwrap()
            .unwrap();

        let group1 = partition.len();
        let group2 = graph.node_count() - group1;

        (group1.max(group2), group1.min(group2))
    }
}

impl<'a> From<&'a str> for WiringDiagram<'a> {
    fn from(value: &'a str) -> Self {
        Self(value)
    }
}

pub(crate) fn input() -> &'static str {
    include_str!("day25-input.txt").trim()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static str {
        "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr"
    }

    #[test]
    fn test_pt1_example() {
        let wiring_diagram = WiringDiagram::from(example_input());
        let divided = wiring_diagram.divide();
        assert_eq!((9, 6), divided);
        assert_eq!(String::from("54"), Day {}.pt1(example_input()));
    }

    #[test]
    fn test_pt1() {
        let wiring_diagram = WiringDiagram::from(input());
        let divided = wiring_diagram.divide();
        let product = divided.0 * divided.1;
        assert!(
            product > 12168,
            "it should be greater than 12168, got: '{product}'"
        );
        assert_eq!(String::from("583338"), Day {}.pt1(input()));
    }

    #[test]
    fn test_pt2_example() {
        assert_eq!(
            String::from("Merry Christmas!"),
            Day {}.pt2(example_input())
        );
    }

    #[test]
    fn test_pt2() {
        assert_eq!(String::from("Merry Christmas!"), Day {}.pt2(input()));
    }
}
