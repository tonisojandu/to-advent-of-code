use std::collections::{HashMap, VecDeque};

fn options_for(
    matcher: &Vec<char>,
    arrangements_left: &mut VecDeque<i64>,
    previous_was: char,
    starting_from: usize,
    cache: &mut HashMap<(char, usize, usize), i64>,
) -> i64 {
    return match cache.get(&(previous_was, arrangements_left.len(), starting_from)) {
        Some(found) => {
            *found
        }
        None => {
            let result: i64 = if arrangements_left.len() > 0 && starting_from >= matcher.len() {
                0
            } else if arrangements_left.len() == 0 {
                if (starting_from..matcher.len())
                    .into_iter()
                    .any(|i| matcher[i] == '#') {
                    0
                } else {
                    1
                }
            } else {
                let mut total_found = 0;
                if matcher[starting_from] != '#' {
                    total_found += options_for(matcher, arrangements_left, '.', starting_from + 1, cache);
                }

                if matcher[starting_from] != '.' && previous_was == '.' {
                    let arrangement = arrangements_left.pop_front().unwrap();
                    let mut can_write = starting_from + arrangement as usize <= matcher.len();
                    if can_write {
                        for i in 0..arrangement as usize {
                            if matcher[starting_from + i] == '.' {
                                can_write = false;
                                break;
                            }
                        }
                    }
                    if can_write {
                        total_found += options_for(matcher, arrangements_left, '#', starting_from + arrangement as usize, cache);
                    }
                    arrangements_left.push_front(arrangement);
                }
                total_found
            };
            cache.insert((previous_was, arrangements_left.len(), starting_from), result);
            result
        }
    };
}

pub fn solve(lines: Vec<String>, part: i32) {
    let sum: i64 = lines.iter().map(|line| {
        let mut split = line.split(" ");
        let mut spring_string = split.next().unwrap().to_string();
        let mut arrangements_string = split.next().unwrap().to_string();
        if part == 2 {
            spring_string = format!("{}?{}?{}?{}?{}", spring_string, spring_string, spring_string, spring_string, spring_string);
            arrangements_string = format!("{},{},{},{},{}", arrangements_string, arrangements_string, arrangements_string, arrangements_string, arrangements_string);
        }

        let springs = spring_string.chars().collect::<Vec<char>>();
        let mut arrangements = arrangements_string.split(",").map(|s| s.parse::<i64>().unwrap()).collect::<VecDeque<i64>>();

        let found = options_for(&springs, &mut arrangements, '.', 0, &mut HashMap::new());

        found
    }).sum();

    println!("Day 12: {} = {:?}", part, sum);
}