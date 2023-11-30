extern crate core;

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
// mod day16;
mod day17;
mod day18;
// mod day19;

use day00::day_00_first;
use day00::day_00_second;
use day01::day_01_first;
use day01::day_01_second;
use day02::day_02_first;
use day02::day_02_second;
use day03::day_03_first;
use day03::day_03_second;
use day04::day_04_first;
use day04::day_04_second;
use day05::day_05_first;
use day05::day_05_second;
use day06::day_06_first;
use day06::day_06_second;
use day07::day_07_first;
use day07::day_07_second;
use day08::day_08_first;
use day08::day_08_second;
use day09::day_09_first;
use day09::day_09_second;
use day10::day_10_first;
use day10::day_10_second;
use day11::day_11_first;
use day11::day_11_second;
use day12::day_12_first;
use day12::day_12_second;
use day13::day_13_first;
use day13::day_13_second;
use day14::day_14_first;
use day14::day_14_second;
use day15::day_15_first;
use day15::day_15_second;
// use day16::day_16_first;
// use day16::day_16_second;
use day17::day_17_first;
use day17::day_17_second;
use day18::day_18_first;
use day18::day_18_second;
// use day19::day_19_first;
// use day19::day_19_second;
use std::env;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!(
            "Running:\n\
            \ttarget/release/aoc-2020 <day>-<part>\n\
            \n\
            Example:\n\
            \ttarget/release/aoc-2020 01-1\n"
        );
        return;
    }

    match args[1].as_str() {
        "00-1" => day_00_first(),
        "00-2" => day_00_second(),
        "01-1" => day_01_first(),
        "01-2" => day_01_second(),
        "02-1" => day_02_first(),
        "02-2" => day_02_second(),
        "03-1" => day_03_first(),
        "03-2" => day_03_second(),
        "04-1" => day_04_first(),
        "04-2" => day_04_second(),
        "05-1" => day_05_first(),
        "05-2" => day_05_second(),
        "06-1" => day_06_first(),
        "06-2" => day_06_second(),
        "07-1" => day_07_first(),
        "07-2" => day_07_second(),
        "08-1" => day_08_first(),
        "08-2" => day_08_second(),
        "09-1" => day_09_first(),
        "09-2" => day_09_second(),
        "10-1" => day_10_first(),
        "10-2" => day_10_second(),
        "11-1" => day_11_first(),
        "11-2" => day_11_second(),
        "12-1" => day_12_first(),
        "12-2" => day_12_second(),
        "13-1" => day_13_first(),
        "13-2" => day_13_second(),
        "14-1" => day_14_first(),
        "14-2" => day_14_second(),
        "15-1" => day_15_first(),
        "15-2" => day_15_second(),
        // "16-1" => day_16_first(),
        // "16-2" => day_16_second(),
        "17-1" => day_17_first(),
        "17-2" => day_17_second(),
        "18-1" => day_18_first(),
        "18-2" => day_18_second(),
        // "19-1" => day_19_first(),
        // "19-2" => day_19_second(),
        _ => println!("No process for command: {}", args[0]),
    }
}
