use lazy_static::lazy_static;

lazy_static! {
    static ref INPUT_SAMPLE: &'static str = include_str!("day-04-sample.txt");
    static ref INPUT: &'static str = include_str!("day-04-main.txt");
}

pub fn day_04_first() {
    let output = execute_first(&INPUT);
    println!("Day 04-1: {}", output);
}

pub fn day_04_second() {
    let output = execute_second(&INPUT);
    println!("Day 04-2: {}", output);
}

fn execute_first(input: &str) -> u32 {
    return execute_on_pairs(input, |pair| {
        return (pair.first.from >= pair.second.from && pair.first.to <= pair.second.to) ||
            (pair.second.from >= pair.first.from && pair.second.to <= pair.first.to)
    });
}

fn execute_second(input: &str) -> u32 {
    return execute_on_pairs(input, |pair| {
        return (pair.first.from >= pair.second.from && pair.first.from <= pair.second.to) ||
            (pair.first.to >= pair.second.from && pair.first.to <= pair.second.to) ||
            (pair.second.from >= pair.first.from && pair.second.from <= pair.first.to) ||
            (pair.second.to >= pair.first.from && pair.second.to <= pair.first.to);
    });
}

fn execute_on_pairs(input: &str, overlaps: fn(Pair) -> bool) -> u32 {
    return input.lines()
        .map(|line| Pair::new(line))
        .map(|pair| overlaps(pair) as u32)
        .sum();
}

struct Assignment {
    from: u32,
    to: u32,
}

impl Assignment {
    fn new(from: &str) -> Assignment {
        let raw = from.split("-").collect::<Vec<&str>>();
        return Self {
            from: raw[0].parse::<u32>().unwrap(),
            to:  raw[1].parse::<u32>().unwrap(),
        }
    }
}

struct Pair {
    first: Assignment,
    second: Assignment,
}

impl Pair {
    fn new(from: &str) -> Pair {
        let raw = from.split(",").collect::<Vec<&str>>();
        return Self {
            first: Assignment::new(raw[0]),
            second: Assignment::new(raw[1]),
        }
    }
}

#[test]
fn test_first() {
    // given when
    let output = execute_first(&INPUT_SAMPLE);

    // then
    assert_eq!(output, 2);
}

#[test]
fn test_second() {
    // given when
    let output = execute_second(&INPUT_SAMPLE);

    // then
    assert_eq!(output, 4);
}
