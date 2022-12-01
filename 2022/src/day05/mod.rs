use lazy_static::lazy_static;

lazy_static! {
    static ref INPUT_SAMPLE: &'static str = include_str!("day-05-sample.txt");
    static ref INPUT: &'static str = include_str!("day-05-main.txt");
}

pub fn day_05_first() {
    let output = execute_first(&INPUT);
    println!("Day 05-1: {}", output);
}

pub fn day_05_second() {
    let output = execute_second(&INPUT);
    println!("Day 05-2: {}", output);
}

fn execute_first(input: &str) -> String {
    return execute(input, |stacks, line| crane_mover_9000(stacks, line));
}

fn execute_second(input: &str) -> String {
    return execute(input, |stacks, line| crane_mover_9001(stacks, line));
}

fn execute(input: &str, crane: fn(Stacks, &str) -> Stacks) -> String {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut break_line = 0;
    for (i, line) in lines.iter().enumerate() {
        if line.is_empty() {
            break_line = i;
            break;
        }
    }

    let mut stacks = Stacks::new(&lines[..break_line]);

    for i in (break_line + 1)..lines.len() {
        stacks = crane(stacks, lines[i]);
    }

    return stacks.peek_top();
}

struct Stacks {
    columns: Vec<Stack>,
}

impl Stacks {
    fn push(&mut self, cargo: char, stack_index: usize) {
        while self.columns.len() <= stack_index {
            self.columns.push(Stack::new());
        }

        self.columns[stack_index].push(cargo)
    }

    fn new(lines: &[&str]) -> Stacks {
        let mut response = Stacks {
            columns: Vec::new(),
        };

        for j in (0..(lines.len() - 1)).rev() {
            let line = lines[j];
            for i in (1..line.len()).step_by(4) {
                let c = line.chars().nth(i).unwrap();
                match c {
                    ' ' => continue,
                    _ => response.push(c, i / 4)
                }
            }
        }

        return response;
    }

    fn peek_top(self) -> String {
        return self.columns.iter().map(|c| c.peek_top()).collect();
    }
}

fn crane_mover_9000(mut stacks: Stacks, line: &str) -> Stacks {
    let parts = line.split(" ").collect::<Vec<&str>>();

    let amount = parts[1].parse::<usize>().unwrap();
    let from = parts[3].parse::<usize>().unwrap() - 1;
    let to = parts[5].parse::<usize>().unwrap() - 1;

    for _ in 0..amount {
        let cargo = stacks.columns[from].pop();
        stacks.push(cargo, to);
    }
    return stacks;
}

fn crane_mover_9001(mut stacks: Stacks, line: &str) -> Stacks {
    let parts = line.split(" ").collect::<Vec<&str>>();

    let amount = parts[1].parse::<usize>().unwrap();
    let from = parts[3].parse::<usize>().unwrap() - 1;
    let to = parts[5].parse::<usize>().unwrap() - 1;

    let mut on_crane: Vec<char> = Vec::new();
    for _ in 0..amount {
        on_crane.push(stacks.columns[from].pop());
    }

    for i in (0..amount).rev() {
        stacks.push(on_crane[i], to);
    }
    return stacks;
}

struct Stack {
    queue: Vec<char>,
}

impl Stack {
    fn new() -> Stack {
        Stack {
            queue: Vec::new(),
        }
    }

    fn push(&mut self, cargo: char) {
        self.queue.push(cargo);
    }

    fn pop(&mut self) -> char {
        self.queue.pop().unwrap()
    }

    fn peek_top(&self) -> char {
        self.queue.last().unwrap().clone()
    }
}

#[test]
fn test_first() {
    // given when
    let output = execute_first(&INPUT_SAMPLE);

    // then
    assert_eq!(output, "CMZ");
}

#[test]
fn test_second() {
    // given when
    let output = execute_second(&INPUT_SAMPLE);

    // then
    assert_eq!(output, "MCD");
}
