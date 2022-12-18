use std::collections::{HashSet, VecDeque};

use lazy_static::lazy_static;

lazy_static! {
    static ref INPUT_SAMPLE: &'static str = include_str!("day-18-sample.txt");
    static ref INPUT: &'static str = include_str!("day-18-main.txt");
}

pub fn day_18_first() {
    let output = execute_first(&INPUT);
    println!("Day 18-1: {}", output);
}

pub fn day_18_second() {
    let output = execute_second(&INPUT);
    println!("Day 18-2: {}", output);
}

fn execute_first(input: &str) -> i32 {
    let coordinates = parse_coordinates(input);

    let mut surface_area = 0;
    for coord in &coordinates {
        let mut sides = 6;
        if coordinates.contains(&(coord.0 + 1, coord.1, coord.2)) {
            sides -= 1;
        }
        if coordinates.contains(&(coord.0 - 1, coord.1, coord.2)) {
            sides -= 1;
        }
        if coordinates.contains(&(coord.0, coord.1 + 1, coord.2)) {
            sides -= 1;
        }
        if coordinates.contains(&(coord.0, coord.1 - 1, coord.2)) {
            sides -= 1;
        }
        if coordinates.contains(&(coord.0, coord.1, coord.2 + 1)) {
            sides -= 1;
        }
        if coordinates.contains(&(coord.0, coord.1, coord.2 - 1)) {
            sides -= 1;
        }
        surface_area += sides;
    }

    return surface_area;
}

fn execute_second(input: &str) -> i32 {
    let coordinates = parse_coordinates(input);
    let mut boundries = Boundaries::new();

    coordinates.iter().for_each(|coord| boundries.update(coord));
    boundries.expand();

    let mut air = HashSet::new();
    for x in boundries.min_x..(boundries.max_x + 1) {
        for y in boundries.min_y..(boundries.max_y + 1) {
            air.insert((x, y, boundries.min_z));
            air.insert((x, y, boundries.max_z));
        }
        for z in boundries.min_z..(boundries.max_z + 1) {
            air.insert((x, boundries.min_y, z));
            air.insert((x, boundries.max_y, z));
        }
    }
    for y in boundries.min_y..(boundries.max_y + 1) {
        for z in boundries.min_z..(boundries.max_z + 1) {
            air.insert((boundries.min_x, y, z));
            air.insert((boundries.max_x, y, z));
        }
    }

    let mut check = VecDeque::<(i32, i32, i32)>::new();
    air.iter().for_each(|c| check.push_back(c.clone()));

    let mut lava_neighbors = 0;

    loop {
        match check.pop_front() {
            Some(checking) => {
                for mut neighbour in neighbours(&checking, &boundries) {
                    if coordinates.contains(&neighbour) {
                        lava_neighbors += 1;
                        continue;
                    }
                    if air.contains(&neighbour) {
                        continue;
                    }
                    check.push_back(neighbour.clone());
                    air.insert(neighbour.clone());
                }
            }
            None => break,
        };
    }

    return lava_neighbors;
}

fn neighbours(coords: &(i32, i32, i32), boundaries: &Boundaries) -> Vec<(i32, i32, i32)> {
    let (x, y, z) = *coords;
    let mut result = vec![];

    let neighbour = (x - 1, y.clone(), z.clone());
    if boundaries.within(&neighbour) { result.push(neighbour); }

    let neighbour = (x + 1, y.clone(), z.clone());
    if boundaries.within(&neighbour) { result.push(neighbour); }

    let neighbour = (x.clone(), y - 1, z.clone());
    if boundaries.within(&neighbour) { result.push(neighbour); }

    let neighbour = (x.clone(), y + 1, z.clone());
    if boundaries.within(&neighbour) { result.push(neighbour); }

    let neighbour = (x.clone(), y.clone(), z - 1);
    if boundaries.within(&neighbour) { result.push(neighbour); }

    let neighbour = (x.clone(), y.clone(), z + 1);
    if boundaries.within(&neighbour) { result.push(neighbour); }

    return result;
}

fn parse_coordinates(input: &str) -> HashSet<(i32, i32, i32)> {
    let mut coordinates = HashSet::new();
    for line in input.lines() {
        let mut split = line.split(",");
        coordinates.insert((
            split.next().unwrap().parse::<i32>().unwrap(),
            split.next().unwrap().parse::<i32>().unwrap(),
            split.next().unwrap().parse::<i32>().unwrap(),
        ));
    }
    coordinates
}

struct Boundaries {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    min_z: i32,
    max_z: i32,
}

impl Boundaries {
    fn new() -> Boundaries {
        Boundaries {
            min_x: i32::MAX,
            max_x: i32::MIN,
            min_y: i32::MAX,
            max_y: i32::MIN,
            min_z: i32::MAX,
            max_z: i32::MIN,
        }
    }

    fn update(&mut self, coord: &(i32, i32, i32)) {
        let (x, y, z) = *coord;
        if x < self.min_x {
            self.min_x = x.clone();
        }
        if x > self.max_x {
            self.max_x = x.clone();
        }
        if y < self.min_y {
            self.min_y = y.clone();
        }
        if y > self.max_y {
            self.max_y = y.clone();
        }
        if z < self.min_z {
            self.min_z = z.clone();
        }
        if z > self.max_z {
            self.max_z = z.clone();
        }
    }

    fn expand(&mut self) {
        self.min_x -= 1;
        self.min_y -= 1;
        self.min_z -= 1;
        self.max_x += 1;
        self.max_y += 1;
        self.max_z += 1;
    }

    fn within(&self, coords: &(i32, i32, i32)) -> bool {
        let (x, y, z) = *coords;
        return x >= self.min_x
            && x <= self.max_x
            && y >= self.min_y
            && y <= self.max_y
            && z >= self.min_z
            && z <= self.max_z;
    }
}

#[test]
fn test_first() {
    // given when
    let output = execute_first(&INPUT_SAMPLE);

    // then
    assert_eq!(output, 64);
}

#[test]
fn test_second() {
    // given when
    let output = execute_second(&INPUT_SAMPLE);

    // then
    assert_eq!(output, 58);
}
