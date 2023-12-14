use std::time::Instant;

use crate::problem::Solver;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod problem;

fn main() {
    let now = Instant::now();
    for day in 1..=11 {
        let (d, inp): (&dyn Solver, &str) = match day {
            1 => (&day01::Day {}, day01::input()),
            2 => (&day02::Day {}, day02::input()),
            3 => (&day03::Day {}, day03::input()),
            4 => (&day04::Day {}, day04::input()),
            5 => (&day05::Day {}, day05::input()),
            6 => (&day06::Day {}, day06::input()),
            7 => (&day07::Day {}, day07::input()),
            8 => (&day08::Day {}, day08::input()),
            9 => (&day09::Day {}, day09::input()),
            10 => (&day10::Day {}, day10::input()),
            11 => (&day11::Day {}, day11::input()),
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
