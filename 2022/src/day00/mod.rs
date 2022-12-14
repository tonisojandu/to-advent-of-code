use lazy_static::lazy_static;

lazy_static! {
    static ref INPUT_SAMPLE: &'static str = include_str!("day-00-sample.txt");
    static ref INPUT: &'static str = include_str!("day-00-main.txt");
}

pub fn day_00_first() {
    let output = execute_first(&INPUT);
    println!("Day 00-1: {}", output);
}

pub fn day_00_second() {
    let output = execute_second(&INPUT);
    println!("Day 00-2: {}", output);
}

fn execute_first(input: &str) -> i32 {
    return input.len() as i32;
}

fn execute_second(input: &str) -> i32 {
    return input.len() as i32;
}

#[test]
fn test_first() {
    // given when
    let output = execute_first(&INPUT_SAMPLE);

    // then
    assert_ne!(output, -1);
}

#[test]
fn test_second() {
    // given when
    let output = execute_second(&INPUT_SAMPLE);

    // then
    assert_ne!(output, -1);
}
