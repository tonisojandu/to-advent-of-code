use std::collections::HashMap;

pub fn solve(lines: Vec<String>, part: i32) {
    match part {
        1 => part1(lines),
        2 => part2(lines),
        _ => panic!("Unknown part {}", part),
    }
}

fn part1(lines: Vec<String>) {
    let instructions: Vec<char> = lines[0].chars().collect();

    let mut left: HashMap<String, String> = HashMap::new();
    let mut right: HashMap<String, String> = HashMap::new();

    lines.iter().skip(2).for_each(|line| {
        let to_parse = line.replace("=", "")
            .replace("(", "")
            .replace(")", "")
            .replace(",", "")
            .replace("  ", " ");
        let path_parts = to_parse.split(" ").collect::<Vec<&str>>();
        left.insert(path_parts[0].to_string(), path_parts[1].to_string());
        right.insert(path_parts[0].to_string(), path_parts[2].to_string());
    });

    let start = "AAA";
    let end = "ZZZ";

    let mut current_path: Vec<String> = vec![start.to_string()];

    loop {
        let current_node = current_path.last().unwrap();
        if current_node == end {
            break;
        }

        let current_instruction = instructions[(current_path.len() - 1) % instructions.len()];
        match current_instruction {
            'L' => {
                let next_node = left.get(current_node).unwrap();
                current_path.push(next_node.to_string());
            }
            'R' => {
                println!("R: {}", current_node);
                let next_node = right.get(current_node).unwrap();
                current_path.push(next_node.to_string());
            }
            _ => panic!("Unknown instruction {}", current_instruction),
        }
    }

    println!("Day 08: 1 = {:?}", current_path.len() - 1);
}

fn part2(lines: Vec<String>) {
    let instructions: Vec<char> = lines[0].chars().collect();

    let mut left: HashMap<String, String> = HashMap::new();
    let mut right: HashMap<String, String> = HashMap::new();

    let mut starts = vec![];

    lines.iter().skip(2).for_each(|line| {
        let to_parse = line.replace("=", "")
            .replace("(", "")
            .replace(")", "")
            .replace(",", "")
            .replace("  ", " ");
        println!("To parse: {}", to_parse);
        let path_parts = to_parse.split(" ").collect::<Vec<&str>>();
        left.insert(path_parts[0].to_string(), path_parts[1].to_string());
        right.insert(path_parts[0].to_string(), path_parts[2].to_string());

        if path_parts[0].ends_with('A') {
            starts.push(path_parts[0].to_string());
        }
    });

    let width = starts.len();
    let mut depth = 0;
    let mut current_positions = starts.clone();

    let mut saw_end_at: Vec<i64> = Vec::new();
    for _ in 0..width {
        saw_end_at.push(0);
    }

    loop {
        let mut ends_seen = 0;
        for i in 0..width {
            if saw_end_at[i] > 0 {
                ends_seen += 1;
            } else if current_positions[i].ends_with('Z') {
                saw_end_at[i] = depth;
                ends_seen += 1;
            }
        }
        if ends_seen == width {
            break;
        }

        let current_instruction = instructions[(depth % instructions.len() as i64) as usize];

        for i in 0..width {
            match current_instruction {
                'L' => {
                    let next_node = left.get(current_positions[i].as_str()).unwrap();
                    current_positions[i] = next_node.to_string();
                }
                'R' => {
                    let next_node = right.get(current_positions[i].as_str()).unwrap();
                    current_positions[i] = next_node.to_string();
                }
                _ => panic!("Unknown instruction {}", current_instruction),
            }
        }
        depth += 1;
    }

    let mut lcm = saw_end_at[0];
    for i in 1..width {
        lcm = lcm * saw_end_at[i] / gcd(lcm, saw_end_at[i]);
    }

    println!("Day 08: 2 = {:?}", lcm);
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    return gcd(b, a % b);
}