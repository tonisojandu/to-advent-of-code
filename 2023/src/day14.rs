use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

pub fn solve(lines: Vec<String>, part: i32) {
    match part {
        1 => solve1(lines),
        2 => solve2(lines),
        _ => println!("Unknown part {}", part),
    }
}

pub fn solve1(lines: Vec<String>) {
    let rocks = lines.iter().map(|line| {
        line.chars().collect::<Vec<char>>()
    }).collect::<Vec<Vec<char>>>();
    let width: i64 = rocks[0].len() as i64;
    let height: i64 = rocks.len() as i64;

    let sum: i64 = (0..width).map(|x| {
        let mut column_load: i64 = 0;
        let mut last_cube_seen: i64 = -1;
        let mut rocks_stack: i64 = 0;

        for y in 0..height {
            match rocks[y as usize][x as usize] {
                '.' => {}
                '#' => {
                    last_cube_seen = y;
                    rocks_stack = 0;
                }
                'O' => {
                    rocks_stack += 1;
                    column_load += height - (last_cube_seen + rocks_stack);
                }
                _ => panic!("Unknown char {}", rocks[y as usize][x as usize]),
            }
        }

        column_load
    }).sum();

    println!("Day 14: 1 = {:?}", sum);
}

struct Platform {
    rocks: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

impl Platform {
    fn new(rocks: Vec<Vec<char>>) -> Platform {
        let height = rocks.len();
        let width = rocks[0].len();
        Platform {
            rocks,
            height,
            width,
        }
    }

    fn move_up(&mut self) {
        for x in 0..self.width {
            let mut last_cube_seen = 0;
            let mut rocks_stack = 0;
            for y in 0..self.height {
                match self.rocks[y][x] {
                    '.' => {}
                    '#' => {
                        last_cube_seen = y + 1;
                        rocks_stack = 0;
                    }
                    'O' => {
                        self.rocks[y][x] = '.';
                        self.rocks[last_cube_seen + rocks_stack][x] = 'O';
                        rocks_stack += 1;
                    }
                    _ => panic!("Unknown char {}", self.rocks[y][x]),
                }
            }
        }
    }
    fn move_left(&mut self) {
        for y in 0..self.height {
            let mut last_cube_seen = 0;
            let mut rocks_stack = 0;
            for x in 0..self.width {
                match self.rocks[y][x] {
                    '.' => {}
                    '#' => {
                        last_cube_seen = x + 1;
                        rocks_stack = 0;
                    }
                    'O' => {
                        self.rocks[y][x] = '.';
                        self.rocks[y][last_cube_seen + rocks_stack] = 'O';
                        rocks_stack += 1;
                    }
                    _ => panic!("Unknown char {}", self.rocks[y][x]),
                }
            }
        }
    }

    fn move_down(&mut self) {
        for x in 0..self.width {
            let mut last_cube_seen: i64 = (self.height - 1) as i64;
            let mut rocks_stack: i64 = 0;
            for y in (0..self.height).rev() {
                match self.rocks[y][x] {
                    '.' => {}
                    '#' => {
                        last_cube_seen = y as i64 - 1;
                        rocks_stack = 0;
                    }
                    'O' => {
                        self.rocks[y][x] = '.';
                        self.rocks[(last_cube_seen - rocks_stack) as usize][x] = 'O';
                        rocks_stack += 1;
                    }
                    _ => panic!("Unknown char {}", self.rocks[y][x]),
                }
            }
        }
    }

    fn move_right(&mut self) {
        for y in 0..self.height {
            let mut last_cube_seen: i64 = (self.width - 1) as i64;
            let mut rocks_stack: i64 = 0;
            for x in (0..self.width).rev() {
                match self.rocks[y][x] {
                    '.' => {}
                    '#' => {
                        last_cube_seen = x as i64 - 1;
                        rocks_stack = 0;
                    }
                    'O' => {
                        self.rocks[y][x] = '.';
                        self.rocks[y][(last_cube_seen - rocks_stack) as usize] = 'O';
                        rocks_stack += 1;
                    }
                    _ => panic!("Unknown char {}", self.rocks[y][x]),
                }
            }
        }
    }

    fn cycle(&mut self) {
        self.move_up();
        self.move_left();
        self.move_down();
        self.move_right();
    }

    fn hash(&self) -> u64 {
        let mut hahser = DefaultHasher::new();

        for y in 0..self.height {
            for x in 0..self.width {
                self.rocks[y][x].hash(&mut hahser);
            }
        }

        return hahser.finish();
    }

    fn weight(&self) -> usize {
        let mut weight = 0;

        for x in 0..self.width {
            for y in 0..self.height {
                if self.rocks[y][x] == 'O' {
                    weight += self.height - y;
                }
            }
        }

        return weight;
    }
}

fn solve2(lines: Vec<String>) {
    let mut platform = Platform::new(lines.iter().map(|line| {
        line.chars().collect::<Vec<char>>()
    }).collect::<Vec<Vec<char>>>());

    let mut number_last_seen: HashMap<u64, usize> = HashMap::new();

    let min_samples: usize = 1000;
    let mut last_has_seen_before = usize::MAX;
    let mut broke_sampling_at = 0;
    for iteration in 0..(min_samples * 2) {
        platform.cycle();
        let number = platform.hash();

        match number_last_seen.get_mut(&number) {
            Some(last_seen) => {
                if iteration > min_samples {
                    last_has_seen_before = last_seen.clone();
                    broke_sampling_at = iteration;
                }
            }
            None => {}
        }
        number_last_seen.insert(number, iteration);
    }

    let interval = broke_sampling_at - last_has_seen_before;

    let mut restart_from = broke_sampling_at;
    let complete_end = 1000000000;
    while restart_from + interval < complete_end {
        restart_from += interval;
    }
    for _ in (restart_from + 1)..complete_end {
        platform.cycle();
    }

    println!("Day 14: 2 = {:?}", platform.weight());
}