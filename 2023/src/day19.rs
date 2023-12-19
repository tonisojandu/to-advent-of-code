use std::collections::HashMap;
use num_bigint::BigInt;
use regex::Regex;

struct Condition {
    key: char,
    operator: char,
    value: i64,
    destination: String,
}

impl Condition {
    fn new(line: String) -> Condition {
        let mut split = line.split(":");
        let operation_string = split.next().unwrap();
        let mut operation_chars: Vec<char> = operation_string.chars().collect();
        let key = operation_chars[0];
        let operator = operation_chars[1];
        let value = operation_chars[2..].iter().collect::<String>().parse::<i64>().unwrap();
        let destination = split.next().unwrap().to_string();
        Condition {
            key,
            operator,
            value,
            destination
        }
    }
}

struct Rule {
    name: String,
    conditions: Vec<Condition>,
    fallback: String
}

impl Rule {
    fn new(line: String) -> Rule {
        let mut split = line.split("{");
        let name = split.next().unwrap().to_string();
        let binding = split.next().unwrap().replace("}", "");
        let condition_strings: Vec<&str> = binding.split(",").collect();
        let fallback = condition_strings[condition_strings.len() - 1].to_string();
        let mut conditions: Vec<Condition> = Vec::new();
        for i in 0..condition_strings.len() - 1 {
            conditions.push(Condition::new(condition_strings[i].to_string()));
        }
        Rule {
            name,
            conditions,
            fallback
        }
    }
}


pub fn solve(lines: Vec<String>, part: i32) {
    match part {
        1 => solve1(lines),
        2 => solve2(lines),
        _ => {}
    }
}
pub fn solve1(lines: Vec<String>) {
    let mut lines_split = lines.split(|line| line.len() == 0);

    let rules: HashMap<String, Rule> = lines_split.next().unwrap().iter().map(|line| {
        let rule = Rule::new(line.to_string());
        (rule.name.clone(), rule)
    }).collect();


    let result: i64 = lines_split.next().unwrap().iter().map(|line| {
        let binding = line.replace("}", "").replace("{", "");
        let re = Regex::new(r"[,=]").unwrap();
        let split: Vec<&str> = re.split(binding.as_str()).collect();
        let x: i64 = split[1].parse::<i64>().unwrap();
        let m: i64 = split[3].parse::<i64>().unwrap();
        let a: i64 = split[5].parse::<i64>().unwrap();
        let s: i64 = split[7].parse::<i64>().unwrap();

        let mut current_rule = rules.get("in").unwrap();
        let mut rule_result: Option<i64> = None;
        while rule_result.is_none() {
            // println!("{} {} {} {} {}", x, m, a, s, current_rule.name);
            let mut condition_result: Option<String> = None;
            'checking_conditions: for i in 0..current_rule.conditions.len() {
                let condition = &current_rule.conditions[i];
                let comparing = match condition.key {
                    'x' => x,
                    'm' => m,
                    'a' => a,
                    's' => s,
                    _ => panic!("Unknown key {}", condition.key)
                };
                let compare_result = match condition.operator {
                    '<' => comparing < condition.value,
                    '>' => comparing > condition.value,
                    // '=' => comparing == condition.value,
                    _ => panic!("Unknown operator {}", condition.operator)
                };
                if compare_result {
                    condition_result = Some(condition.destination.clone());
                    break 'checking_conditions;
                }
            }
            if condition_result.is_none() {
                condition_result = Some(current_rule.fallback.clone());
            }
            match condition_result.as_deref() {
                Some("R") => {
                    rule_result = Some(0);
                },
                Some("A") => {
                    rule_result = Some(x + m + a + s);
                },
                Some(new_rule) => {
                    current_rule = rules.get(new_rule).unwrap();
                },
                _ => panic!("Unknown condition result")
            }
        }

        rule_result.unwrap()
    }).sum();


    println!("Day 19: 1 = {:?}", result);

}


pub fn solve2(lines: Vec<String>) {
    let mut lines_split = lines.split(|line| line.len() == 0);

    let rules: HashMap<String, Rule> = lines_split.next().unwrap().iter().map(|line| {
        let rule = Rule::new(line.to_string());
        (rule.name.clone(), rule)
    }).collect();

    let mut ranges: HashMap<char, (i64, i64)> = HashMap::new();
    ranges.insert('x', (1, 4000));
    ranges.insert('m', (1, 4000));
    ranges.insert('a', (1, 4000));
    ranges.insert('s', (1, 4000));

    let result = find_range(&rules, &"in".to_string(), ranges);


    println!("Day 19: 2 = {:?}", result);
}

fn find_range(
    rules: &HashMap<String, Rule>,
    current_rule: &String,
    ranges: HashMap<char, (i64, i64)>,
) -> BigInt {
    let mut result = BigInt::from(0);
    let rule = rules.get(current_rule).unwrap();

    let mut changed_ranges = ranges.clone();
    for i in 0..rule.conditions.len() {
        let condition = &rule.conditions[i];
        let pivot = condition.value;
        let range = changed_ranges.get(&condition.key).unwrap();

        let (next_rule_range, next_condition_range ) = match condition.operator {
            '<' => {
                ((range.0, (pivot - 1).max(range.0)), (pivot.min(range.1), range.1))
            },
            '>' => {
                (((pivot + 1).min(range.1), range.1), (range.0, pivot.max(range.0)))
            },
            _ => panic!("Unknown operator {}", condition.operator)
        };
        changed_ranges.insert(condition.key, next_rule_range);
        match (&condition.destination.clone()).as_str() {
            "R" => {}
            "A" => {
                let sum: BigInt = changed_ranges.iter()
                    .map(|(_, (f, t))| BigInt::from((t - f + 1).max(0)))
                    .product();
                result += sum;
            }
            next_rule => {
                result += find_range(rules, &next_rule.to_string(), changed_ranges.clone());
            }
        }
        changed_ranges.insert(condition.key, next_condition_range);

    }
    match (&rule.fallback.clone()).as_str() {
        "R" => {}
        "A" => {
            let sum: BigInt = changed_ranges.iter()
                .map(|(_, (f, t))| BigInt::from((t - f + 1).max(0)))
                .product();
            result += sum;
        }
        nex_rule => {
            result += find_range(rules, &nex_rule.to_string(), changed_ranges.clone());
        }
    }

    return result;
}
