use std::collections::{HashSet, VecDeque};
use lazy_static::lazy_static;

lazy_static! {
    static ref INPUT: &'static str = include_str!("day-06-main.txt");
}

pub fn day_06_first() {
    let output = execute_first(&INPUT);
    println!("Day 06-1: {}", output);
}

pub fn day_06_second() {
    let output = execute_second(&INPUT);
    println!("Day 06-2: {}", output);
}

fn execute_first(input: &str) -> i32 {
    return execute(input, 4);
}

fn execute_second(input: &str) -> i32 {
    return execute(input, 14);
}

fn execute(input: &str, marker_length: usize) -> i32 {
    let mut marker = Marker::new(marker_length);
    for (i, c) in input.chars().enumerate() {
        marker.add(c);
        if marker.ready() {
            return (i + 1) as i32;
        }
    }
    panic!("Did not find the marker");
}

struct Marker {
    expected_length: usize,
    order_queue: VecDeque<char>,
}

impl Marker {
    fn new(expected_length: usize) -> Marker {
        Marker {
            expected_length,
            order_queue: VecDeque::new(),
        }
    }

    fn ready(&self) -> bool {
        let mut unique_set = HashSet::<char>::new();
        self.order_queue.iter().for_each(|c| {
            unique_set.insert(*c);
        });
        return unique_set.len() == self.expected_length;
    }

    fn add(&mut self, c: char) {
        if self.order_queue.len() >= self.expected_length {
            self.order_queue.pop_front();
        }
        self.order_queue.push_back(c);
    }
}

#[cfg(test)]
const INPUT_SAMPLE_1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

#[cfg(test)]
const INPUT_SAMPLE_2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";

#[cfg(test)]
const INPUT_SAMPLE_3: &str = "nppdvjthqldpwncqszvftbrmjlhg";

#[cfg(test)]
const INPUT_SAMPLE_4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";

#[cfg(test)]
const INPUT_SAMPLE_5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

#[test]
fn test_first_1() {
    // given when
    let output = execute_first(INPUT_SAMPLE_1);

    // then
    assert_eq!(output, 7);
}

#[test]
fn test_first_2() {
    // given when
    let output = execute_first(INPUT_SAMPLE_2);

    // then
    assert_eq!(output, 5);
}

#[test]
fn test_first_3() {
    // given when
    let output = execute_first(INPUT_SAMPLE_3);

    // then
    assert_eq!(output, 6);
}

#[test]
fn test_first_4() {
    // given when
    let output = execute_first(INPUT_SAMPLE_4);

    // then
    assert_eq!(output, 10);
}

#[test]
fn test_first_5() {
    // given when
    let output = execute_first(&INPUT_SAMPLE_5);

    // then
    assert_eq!(output, 11);
}

#[test]
fn test_second_1() {
    // given when
    let output = execute_second(INPUT_SAMPLE_1);

    // then
    assert_eq!(output, 19);
}

#[test]
fn test_second_2() {
    // given when
    let output = execute_second(INPUT_SAMPLE_2);

    // then
    assert_eq!(output, 23);
}

#[test]
fn test_second_3() {
    // given when
    let output = execute_second(INPUT_SAMPLE_3);

    // then
    assert_eq!(output, 23);
}

#[test]
fn test_second_4() {
    // given when
    let output = execute_second(INPUT_SAMPLE_4);

    // then
    assert_eq!(output, 29);
}

#[test]
fn test_second_5() {
    // given when
    let output = execute_second(INPUT_SAMPLE_5);

    // then
    assert_eq!(output, 26);
}
