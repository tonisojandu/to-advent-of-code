mod monkey;

use lazy_static::lazy_static;
use num_bigint::BigUint;
use monkey::Monkey;

lazy_static! {
    static ref INPUT_SAMPLE: &'static str = include_str!("day-11-sample.txt");
    static ref INPUT: &'static str = include_str!("day-11-main.txt");
}

pub fn day_11_first() {
    let output = execute_first(&INPUT);
    println!("Day 11-1: {}", output);
}

pub fn day_11_second() {
    let output = execute_second(&INPUT);
    println!("Day 11-2: {}", output);
}

fn execute_first(input: &str) -> usize {
    return execute( parse_monkeys(input), 20,  &(|item| item /  (3 as u32)));
}

fn execute_second(input: &str) -> usize {
    let monkeys = parse_monkeys(input);
    let mut m = 1;
    for i in 0..monkeys.len() {
        m *= monkeys[i].test_divider;
    }
    return execute(monkeys , 10000, &(|item| item % m));
}

fn execute(mut monkeys: Vec<Monkey>, iterations: usize, calm_down: &impl Fn(&BigUint) -> BigUint) -> usize {
    let mut inspect_counts = vec![0; monkeys.len()];

    for _ in 0..iterations {
        for i in 0..monkeys.len() {
            let throws = monkeys[i].inspect_items(calm_down);
            inspect_counts[i] += throws.len();

            for throw in throws {
                monkeys[throw.0].catch_item(throw.1);
            }
        }
    }

    inspect_counts.sort_by(|a, b| b.cmp(a));

    return inspect_counts[0] * inspect_counts[1];
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    let lines: Vec<&str> = input.lines().collect();

    let mut monkeys = vec!();
    for i in (0..lines.len()).step_by(7) {
        monkeys.push(Monkey::parse(&lines[i..]));
    }

    return monkeys;
}


#[test]
fn test_first() {
    // given when
    let output = execute_first(&INPUT_SAMPLE);

    // then
    assert_eq!(output, 10605);
}

#[test]
fn test_second() {
    // given when
    let output = execute_second(&INPUT_SAMPLE);

    // then
    assert_eq!(output, 2713310158);
}
