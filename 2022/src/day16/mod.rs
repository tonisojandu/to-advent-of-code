use std::collections::{HashMap, VecDeque};

use derive_new::new;
use grid::Grid;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref INPUT_SAMPLE: &'static str = include_str!("day-16-sample.txt");
    static ref INPUT: &'static str = include_str!("day-16-main.txt");
}

pub fn day_16_first() {
    let output = execute_first(&INPUT);
    println!("Day 16-1: {}", output);
}

pub fn day_16_second() {
    let output = execute_second(&INPUT);
    println!("Day 16-2: {}", output);
}

fn execute_second(input: &str) -> i64 {
    return execute(input, true);
}

fn execute_first(input: &str) -> i64 {
    return execute(input, false);
}

fn execute(input: &str, include_elephant: bool) -> i64 {
    let mut key_indexes = KeyIndexes::new();
    let mut valves = HashMap::new();
    for line in input.lines() {
        let (name, valve) = Valve::from(line);
        key_indexes.get(&name);
        valves.insert(name, valve);
    }

    let mut useful_valves = Vec::new();
    let mut shortest_path = Grid::init(valves.len(), valves.len(), i64::MAX);

    for (name, valve) in &valves {
        let key_index = key_indexes.get(*name);
        shortest_path[key_index][key_index] = 0;
        if valve.rate > 0 {
            useful_valves.push((key_index, valve.rate.clone()));
        }

        for other_name in &valve.leading_to {
            let other_key_index = key_indexes.get(&other_name);
            shortest_path[key_index][other_key_index] = 1;
        }
    }

    for valve1_name in valves.keys() {
        let valve1 = key_indexes.get(valve1_name);
        for valve2_name in valves.keys() {
            let valve2 = key_indexes.get(valve2_name);
            for valve3_name in valves.keys() {
                let valve3 = key_indexes.get(valve3_name);
                if shortest_path[valve2][valve1] < i64::MAX &&
                    shortest_path[valve1][valve3] < i64::MAX &&
                    shortest_path[valve2][valve3] > shortest_path[valve2][valve1] + shortest_path[valve1][valve3] {
                    shortest_path[valve2][valve3] = shortest_path[valve2][valve1] + shortest_path[valve1][valve3];
                }
            }
        }
    }

    let current_valve = key_indexes.get("AA");

    let mut max = 0;

    let start_time = if include_elephant { 26 } else { 30 };

    let mut solution_queue = VecDeque::new();
    solution_queue.push_back(Solution::new(
        key_indexes.get("AA"),
        key_indexes.get("AA"),
        start_time,
        start_time,
        0,
        // Vec::new(),
        // Vec::new(),
        useful_valves,
        0,
    ));


    loop {
        let add_solution = match solution_queue.get_mut(0) {
            Some(solution) => {
                if solution.options_iterator * 2 < solution.options.len() {
                    let option = solution.options[solution.options_iterator / 2];

                    if solution.options_iterator % 2 == 0 {
                        let new_time = solution.time - shortest_path[solution.current_valve][option.0] - 1;
                        let pressure = solution.pressure + option.1 * new_time;
                        if new_time >= 0 {
                            if pressure > max {
                                max = pressure;
                                // println!();
                                // println!("Chose me: {}+{}*{}={}+{}={:?}", solution.pressure, option.1, new_time, solution.pressure, option.1 * new_time, pressure);
                                // for choice in &solution.choices {
                                //     println!("  {:?}", key_indexes.reverse(*choice));
                                // }
                                // println!("  {:?}", key_indexes.reverse(option.0));
                                // println!("--");
                                // for choice in &solution.elephant_choices {
                                //     println!("  {:?}", key_indexes.reverse(*choice));
                                // }
                            }
                            // let mut choices = solution.choices.clone();
                            // choices.push(option.0);
                            let mut options = Vec::new();
                            solution.options.iter().filter(|x| *x != &option).for_each(|x| options.push(x.clone()));
                            solution_queue.push_back(Solution::new(
                                option.0,
                                solution.elephant_valve.clone(),
                                new_time,
                                solution.elephant_time.clone(),
                                pressure,
                                // choices,
                                // solution.elephant_choices.clone(),
                                options,
                                0,
                            ));
                        }
                    } else if include_elephant {
                        let new_elephant_time = solution.elephant_time - shortest_path[solution.elephant_valve][option.0] - 1;
                        let elephant_pressure = solution.pressure + option.1 * new_elephant_time;
                        if new_elephant_time >= 0 {
                            if elephant_pressure > max {
                                max = elephant_pressure;
                                // println!();
                                // println!("Chose elephant: {}+{}*{}={}+{}={:?}", solution.pressure, option.1, new_elephant_time, solution.pressure, option.1 * new_elephant_time, elephant_pressure);
                                // for choice in &solution.elephant_choices {
                                //     println!("  {:?}", key_indexes.reverse(*choice));
                                // }
                                // println!("  {:?}", key_indexes.reverse(option.0));
                                // println!("--");
                                // for choice in &solution.choices {
                                //     println!("  {:?}", key_indexes.reverse(*choice));
                                // }
                            }
                            // let mut elephant_choices = solution.elephant_choices.clone();
                            // elephant_choices.push(option.0);
                            let mut elephant_options = Vec::new();
                            solution.options.iter().filter(|x| *x != &option).for_each(|x| elephant_options.push(x.clone()));
                            solution_queue.push_back(Solution::new(
                                solution.current_valve.clone(),
                                option.0,
                                solution.time.clone(),
                                new_elephant_time,
                                elephant_pressure,
                                // solution.choices.clone(),
                                // elephant_choices,
                                elephant_options,
                                0,
                            ));
                        }
                    }

                    solution.options_iterator += 1;
                    Some(solution)
                } else {
                    None
                }
            }
            None => None
        };
        match add_solution {
            Some(s) => solution_queue.push_front(s),
            None => {
                match solution_queue.get(0) {
                    Some(solution) => {
                        if solution.options_iteratro >= solution.options.len() {
                            solution_queue.pop_front();
                        }
                    }
                    None => break
                }
            }
        }
    }

    return max;
}

#[derive(new)]
struct Solution {
    current_valve: usize,
    elephant_valve: usize,
    time: i64,
    elephant_time: i64,
    pressure: i64,
    // choices: Vec<usize>,
    // elephant_choices: Vec<usize>,
    options: Vec<(usize, i64)>,
    options_iterator: usize,
}

struct KeyIndexes {
    index_generator: usize,
    map: HashMap<String, usize>,
    reverse_map: HashMap<usize, String>,
}

impl KeyIndexes {
    fn new() -> KeyIndexes {
        Self {
            index_generator: 0,
            map: HashMap::new(),
            reverse_map: HashMap::new(),
        }
    }

    fn get(&mut self, name: &str) -> usize {
        match self.map.get(name) {
            Some(c) => c.clone(),
            _ => {
                self.map.insert(name.to_string(), self.index_generator.clone());
                self.reverse_map.insert(self.index_generator.clone(), name.to_string());
                self.index_generator += 1;
                self.map.get(name).unwrap().clone()
            }
        }
    }

    fn reverse(&self, index: usize) -> &str {
        self.reverse_map.get(&index).unwrap()
    }
}

struct Valve {
    rate: i64,
    leading_to: Vec<String>,
}

impl Valve {
    fn from(line: &str) -> (&str, Valve) {
        let re = Regex::new(r"Valve ([^ ]+) has flow rate=(\d+); tunnels? leads? to valves? (.+)").unwrap();
        let caps = re.captures(line).unwrap();
        let name = caps.get(1).unwrap().as_str();
        let rate = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
        let leading_to = caps.get(3).unwrap().as_str().split(", ").map(|s| s.to_string()).collect::<Vec<String>>();
        return (name, Valve {
            rate,
            leading_to,
        });
    }
}

#[test]
fn test_first() {
    // given when
    let output = execute_first(&INPUT_SAMPLE);

    // then
    assert_eq!(output, 1651);
    // assert_ne!(output, 0);
}

#[test]
fn test_second() {
    // given when
    let output = execute_second(&INPUT_SAMPLE);

    // then
    assert_eq!(output, 1707);
}
