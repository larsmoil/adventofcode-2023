use std::fmt::Display;
use std::ops::{Add, Index, IndexMut};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Coordinate<T>(pub T, pub T);

pub type Direction = u8;
pub const UP: u8 = 0b1000;
pub const RIGHT: u8 = 0b0100;
pub const DOWN: u8 = 0b0010;
pub const LEFT: u8 = 0b0001;
pub const OFFSETS: [Direction; 4] = [UP, RIGHT, DOWN, LEFT];

impl<T: Add<Output = T>> Add for Coordinate<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl Add<Coordinate<isize>> for Coordinate<usize> {
    type Output = Self;
    fn add(self, other: Coordinate<isize>) -> Self::Output {
        Self(
            self.0.checked_add_signed(other.0).unwrap(),
            self.1.checked_add_signed(other.1).unwrap(),
        )
    }
}

impl Add<Direction> for Coordinate<usize> {
    type Output = Self;
    fn add(self, other: Direction) -> Self::Output {
        let (dx, dy) = match other {
            UP => (0, -1),
            RIGHT => (1, 0),
            DOWN => (0, 1),
            LEFT => (-1, 0),
            _ => panic!("Unhandled direction: {other}"),
        };
        Self(
            self.0.checked_add_signed(dx).unwrap(),
            self.1.checked_add_signed(dy).unwrap(),
        )
    }
}

impl Display for Coordinate<isize> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("({}, {})", self.0, self.1))
    }
}

impl Display for Coordinate<usize> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("({}, {})", self.0, self.1))
    }
}

pub fn shoelace(points: &[Coordinate<isize>]) -> isize {
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

pub struct Grid<T> {
    pub points: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid<T> {
    pub fn coord(&self, index: usize) -> Coordinate<usize> {
        Coordinate(index % self.width, index / self.width)
    }
}

impl From<&str> for Grid<u8> {
    fn from(value: &str) -> Self {
        let raw: Vec<_> = value.lines().map(str::as_bytes).collect();
        let width = raw[0].len();
        let height = raw.len();
        let mut points = Vec::with_capacity(width * height);
        raw.iter().for_each(|slice| points.extend_from_slice(slice));
        Self {
            points,
            width,
            height,
        }
    }
}

impl<T> Index<&Coordinate<isize>> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, coordinate: &Coordinate<isize>) -> &Self::Output {
        &self.points[usize::try_from(coordinate.0).unwrap()
            + self.width * usize::try_from(coordinate.1).unwrap()]
    }
}

impl<T> Index<&Coordinate<usize>> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, coordinate: &Coordinate<usize>) -> &Self::Output {
        &self.points[coordinate.0 + self.width * coordinate.1]
    }
}

impl<T> IndexMut<&Coordinate<usize>> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, coordinate: &Coordinate<usize>) -> &mut Self::Output {
        &mut self.points[self.width * coordinate.1 + coordinate.0]
    }
}
