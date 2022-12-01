use lazy_static::lazy_static;

lazy_static! {
    static ref INPUT_SAMPLE: &'static str = include_str!("day-01-sample.txt");
    static ref INPUT: &'static str = include_str!("day-01-main.txt");
}

pub fn day_01_first() {
    let output = execute_first(&INPUT);
    println!("Day 01-1: {}", output);
}

pub fn day_01_second() {
    let output = execute_second(&INPUT);
    println!("Day 01-2: {}", output);
}

fn execute_first(input: &str) -> i32 {
    return execute_and_sort(input)[0];
}

fn execute_second(input: &str) -> i32 {
    let elves = execute_and_sort(input);
    return elves[0] + elves[1] + elves[2];
}

fn execute_and_sort(input: &str) -> Vec<i32> {
    let mut elves = Vec::new();
    let mut current = 0;

    for line in input.lines() {
        if line.trim().len() == 0 {
            elves.push(current);
            current = 0;
        }
        else {
            current += line.trim().parse::<i32>().unwrap();
        }
    }

    if current > 0 {
        elves.push(current);
    }

    elves.sort_by(|a, b| b.cmp(a));

    return elves;
}

#[test]
fn test_first() {
    // given when
    let output = execute_first(&INPUT_SAMPLE);

    // then
    assert_eq!(output, 24000);
}

#[test]
fn test_second() {
    // given when
    let output = execute_second(&INPUT_SAMPLE);

    // then
    assert_eq!(output, 45000);
}
