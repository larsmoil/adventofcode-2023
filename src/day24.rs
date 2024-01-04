use std::ops::{Add, Div, Mul, RangeInclusive, Sub};

use crate::problem::Solver;
pub struct Day {}

impl Solver for Day {
    fn pt1(&self, input: &str) -> String {
        let hailstones: Hailstones<i128> = Hailstones::from(input);
        format!(
            "{}",
            hailstones.cross(200_000_000_000_000..=400_000_000_000_000)
        )
    }
    fn pt2(&self, input: &str) -> String {
        let hailstones: Hailstones<i128> = Hailstones::from(input);
        let rock_position = hailstones.rock_position();

        format!("{rock_position}")
    }
}

pub(crate) fn input() -> &'static str {
    include_str!("day24-input.txt").trim()
}

impl Hailstones<i128> {
    fn cross(&self, test_area: RangeInclusive<i128>) -> usize {
        let mut crosses = 0;

        for (i, hailstone) in self.0.iter().enumerate() {
            for other in self.0.iter().skip(i + 1) {
                if let Some((cross_x, cross_y, cross_time_1, cross_time_2)) = hailstone.cross(other)
                {
                    if test_area.contains(&cross_x)
                        && test_area.contains(&cross_y)
                        && cross_time_1 > 0
                        && cross_time_2 > 0
                    {
                        crosses += 1;
                    }
                }
            }
        }

        crosses
    }
    fn rock_position(&self) -> i128 {
        // Threes stones, relative
        let h0 = self.0[0];
        let h1 = self.0[1] - h0;
        let h2 = self.0[2] - h0;

        // Find plane for seond and third hailstone
        let q = h1.velocity.cross(h1.position).gcd();
        let r = h2.velocity.cross(h2.position).gcd();
        let s = q.cross(r).gcd();

        let t1 = (h1.position.y * s.x - h1.position.x * s.y)
            / (h1.velocity.x * s.y - h1.velocity.y * s.x);
        let t2 = (h2.position.y * s.x - h2.position.x * s.y)
            / (h2.velocity.x * s.y - h2.velocity.y * s.x);
        assert!(t1 != t2);

        let a = h0.position.add(h1.position).sum();
        let b = h0.position.add(h2.position).sum();
        let c = h1.velocity.sub(h2.velocity).sum();

        (t2 * a - t1 * b + t2 * t1 * c) / (t2 - t1)
    }
}

impl Vector3d<i128> {
    fn cross(&self, other: Self) -> Self {
        let x = self.y * other.z - self.z * other.y;
        let y = self.z * other.x - self.x * other.z;
        let z = self.x * other.y - self.y * other.x;
        Self { x, y, z }
    }
    fn gcd(&self) -> Self {
        let gcd = [self.x, self.y, self.z].into_iter().reduce(gcd).unwrap();
        let x = self.x / gcd;
        let y = self.y / gcd;
        let z = self.z / gcd;
        Self { x, y, z }
    }
    fn sum(self) -> i128 {
        self.x + self.y + self.z
    }
}
fn gcd(a: i128, b: i128) -> i128 {
    let (mut a, mut b) = (a, b);

    while b != 0 {
        (a, b) = (b, a % b);
    }

    a
}

struct Hailstones<T>(Vec<Hailstone<T>>);

#[derive(Clone, Copy, Debug)]
struct Hailstone<T> {
    position: Vector3d<T>,
    velocity: Vector3d<T>,
}
impl Hailstone<i128> {
    fn cross(&self, other: &Hailstone<i128>) -> Option<(i128, i128, i128, i128)> {
        if self.a() * other.b() == self.b() * other.a() {
            // parallel
            None
        } else {
            let x = (self.c() * other.b() - other.c() * self.b())
                / (self.a() * other.b() - other.a() * self.b());
            let y = (other.c() * self.a() - self.c() * other.a())
                / (self.a() * other.b() - other.a() * self.b());
            let t1 = (x - self.position.x) / self.velocity.x;
            let t2 = (x - other.position.x) / other.velocity.x;
            Some((x, y, t1, t2))
        }
    }
    fn a(&self) -> i128 {
        self.velocity.y
    }
    fn b(&self) -> i128 {
        -self.velocity.x
    }
    fn c(&self) -> i128 {
        self.velocity.y * self.position.x - self.velocity.x * self.position.y
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Vector3d<T> {
    x: T,
    y: T,
    z: T,
}

impl From<&str> for Hailstones<i128> {
    fn from(value: &str) -> Self {
        Self(value.lines().map(Hailstone::from).collect())
    }
}
impl From<&str> for Hailstone<i128> {
    fn from(value: &str) -> Self {
        let (position, velocity) = value
            .split_once(" @ ")
            .map(|(p, v)| (Vector3d::from(p), Vector3d::from(v)))
            .unwrap();

        Self { position, velocity }
    }
}
impl From<&str> for Vector3d<i128> {
    fn from(value: &str) -> Self {
        let xyz = value
            .split(',')
            .map(|n| {
                n.trim()
                    .parse::<i128>()
                    .unwrap_or_else(|_| panic!("Invalid digit in value '{value}': '{n}'"))
            })
            .collect::<Vec<_>>();
        Self {
            x: xyz[0],
            y: xyz[1],
            z: xyz[2],
        }
    }
}

impl Sub<Hailstone<i128>> for Hailstone<i128> {
    type Output = Self;
    fn sub(self, other: Hailstone<i128>) -> Self::Output {
        Self {
            position: self.position - other.position,
            velocity: self.velocity - other.velocity,
        }
    }
}

impl Add<Vector3d<i128>> for Vector3d<i128> {
    type Output = Self;
    fn add(self, other: Vector3d<i128>) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl Div<Vector3d<i128>> for Vector3d<i128> {
    type Output = Self;
    fn div(self, other: Vector3d<i128>) -> Self::Output {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}
impl Mul<i128> for Vector3d<i128> {
    type Output = Self;
    fn mul(self, other: i128) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}
impl Sub<Vector3d<i128>> for Vector3d<i128> {
    type Output = Self;
    fn sub(self, other: Vector3d<i128>) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static str {
        "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3"
    }

    #[test]
    fn test_pt1_example() {
        let lower_bound = 7_i128;
        let upper_bound = 27_i128;
        let hailstones = Hailstones::from(example_input());
        let h0 = hailstones.0[0];
        let h1 = hailstones.0[1];
        let h2 = hailstones.0[2];
        let h3 = hailstones.0[3];

        assert_eq!(Some((14, 15, 2, 4)), h0.cross(&h1));
        assert_eq!(Some((11, 16, 4, 4)), h0.cross(&h2));
        assert_eq!(Some((6, 19, 6, 6)), h0.cross(&h3));

        assert_eq!(2, hailstones.cross(lower_bound..=upper_bound));
    }

    #[test]
    fn test_pt1() {
        assert_eq!(String::from("15889"), Day {}.pt1(input()));
    }

    #[test]
    fn test_pt2_example() {
        assert_eq!(String::from("47"), Day {}.pt2(example_input()));
    }

    #[test]
    fn test_pt2() {
        assert_eq!(String::from("801386475216902"), Day {}.pt2(input()));
    }
}
