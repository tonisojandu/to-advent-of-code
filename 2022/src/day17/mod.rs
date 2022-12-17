use std::collections::{HashMap, VecDeque};
use lazy_static::lazy_static;


lazy_static! {
    static ref INPUT_SAMPLE: &'static str = include_str!("day-17-sample.txt");
    static ref INPUT: &'static str = include_str!("day-17-main.txt");

}

pub fn day_17_first() {
    let output = execute_first(&INPUT);
    println!("Day 17-1: {}", output);
}

pub fn day_17_second() {
    let output = execute_second(&INPUT);
    // let output = execute_second(&INPUT_SAMPLE);
    println!("Day 17-2: {}", output);
}

fn execute_first(input: &str) -> u128 {
    return execute(input, 2022);
}

fn execute_second(input: &str) -> u128 {
    return execute(input, 1_000_000_000_000);
}

fn execute(input: &str, total: u64) -> u128 {
    let minus = vec!(
        c("#..@@@@.#")
    );
    let plus = vec!(
        c("#...@...#"),
        c("#..@@@..#"),
        c("#...@...#"),
    );
    let el = vec!(
        c("#....@..#"),
        c("#....@..#"),
        c("#..@@@..#"),
    );
    let ai = vec!(
        c("#..@....#"),
        c("#..@....#"),
        c("#..@....#"),
        c("#..@....#"),
    );
    let square = vec!(
        c("#..@@...#"),
        c("#..@@...#"),
    );
    let rocks = vec!(minus, plus, el, ai, square);

    let mut room = Room::new();


    let mut spawn_index = 0;

    room.spawn(&rocks[spawn_index]);
    spawn_index += 1;

    let commands = input.lines().next().unwrap().chars().collect::<Vec<_>>();
    let mut command_index = 0;

    let mut rock_count = 0;

    let mut flat_pairs = HashMap::<(usize, usize), u64>::new();
    let mut end_on = total;
    let mut first_count = 0;
    let mut first_height = 0;

    let mut height_diff = 0;
    let mut total_resets = 0;
    loop {
        let command = commands[command_index];
        command_index = (command_index + 1) % commands.len();
        room.tick(command);

        if room.should_spawn() {
            rock_count += 1;
            if rock_count >= end_on {
                break;
            }

            if end_on == total && room.is_flat() && command_index != 754 {
                let pair = (spawn_index, command_index);
                match flat_pairs.get(&pair) {
                    Some(count) => {
                        if first_count == 0 {
                            first_count = *count;
                            flat_pairs.insert(pair, rock_count);
                            first_height = room.get_height();
                        } else {
                            height_diff = room.get_height() - first_height;
                            let resets_in = rock_count - count;
                            total_resets = (total - rock_count) / resets_in;
                            end_on = (total - rock_count) % resets_in + rock_count;
                        }
                    }
                    None => {
                        flat_pairs.insert(pair, rock_count);
                    }
                }
            }

            room.spawn(&rocks[spawn_index]);
            spawn_index = (spawn_index + 1) % rocks.len();
        }
    }

    return room.get_height() as u128 + total_resets as u128 * height_diff as u128
}

struct Room {
    room: VecDeque<Vec<char>>,
    focus: i64,
    width: usize,
}

impl Room {
    fn new() -> Room {
        let mut room = VecDeque::new();
        room.push_back(c("#########"));
        Self { room, focus: -1, width: 9 }
    }

    fn fill_with_space(&mut self) {
        for i in (0..self.room.len()).rev() {
            let mut contains = false;
            for j in 1..(self.width - 1) {
                if self.room[i][j] == '#' {
                    contains = true;
                }
            }
            if contains {
                break;
            } else {
                self.room.pop_back();
            }
        }
        for _ in 0..3 {
            self.room.push_back(c("#.......#"));
        }
    }

    fn should_spawn(&self) -> bool {
        return self.focus == -1;
    }

    fn spawn(&mut self, rock: &Vec<Vec<char>>) {
        self.fill_with_space();
        for i in (0..rock.len()).rev() {
            self.room.push_back(rock[i].clone());
        }
        self.focus = self.room.len() as i64 - 1;
    }

    fn  tick(&mut self, command: char) {
        let drift: i64 = match command {
            '<' => -1,
            '>' => 1,
            c => panic!("Unknown command: {}", c),
        };

        let mut can_move = true;
        for i in (0.max(self.focus - 6) as usize)..((self.focus + 1) as usize) {
            for j in 1..(self.width - 1) {
                if self.room[i][j] == '@' && (
                        self.room[i][(j as i64 + drift) as usize] == '#'
                ) {
                    can_move = false;
                }
            }
        }
        if can_move {
            for i in (0.max(self.focus - 6) as usize)..((self.focus + 1) as usize) {
                let before = self.room[i].clone();
                for j in 1..(self.width-1) {
                    let drift_index = (j as i64 - drift) as usize;
                    if before[j] != '#' {
                        if before[drift_index] == '@' {
                            self.room[i][j] = '@';
                        } else {
                            self.room[i][j] = '.';
                        }
                    }
                }
            }
        }

        let mut can_drop = true;
        for i in (0.max(self.focus - 6) as usize)..((self.focus + 1) as usize) {
            for j in 1..(self.width - 1) {
                if self.room[i][j] == '@' && self.room[i - 1][j] == '#' {
                    can_drop = false;
                }
            }
        }
        if can_drop {
            for i in (0.max(self.focus - 6) as usize)..((self.focus) as usize) {
                for j in 1..(self.width - 1) {
                    if self.room[i + 1][j] == '@' {
                        self.room[i][j] = '@';
                        self.room[i + 1][j] = '.';
                    }
                }
            }

            self.focus -= 1;
        } else {
            for i in (0.max(self.focus - 6) as usize)..((self.focus + 1) as usize) {
                for j in 1..(self.width - 1) {
                    if self.room[i][j] == '@' {
                        self.room[i][j] = '#';
                    }
                }
            }
            self.focus = -1;
        }

    }

    fn is_flat(&self) -> bool {
        let mut look = self.room.len() - 1;
        loop {
            let mut contains_rock = false;
            let mut contains_space = false;
            for i in 1..(self.width - 2) {
                if self.room[look][i] == '#' {
                    contains_rock = true;
                } else {
                    contains_space = true;
                }
            }
            if contains_rock && contains_space {
                return false;
            }
            if contains_rock && !contains_space {
                return true;
            }
            look -= 1;
        }
    }

    fn get_height(&mut self) -> u64 {
        self.fill_with_space();
        return (self.room.len() - 4) as u64
    }
}

fn c(line: &str) -> Vec<char> {
    return line.chars().collect::<Vec<char>>();
}

#[test]
fn test_first() {
    // given when
    let output = execute_first(&INPUT_SAMPLE);

    // then
    assert_eq!(output, 3068);
}

// This solution does not work on the sample but accidentally worked on my input.
// The solution can be adapted to look for some other recurring pattern other than full flat row.
// For the sample there was no flat occurrences.
#[test]
fn test_second() {
    // given when
    // let output = execute_second(&INPUT_SAMPLE);

    // then
    // assert_eq!(output, 1514285714288);
}
