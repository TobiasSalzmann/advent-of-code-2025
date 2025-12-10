#![feature(let_chains)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
#[allow(dead_code)]
mod util;
mod day10;

extern crate core;
extern crate dotenv;

use std::env;
use std::time::Instant;

use clap::{Parser, arg};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Day to run
    #[arg(value_parser = clap::value_parser!(i32).range(0..=25))]
    day: i32,

    /// Use test file instead (resources/day<day>.test.txt)
    #[arg(long, env, default_value_t = false)]
    test: bool,

    /// Measure execution time
    #[arg(short, long, env, default_value_t = false)]
    time: bool,
}
fn main() {
    let args = Args::parse();

    if args.test {
        unsafe {
            env::set_var("TEST", "true");
        }
    }

    if args.day == 0 {
        let start = Instant::now();
        for d in 1..=25 {
            run(d, args.time);
            println!();
        }
        if args.time {
            println!("Total Time: {} ms", start.elapsed().as_millis());
        }
    } else {
        run(args.day, args.time)
    }
}

fn run(day: i32, time: bool) {
    let start = Instant::now();
    match day {
        1 => day1::main(),
        2 => day2::main(),
        3 => day3::main(),
        4 => day4::main(),
        5 => day5::main(),
        6 => day6::main(),
        7 => day7::main(),
        8 => day8::main(),
        9 => day9::main(),
        10 => day10::main(),
        other => {
            println!("Day {} not yet implemented 😅", other)
        }
    }
    let duration = start.elapsed();
    if time {
        if duration.as_millis() < 3 {
            println!("Time: {} μs", duration.as_micros())
        } else {
            println!("Time: {} ms", duration.as_millis())
        }
    }
}
