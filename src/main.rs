extern crate clap;

use std::thread;

use clap::{App, Arg};

mod day01;

fn main() {
    let matches = App::new("AOC2020")
        .arg(
            Arg::with_name("DAY")
                .required(true)
                .index(1)
                .help("Day number to run"),
        )
        .get_matches();

    let day = matches.value_of("DAY").unwrap().parse().unwrap();

    match day {
        0 => {
            let mut threads = Vec::with_capacity(25);
            threads.push(thread::spawn(|| {
                let input = day01::load_input();
                println!("Day 1 Part 1 Solution {:?}", day01::part1(&input));
                println!("Day 1 Part 2 Solution {:?}", day01::part2(&input));
            }));
        }
        _ => println!("Day {} not yet implemented", day),
    }
}
