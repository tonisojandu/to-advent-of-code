use std::cmp::Ordering;
use std::collections::VecDeque;
use lazy_static::lazy_static;
use serde_json::{json, Value};

lazy_static! {
    static ref INPUT_SAMPLE: &'static str = include_str!("day-13-sample.txt");
    static ref INPUT: &'static str = include_str!("day-13-main.txt");
}

pub fn day_13_first() {
    let output = execute_first(&INPUT);
    println!("Day 13-1: {}", output);
}

pub fn day_13_second() {
    let output = execute_second(&INPUT);
    println!("Day 13-2: {}", output);
}

fn execute_first(input: &str) -> i64 {
    let lines = input.lines().collect::<Vec<&str>>();

    let mut result: i64 = 0;

    let mut pair_index = 0;
    for i in (0..lines.len()).step_by(3) {
        pair_index += 1;
        if compare_lines(lines[i], lines[i + 1]) != Ordering::Greater {
            result += pair_index;
        }
    }

    return result;
}

fn compare_lines(left_line: &str, right_line: &str) -> Ordering {
    let left: Vec<Value> = serde_json::from_str(left_line).unwrap();
    let right: Vec<Value> = serde_json::from_str(right_line).unwrap();

    let mut left_stack = VecDeque::new();
    let mut right_stack = VecDeque::new();
    let mut observing_left = left;
    let mut observing_right = right;

    let mut index_stack = VecDeque::<usize>::new();
    let mut current_index = 0;
    loop {
        if current_index >= observing_left.len() || current_index >= observing_right.len() {
            if current_index < observing_right.len() {
                return Ordering::Less;
            }
            if current_index < observing_left.len() {
                return Ordering::Greater;
            }
            if index_stack.is_empty() {
                if current_index > observing_left.len() {
                    return Ordering::Less;
                }
                return Ordering::Equal;
            }

            current_index = index_stack.pop_front().unwrap();
            observing_left = left_stack.pop_front().unwrap();
            observing_right = right_stack.pop_front().unwrap();
        } else {
            let left_value = &observing_left[current_index];
            let right_value = &observing_right[current_index];
            if left_value.is_array() || right_value.is_array() {
                let new_left = if left_value.is_number() {
                    let num = json!(left_value.as_i64().unwrap());
                    vec!(num)
                } else {
                    left_value.as_array().unwrap().to_vec()
                };

                let new_right = if right_value.is_number() {
                    let num = json!(right_value.as_i64().unwrap());
                    vec!(num)
                } else {
                    right_value.as_array().cloned().unwrap().to_vec()
                };

                left_stack.push_front(observing_left);
                right_stack.push_front(observing_right);
                index_stack.push_front(current_index + 1);

                observing_left = new_left;
                observing_right = new_right;
                current_index = 0;
            } else {
                let left_num = left_value.as_i64().unwrap();
                let right_num = right_value.as_i64().unwrap();
                if left_num < right_num {
                    return Ordering::Less;
                }
                if left_num > right_num {
                    return Ordering::Greater;
                }
                current_index += 1;
            }
        }

    }
}

fn execute_second(input: &str) -> usize {
    let mut lines = input.lines().filter(|line| !line.trim().is_empty()).collect::<Vec<&str>>();
    lines.push("[[2]]");
    lines.push("[[6]]");

    lines.sort_by(|a, b| compare_lines(a, b));

    let mut result = 1;

    for (i, line) in lines.iter().enumerate() {
        match *line {
            "[[2]]" => result *= i + 1,
            "[[6]]" => result *= i + 1,
            _ => {}
        }
    }

    return result;
}

#[test]
fn test_first() {
    // given when
    let output = execute_first(&INPUT_SAMPLE);

    // then
    assert_eq!(output, 13);
}

#[test]
fn test_second() {
    // given when
    let output = execute_second(&INPUT_SAMPLE);

    // then
    assert_eq!(output, 140);
}
