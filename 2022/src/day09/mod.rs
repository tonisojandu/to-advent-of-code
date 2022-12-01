use std::collections::HashSet;
use lazy_static::lazy_static;

lazy_static! {
    static ref INPUT_SAMPLE_1: &'static str = include_str!("day-09-sample-1.txt");
    static ref INPUT_SAMPLE_2: &'static str = include_str!("day-09-sample-2.txt");
    static ref INPUT: &'static str = include_str!("day-09-main.txt");
}

pub fn day_09_first() {
    let output = execute_first(&INPUT);
    println!("Day 09-1: {}", output);
}

pub fn day_09_second() {
    let output = execute_second(&INPUT);
    println!("Day 09-2: {}", output);
}

fn execute_first(input: &str) -> i32 {
    return execute(input, 2);
}

fn execute_second(input: &str) -> i32 {
    return execute(input, 10);
}

fn execute(input: &str, rope_length: usize) -> i32 {
    let mut rope_coords = Vec::<(i32,i32)>::new();
    for _ in 0..rope_length {
        rope_coords.push((0, 0));
    }

    let mut all_tail_coords = HashSet::<(i32,i32)>::new();
    all_tail_coords.insert((0, 0));

    for line in input.lines() {
        let head_moves = parse_moves(line);

        for head_move in head_moves {
            rope_coords[0] = (rope_coords[0].0 + head_move.0, rope_coords[0].1 + head_move.1);

            for i in 1..rope_length {
                let delta_vector = (rope_coords[i-1].0 - rope_coords[i].0, rope_coords[i-1].1 - rope_coords[i].1);
                let tail_move = decide_tail_movement(delta_vector);
                rope_coords[i] = (rope_coords[i].0 + tail_move.0, rope_coords[i].1 + tail_move.1);
            }

            all_tail_coords.insert(rope_coords[rope_length-1].clone());
        }
    }

    return all_tail_coords.len() as i32;
}

fn parse_moves(input: &str) -> Vec<(i32, i32)> {
    let split = input.split_whitespace().collect::<Vec<&str>>();

    let unit_vector = match split[0] {
        "R" => (1, 0),
        "L" => (-1, 0),
        "U" => (0, 1),
        "D" => (0, -1),
        c => panic!("Invalid direction: {}", c),
    };

    let multiplier = split[1].parse::<i32>().unwrap();

    let mut result = Vec::<(i32, i32)>::new();
    for _ in 0..multiplier {
        result.push(unit_vector.clone());
    }

    return result;
}

fn decide_tail_movement(delta_vector: (i32, i32)) -> (i32, i32) {
    let abs_delta = (delta_vector.0.abs(), delta_vector.1.abs());
    if abs_delta.0 < 2 && abs_delta.1 < 2 {
        return (0, 0);
    }
    if abs_delta.0 > 0 && abs_delta.1 > 0 && (abs_delta.0 > 1 || abs_delta.1 > 1) {
        return (delta_vector.0 / abs_delta.0, delta_vector.1 / abs_delta.1);
    }
    if abs_delta.0 > 1 {
        return (delta_vector.0 / abs_delta.0, 0);
    }
    if abs_delta.1 > 1 {
        return (0, delta_vector.1 / abs_delta.1);
    }

    panic!("No rule for delta: {:?}", delta_vector);
}

#[test]
fn test_first() {
    // given when
    let output = execute_first(&INPUT_SAMPLE_1);

    // then
    assert_eq!(output, 13);
}

#[test]
fn test_second_1() {
    // given when
    let output = execute_second(&INPUT_SAMPLE_1);

    // then
    assert_eq!(output, 1);
}

#[test]
fn test_second_2() {
    // given when
    let output = execute_second(&INPUT_SAMPLE_2);

    // then
    assert_eq!(output, 36);
}
