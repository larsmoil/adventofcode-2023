use std::fmt::Display;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Coordinate(pub isize, pub isize);

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("({}, {})", self.0, self.1))
    }
}

pub fn shoelace(points: &[Coordinate]) -> isize {
    let area = points
        .windows(2)
        .map(|w| w[0].0 * w[1].1 - w[0].1 * w[1].0)
        .sum::<isize>()
        .abs()
        / 2;

    let perimeter: isize = points
        .windows(2)
        .map(|w| {
            let dx: isize = w[1].0.abs_diff(w[0].0).try_into().unwrap();
            let dy: isize = w[1].1.abs_diff(w[0].1).try_into().unwrap();
            dx + dy
        })
        .sum::<isize>();

    area + perimeter / 2 + 1
}
