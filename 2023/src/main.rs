use std::fs::File;
use std::io;
use std::io::BufRead;
use std::string::ToString;
use clap::ArgAction;
use clap::Parser;

mod common;
mod day00;
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
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
// mod day17;
mod day18;
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

    let file_name = build_input_file_name(&args.inputs, day, args.example, part);
    let fallback_name = build_input_file_name(&args.inputs, day, args.example, 1);

    let file = match File::open(file_name.clone()).or(File::open(&fallback_name)) {
        Ok(file) => {
            file
        },
        Err(error) => {
            println!("Error opening both {} and {}: {}", file_name, fallback_name, error);
            return;
        }
    };

    let buf_reader = io::BufReader::new(file);
    let mut lines: Vec<String> = buf_reader.lines().map(|s| s.unwrap()).collect();

    // remove last empty lines
    while lines.len() > 0 && lines[lines.len() - 1].len() == 0 {
        lines.pop();
    }

    match day {
        0 => day00::solve(lines, part),
        1 => day01::solve(lines, part),
        2 => day02::solve(lines, part),
        3 => day03::solve(lines, part),
        4 => day04::solve(lines, part),
        5 => day05::solve(lines, part),
        6 => day06::solve(lines, part),
        7 => day07::solve(lines, part),
        8 => day08::solve(lines, part),
        9 => day09::solve(lines, part),
        10 => day10::solve(lines, part),
        11 => day11::solve(lines, part),
        12 => day12::solve(lines, part),
        13 => day13::solve(lines, part),
        14 => day14::solve(lines, part),
        15 => day15::solve(lines, part),
        16 => day16::solve(lines, part),
        // 17 => day17::solve(lines, part),
        18 => day18::solve(lines, part),
        // 19 => day19::solve(lines, part),
        // 20 => day20::solve(lines, part),
        // 21 => day21::solve(lines, part),
        // 22 => day22::solve(lines, part),
        // 23 => day23::solve(lines, part),
        // 24 => day24::solve(lines, part),
        // 25 => day25::solve(lines, part),
        _ => println!("Day {} not implemented", day),
    };
}

fn build_input_file_name(directory: &str, day: i32, is_example: bool, _: i32) -> String {
    return format!(
        "{}/{:02}{}",
        directory,
        day,
        if is_example { ".example" } else { ".in" }
    );
}