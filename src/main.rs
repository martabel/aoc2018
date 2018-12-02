extern crate clap;
use clap::{App, Arg};
mod day_one;
mod day_two;

fn main() {
    let matches = App::new("adventofcode")
        .version("0.1.0")
        .author("Martin Abel")
        .about("AOC 2018 in Rust")
        .arg(
            Arg::with_name("DAY")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("AOC Day"),
        ).arg(
            Arg::with_name("INPUT")
                .required(true)
                .takes_value(true)
                .index(2)
                .help("AOC Input"),
        ).get_matches();
    let day = matches.value_of("DAY").unwrap();
    let input = matches.value_of("INPUT").unwrap();
    println!("{} {}", day, input);
    match day {
        "1" => day_one::run(input),
        "2" => day_two::run(input),
        _ => println!("Day {} not implemented yet!", day),
    }
}
