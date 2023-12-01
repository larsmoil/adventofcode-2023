use std::time::Instant;

use crate::problem::Solver;

mod day01;
mod problem;

fn main() {
    let now = Instant::now();
    for day in 1..=1 {
        let (d, inp): (&dyn Solver, &str) = match day {
            1 => (&day01::Day {}, day01::input()),
            _ => panic!("Invalid day!"),
        };
        let now = Instant::now();
        println!(
            "day{:02} - pt1: {:>15} ({:.2?})",
            day,
            d.pt1(inp),
            now.elapsed()
        );

        let now = Instant::now();
        println!(
            "day{:02} - pt2: {:>15} ({:.2?})",
            day,
            d.pt2(inp),
            now.elapsed()
        );
    }
    println!("total: {:.2?}", now.elapsed());
}
