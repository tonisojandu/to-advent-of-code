use lazy_static::lazy_static;
use std::collections::HashSet;

lazy_static! {
    static ref INPUT_SAMPLE: &'static str = include_str!("day-03-sample.txt");
    static ref INPUT: &'static str = include_str!("day-03-main.txt");
}

pub fn day_03_first() {
    let output = execute_first(&INPUT);
    println!("Day 03-1: {}", output);
}

pub fn day_03_second() {
    let output = execute_second(&INPUT);
    println!("Day 03-2: {}", output);
}

fn execute_first(input: &str) -> i32 {
    return input.lines().map(|line| {
        let compartment_size = line.len() / 2;

        let (first, second) = line.split_at(compartment_size);
        let first_chars = HashSet::<char>::from_iter(first.chars());
        let second_chars = HashSet::<char>::from_iter(second.chars());
        return second_chars.iter()
            .filter(|c| first_chars.contains(c))
            .map(|c| return priority(c))
            .sum::<u32>()
    }).sum::<u32>() as i32
}

fn execute_second(input: &str) -> i32 {
    let mut sum: u32 = 0;
    let lines = input.lines().collect::<Vec<&str>>();

    for i in (0..lines.len()).step_by(3) {
        let first = lines[i];
        let second = lines[i + 1];
        let third = lines[i + 2];

        let first_chars = HashSet::<char>::from_iter(first.chars());
        let second_chars = HashSet::<char>::from_iter(second.chars());
        let third_chars = HashSet::<char>::from_iter(third.chars());

        let first_intersection = first_chars.intersection(&second_chars)
            .map(|c| c.clone())
            .collect::<HashSet<char>>();
        let second_intersection = first_intersection.intersection(&third_chars)
            .collect::<HashSet<&char>>();

        sum += second_intersection.iter()
            .map(|c| return priority(c))
            .sum::<u32>()
    }
    return sum as i32;
}

fn priority(c: &char) -> u32 {
    let code = (*c) as u32;
    if code >= 65 && code <= 90 {
        return code - 38;
    }
    if code >= 97 && code <= 122 {
        return code - 96;
    }
    panic!("no priority for char: {}", c)
}

#[test]
fn test_first() {
    // given when
    let output = execute_first(&INPUT_SAMPLE);

    // then
    assert_eq!(output, 157);
}

#[test]
fn test_second() {
    // given when
    let output = execute_second(&INPUT_SAMPLE);

    // then
    assert_eq!(output, 70);
}
