use grid::Grid;
use lazy_static::lazy_static;

lazy_static! {
    static ref INPUT_SAMPLE: &'static str = include_str!("day-14-sample.txt");
    static ref INPUT: &'static str = include_str!("day-14-main.txt");
}

pub fn day_14_first() {
    let output = execute_first(&INPUT);
    println!("Day 14-1: {}", output);
}

pub fn day_14_second() {
    let output = execute_second(&INPUT);
    println!("Day 14-2: {}", output);
}

fn execute_first(input: &str) -> i32 {
    return execute(input, true);
}

fn execute_second(input: &str) -> i32 {
    return execute(input, false);
}

fn execute(input: &str, no_floor: bool) -> i32 {
    let mut rock_lines = Vec::<(usize, usize, usize, usize)>::new();

    let mut max_y = usize::MIN;

    for line in input.lines() {
        let coords: Vec<&str> = line.trim().split(" -> ").collect();
        for i in 1..coords.len() {
            let mut previous = coords[i-1].split(",");
            let mut current = coords[i].split(",");
            let rock_line = (
                previous.next().unwrap().parse::<usize>().unwrap(),
                previous.next().unwrap().parse::<usize>().unwrap(),
                current.next().unwrap().parse::<usize>().unwrap(),
                current.next().unwrap().parse::<usize>().unwrap(),
            );
            max_y = max_y.max(rock_line.1);
            max_y = max_y.max(rock_line.3);

            rock_lines.push(rock_line);
        }
    }
    max_y += 2;
    let min_x = 500 - max_y - 1;
    let max_x = 500 + max_y + 1;

    let mut map = Grid::<char>::init(max_y + 1, max_x - min_x, '.');
    for rock_line in rock_lines {
        let (x1, y1, x2, y2) = rock_line;
        if x1 == x2 {
            for y in y1.min(y2)..(y1.max(y2)+1) {
                map[y][x1 - min_x] = '#';
            }
        } else {
            for x in x1.min(x2)..(x1.max(x2)+1) {
                map[y1][x - min_x] = '#';
            }
        }
    }
    if !no_floor {
        for x in 0..(max_x - min_x) {
            map[max_y][x] = '#';
        }
    }

    let spawn_x = 500 - min_x;
    let mut sand_x = spawn_x;
    let mut sand_y = 0;

    let mut sand_rested = 0;
    loop {
        if map[sand_y + 1][sand_x] == '.' {
            sand_y += 1;
        } else if map[sand_y + 1][sand_x - 1] == '.' {
            sand_x -= 1;
            sand_y += 1;
        } else if map[sand_y + 1][sand_x + 1] == '.' {
            sand_x += 1;
            sand_y += 1;
        } else {
            map[sand_y][sand_x] = 'O';
            sand_rested += 1;
            sand_x = spawn_x;
            sand_y = 0;
            if map[sand_y][sand_x] != '.' {
                break;
            }
        }

        if no_floor && sand_y >= max_y-1 {
            break;
        }
    }

    return sand_rested;
}

#[test]
fn test_first() {
    // given when
    let output = execute_first(&INPUT_SAMPLE);

    // then
    assert_eq!(output, 24);
}

#[test]
fn test_second() {
    // given when
    let output = execute_second(&INPUT_SAMPLE);

    // then
    assert_eq!(output, 93);
}
