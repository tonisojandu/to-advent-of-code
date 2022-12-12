use std::collections::HashSet;
use lazy_static::lazy_static;
use grid::Grid;

lazy_static! {
    static ref INPUT_SAMPLE: &'static str = include_str!("day-12-sample.txt");
    static ref INPUT: &'static str = include_str!("day-12-main.txt");
}

pub fn day_12_first() {
    let output = execute_first(&INPUT);
    println!("Day 12-1: {}", output);
}

pub fn day_12_second() {
    let output = execute_second(&INPUT);
    println!("Day 12-2: {}", output);
}

fn execute_first(input: &str) -> i32 {
    let map = parse_map(input);

    let mut current_position = (0, 0);
    for y in 0..map.size().0 {
        for x in 0..map.size().1 {
            match map[y][x] {
                83 => current_position = (x, y),
                _ => (),
            }
        }
    }

    return search_from(&map, current_position);
}

fn execute_second(input: &str) -> i32 {
    let map = parse_map(input);

    let mut start_positions = vec!();
    for y in 0..map.size().0 {
        for x in 0..map.size().1 {
            match map[y][x] {
                97 => start_positions.push((x, y)),
                _ => (),
            }
        }
    }

    let mut min = i32::MAX;

    for start_position in start_positions {
        let distance = search_from(&map, start_position);
        if distance < min {
            min = distance;
        }
    }

    return min;
}

fn parse_map(input: &str) -> Grid<i32> {
    let mut map: Grid<i32> = Grid::new(0, 0);
    input.lines().for_each(|line| map.push_row(line.chars().map(|c| c as i32).collect()));
    map
}

fn search_from(map: &Grid<i32>, current_position: (usize, usize)) -> i32 {
    let map_size = map.size();
    let mut distance_map = Grid::init(map_size.0, map_size.1, -1);
    distance_map[current_position.1][current_position.0] = 0;
    let mut distance = 0;
    let mut search_space = HashSet::new();
    search_space.insert(current_position.clone());

    loop {
        let mut new_search_space = HashSet::new();
        if search_space.len() == 0 {
            return i32::MAX;
        }
        for (x, y) in search_space {
            distance_map[y][x] = distance;
            let mut current_height = height(&map, y, x);
            if current_height == 83 {
                current_height = 97
            }
            if map[y][x] == 69 {
                return distance;
            }

            if y < map_size.0 - 1 && distance_map[y + 1][x] < 0 && ((height(&map, y + 1, x) - current_height) < 2) {
                new_search_space.insert((x, y + 1));
            }
            if y > 0 && distance_map[y - 1][x] < 0 && ((height(&map, y - 1, x) - current_height) < 2) {
                new_search_space.insert((x, y - 1));
            }
            if x < map_size.1 - 1 && distance_map[y][x + 1] < 0 && ((height(&map, y, x + 1) - current_height) < 2) {
                new_search_space.insert((x + 1, y));
            }
            if x > 0 && distance_map[y][x - 1] < 0 && ((height(&map, y, x - 1) - current_height) < 2) {
                new_search_space.insert((x - 1, y));
            }
        }

        distance += 1;
        search_space = new_search_space;
    }
}

fn height(map: &Grid<i32>, y: usize, x: usize) -> i32 {
    let result = map[y][x];
    if result == 69 {
        return 122;
    }
    if result == 83 {
        return 97;
    }
    return result;
}

#[test]
fn test_first() {
    // given when
    let output = execute_first(&INPUT_SAMPLE);

    // then
    assert_eq!(output, 31);
}

#[test]
fn test_second() {
    // given when
    let output = execute_second(&INPUT_SAMPLE);

    // then
    assert_eq!(output, 29);
}
