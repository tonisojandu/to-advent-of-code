use std::fs::File;
use std::io;
use std::io::BufRead;
use std::string::ToString;
use clap::ArgAction;
use clap::Parser;
use crate::common::Day;
use crate::day00::Day00;
use crate::day01::Day01;
// use crate::day02::Day02;
// use crate::day03::Day03;
// use crate::day04::Day04;
// use crate::day05::Day05;
// use crate::day06::Day06;
// use crate::day07::Day07;
// use crate::day08::Day08;
// use crate::day09::Day09;
// use crate::day10::Day10;
// use crate::day11::Day11;
// use crate::day12::Day12;
// use crate::day13::Day13;
// use crate::day14::Day14;
// use crate::day15::Day15;
// use crate::day16::Day16;
// use crate::day17::Day17;
// use crate::day18::Day18;
// use crate::day19::Day19;
// use crate::day20::Day20;
// use crate::day21::Day21;
// use crate::day22::Day22;
// use crate::day23::Day23;
// use crate::day24::Day24;
// use crate::day25::Day25;

mod common;
mod day00;
mod day01;
// mod day02;
// mod day03;
// mod day04;
// mod day05;
// mod day06;
// mod day07;
// mod day08;
// mod day09;
// mod day10;
// mod day11;
// mod day12;
// mod day13;
// mod day14;
// mod day15;
// mod day16;
// mod day17;
// mod day18;
// mod day19;
// mod day20;
// mod day21;
// mod day22;
// mod day23;
// mod day24;
// mod day25;

static DEFAULT_INPUTS: &'static str = "../../to-advent-of-code-inputs/2023";

/// 2023 Advent of Code solutions by Tõnis Ojandu
#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
struct Args {
    /// Day and part to run in format <day>-<part>
    #[arg(short, long)]
    command: String,

    /// Inputs directory
    #[arg(short, long, default_value_t = DEFAULT_INPUTS.to_string())]
    inputs: String,

    /// Use example inputs
    #[clap(long, short, action = ArgAction::SetTrue)]
    example: bool,
}


fn main() {
    let args = Args::parse();

    let parts: Vec<&str> = args.command.split("-").collect();
    let day = parts[0].parse::<i32>().unwrap();
    let part = parts[1].parse::<i32>().unwrap();

    let file_name = format!(
        "{}/day{:02}{}.in",
        args.inputs, day,
        if args.example { ".example" } else { "" }
    );

    println!("command: {} input: {}", args.command, file_name);

    let file = match File::open(file_name.clone()) {
        Ok(file) => file,
        Err(error) => {
            println!("Error opening file {}: {}", file_name, error);
            return;
        }
    };

    let buf_reader = io::BufReader::new(file);
    let lines = buf_reader.lines().map(|s| s.unwrap()).collect();

    match day {
        0 => Day00::spawn(lines).solve(part),
        1 => Day01::spawn(lines).solve(part),
        // 2 => Some(Day02::spawn(lines)),
        // 3 => Some(Day03::spawn(lines)),
        // 4 => Some(Day04::spawn(lines)),
        // 5 => Some(Day05::spawn(lines)),
        // 6 => Some(Day06::spawn(lines)),
        // 7 => Some(Day07::spawn(lines)),
        // 8 => Some(Day08::spawn(lines)),
        // 9 => Some(Day09::spawn(lines)),
        // 10 => Some(Day10::spawn(lines)),
        // 11 => Some(Day11::spawn(lines)),
        // 12 => Some(Day12::spawn(lines)),
        // 13 => Some(Day13::spawn(lines)),
        // 14 => Some(Day14::spawn(lines)),
        // 15 => Some(Day15::spawn(lines)),
        // 16 => Some(Day16::spawn(lines)),
        // 17 => Some(Day17::spawn(lines)),
        // 18 => Some(Day18::spawn(lines)),
        // 19 => Some(Day19::spawn(lines)),
        // 20 => Some(Day20::spawn(lines)),
        // 21 => Some(Day21::spawn(lines)),
        // 22 => Some(Day22::spawn(lines)),
        // 23 => Some(Day23::spawn(lines)),
        // 24 => Some(Day24::spawn(lines)),
        // 25 => Some(Day25::spawn(lines)),
        _ => println!("Day {} not implemented", day),
    };

    // match day_option {
    //     Some(day) => day.solve(part),
    //     None => println!("Day {} not implemented", day),
    // }
}
