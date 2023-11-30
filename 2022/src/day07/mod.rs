use std::collections::{HashMap, VecDeque};
use lazy_static::lazy_static;

lazy_static! {
    static ref INPUT_SAMPLE: &'static str = include_str!("day-07-sample.txt");
    static ref INPUT: &'static str = include_str!("day-07-main.txt");
}

pub fn day_07_first() {
    let output = execute_first(&INPUT);
    println!("Day 07-1: {}", output);
}

pub fn day_07_second() {
    let output = execute_second(&INPUT);
    println!("Day 07-2: {}", output);
}

fn execute_first(input: &str) -> u64 {
    let root_dir = build_dir_structure(input);

    return calculate_first(&root_dir);
}

fn calculate_first(dir: &Dir) -> u64 {
    let mut size = 0;

    if dir.size < 100000 {
        size += dir.size;
    }

    for (_, sub_dir) in dir.sub_dirs.iter() {
        size += calculate_first(sub_dir);
    }

    return size;
}

fn execute_second(input: &str) -> u64 {
    let root_dir = build_dir_structure(input);

    let mut space_needed = root_dir.size - 40000000;

    let result = calculate_second(&root_dir, space_needed);

    space_needed = 0;

    return space_needed;
}

fn calculate_second(dir: &Dir, space_needed: u64) -> u64  {
    let mut result = u64::MAX;

    if dir.size >= space_needed && dir.size < result {
        result = dir.size;
    }

    for (_, sub_dir) in dir.sub_dirs.iter() {
        let sub_result = calculate_second(sub_dir, space_needed);
        if sub_result < result {
            result = sub_result;
        }
    }

    return result;
}

fn build_dir_structure(input: &str) -> Dir {
    unsafe {
        let mut root_dir = Dir::new();
        let mut dir_queue = VecDeque::<*mut Dir>::new();
        dir_queue.push_back(&mut root_dir);

        for line in input.lines() {
            if line == "$ cd /" {
                while dir_queue.len() > 1 {
                    dir_queue.pop_back();
                }
            } else if line == "$ cd .." {
                if dir_queue.len() > 1 {
                    dir_queue.pop_back();
                }
            } else if line.starts_with("$ cd ") {
                let next_dir = line.split_whitespace().collect::<Vec<&str>>()[2];
                let current_dir = dir_queue.pop_back().unwrap();

                match (*current_dir).sub_dirs.get_mut(next_dir) {
                    Some(dir) => {
                        dir_queue.push_back(current_dir);
                        dir_queue.push_back(dir);
                    },
                    None => {
                        let dir = Dir::new();
                        (*current_dir).sub_dirs.insert(next_dir.to_string(), dir);
                        dir_queue.push_back(current_dir);
                        dir_queue.push_back((*current_dir).sub_dirs.get_mut(next_dir).unwrap());
                    }
                }
            } else if line.starts_with("dir ") || line == "$ ls" {
                // ignore
            } else {
                let components = line.split_whitespace().collect::<Vec<&str>>();
                let file_size: u64 = components[0].parse().unwrap();
                let file_name = components[1];
                let current_dir = dir_queue.pop_back().unwrap();
                (*current_dir).files.insert(file_name.to_string(), file_size);
                dir_queue.push_back(current_dir);
            }
        }

        calculate_size(&mut root_dir);

        return root_dir;
    }
}

fn calculate_size(dir: &mut Dir) -> u64 {
    let mut size = 0;

    for (_, file_size) in dir.files.iter() {
        size += file_size;
    }

    for (_, sub_dir) in dir.sub_dirs.iter_mut() {
        size += calculate_size(sub_dir);
    }

    dir.size = size;

    return size;
}

struct Dir {
    size: u64,
    sub_dirs: HashMap<String, Dir>,
    files: HashMap<String, u64>,
}

impl Dir {
    fn new() -> Dir {
        Dir {
            size: 0,
            sub_dirs: HashMap::new(),
            files: HashMap::new(),
        }
    }
}

#[test]
fn test_first() {
    // given when
    let output = execute_first(&INPUT_SAMPLE);

    // then
    assert_eq!(output, 95437);
}

#[test]
fn test_second() {
    // given when
    let output = execute_second(&INPUT_SAMPLE);

    // then
    assert_eq!(output, 24933642);
}
