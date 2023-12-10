use std::collections::HashSet;

struct Pipeline {
    pipes: Vec<Vec<char>>,
}

impl Pipeline {
    fn can_go_up(&self, current: (usize, usize), came_from: (usize, usize)) -> bool {
        if current.0 == 0 {
            return false;
        }
        let new_coords = (current.0 - 1, current.1);
        let above = self.pipes[new_coords.0][new_coords.1];
        return new_coords != came_from && (above == '|' || above == '7' || above == 'F' || above == 'S');
    }

    fn can_go_down(&self, current: (usize, usize), came_from: (usize, usize)) -> bool {
        if current.0 == self.pipes.len() - 1 {
            return false;
        }
        let new_coords = (current.0 + 1, current.1);
        let below = self.pipes[new_coords.0][new_coords.1];
        return new_coords != came_from && (below == '|' || below == 'J' || below == 'L' || below == 'S');
    }

    fn can_go_left(&self, current: (usize, usize), came_from: (usize, usize)) -> bool {
        if current.1 == 0 {
            return false;
        }
        let new_coords = (current.0, current.1 - 1);
        let left = self.pipes[new_coords.0][new_coords.1];
        return new_coords != came_from && (left == '-' || left == 'F' || left == 'L' || left == 'S');
    }

    fn can_go_right(&self, current: (usize, usize), came_from: (usize, usize)) -> bool {
        if current.1 == self.pipes[current.0].len() - 1 {
            return false;
        }
        let new_coords = (current.0, current.1 + 1);
        let right = self.pipes[new_coords.0][new_coords.1];
        return new_coords != came_from && (right == '-' || right == '7' || right == 'J' || right == 'S');
    }
}

pub fn solve(lines: Vec<String>, _: i32) {
    let pipeline = Pipeline {
        pipes: lines.iter().map(|line| {
            line.chars().collect::<Vec<char>>()
        }).collect()
    };

    let mut start = (0, 0);
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            if pipeline.pipes[y][x] == 'S' {
                start = (y, x);
            }
        }
    }

    let mut part_of_main_loop = HashSet::new();
    part_of_main_loop.insert(start);

    let mut current = start;
    let mut came_from = (usize::MAX, usize::MAX);
    let mut depth = 0;
    loop {
        let current_pipe = pipeline.pipes[current.0][current.1];
        let can_go_up = pipeline.can_go_up(current, came_from);
        let can_go_down = pipeline.can_go_down(current, came_from);
        let can_go_left = pipeline.can_go_left(current, came_from);
        let can_go_right = pipeline.can_go_right(current, came_from);
        came_from = current;

        if current_pipe == 'S' {
            if can_go_down {
                current = (current.0 + 1, current.1);
            } else if can_go_right {
                current = (current.0, current.1 + 1);
            } else if can_go_up {
                current = (current.0 - 1, current.1);
            } else if can_go_left {
                current = (current.0, current.1 - 1);
            } else {
                panic!("Nowhere to go from start");
            }
        } else if current_pipe == 'L' {
            if can_go_up {
                current = (current.0 - 1, current.1);
            } else if can_go_right {
                current = (current.0, current.1 + 1);
            } else {
                panic!("Nowhere to go from L");
            }
        } else if current_pipe == 'J' {
            if can_go_left {
                current = (current.0, current.1 - 1);
            } else if can_go_up {
                current = (current.0 - 1, current.1);
            } else {
                panic!("Nowhere to go from J");
            }
        } else if current_pipe == '7' {
            if can_go_left {
                current = (current.0, current.1 - 1);
            } else if can_go_down {
                current = (current.0 + 1, current.1);
            } else {
                panic!("Nowhere to go from 7");
            }
        } else if current_pipe == 'F' {
            if can_go_right {
                current = (current.0, current.1 + 1);
            } else if can_go_down {
                current = (current.0 + 1, current.1);
            } else {
                panic!("Nowhere to go from F");
            }
        } else if current_pipe == '|' {
            if can_go_up {
                current = (current.0 - 1, current.1);
            } else if can_go_down {
                current = (current.0 + 1, current.1);
            } else {
                panic!("Nowhere to go from |");
            }
        } else if current_pipe == '-' {
            if can_go_left {
                current = (current.0, current.1 - 1);
            } else if can_go_right {
                current = (current.0, current.1 + 1);
            } else {
                panic!("Nowhere to go from -");
            }
        } else {
            panic!("Unknown pipe {}", current_pipe);
        }
        part_of_main_loop.insert(current);

        depth += 1;
        if pipeline.pipes[current.0][current.1] == 'S' {
            break;
        }
    }

    let mut outside = HashSet::new();

    for y in 0..lines.len() {
        if !part_of_main_loop.contains(&(y, 0)) {
            outside.insert((y, 0));
        }
        if !part_of_main_loop.contains(&(y, lines[y].len() - 1)) {
            outside.insert((y, lines[y].len() - 1));
        }
    }

    for x in 0..lines[0].len() {
        if !part_of_main_loop.contains(&(0, x)) {
            outside.insert((0, x));
        }
        if !part_of_main_loop.contains(&(lines.len() - 1, x)) {
            outside.insert((lines.len() - 1, x));
        }
    }

    let mut stop_checking = HashSet::new();
    loop {
        let mut add_new = vec![];
        for coords in outside.iter() {
            if stop_checking.contains(coords) {
                continue;
            }
            if coords.0 > 0 {
                let up = (coords.0 - 1, coords.1);
                if !part_of_main_loop.contains(&up) {
                    add_new.push(up);
                }
            }
            if coords.0 < lines.len() - 1 {
                let down = (coords.0 + 1, coords.1);
                if !part_of_main_loop.contains(&down) {
                    add_new.push(down);
                }
            }
            if coords.1 > 0 {
                let left = (coords.0, coords.1 - 1);
                if !part_of_main_loop.contains(&left) {
                    add_new.push(left);
                }
            }
            if coords.1 < lines[coords.0].len() - 1 {
                let right = (coords.0, coords.1 + 1);
                if !part_of_main_loop.contains(&right) {
                    add_new.push(right);
                }
            }
            if coords.0 > 0 && coords.1 > 0 {
                let up_left = (coords.0 - 1, coords.1 - 1);
                if !part_of_main_loop.contains(&up_left) {
                    add_new.push(up_left);
                }
            }
            if coords.0 > 0 && coords.1 < lines[coords.0].len() - 1 {
                let up_right = (coords.0 - 1, coords.1 + 1);
                if !part_of_main_loop.contains(&up_right) {
                    add_new.push(up_right);
                }
            }
            if coords.0 < lines.len() - 1 && coords.1 > 0 {
                let down_left = (coords.0 + 1, coords.1 - 1);
                if !part_of_main_loop.contains(&down_left) {
                    add_new.push(down_left);
                }
            }
            if coords.0 < lines.len() - 1 && coords.1 < lines[coords.0].len() - 1 {
                let down_right = (coords.0 + 1, coords.1 + 1);
                if !part_of_main_loop.contains(&down_right) {
                    add_new.push(down_right);
                }
            }
            stop_checking.insert(*coords);
        }
        if add_new.len() == 0 {
            break;
        }
        for coords in add_new.iter() {
            outside.insert(*coords);
        }
    }


    let mut inside_count = 0;
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            if !outside.contains(&(y, x)) && !part_of_main_loop.contains(&(y, x)) {
                let mut crossing_main_loop_count = 0;
                let mut saw_l = false;
                let mut saw_f = false;
                for x2 in 0..x {
                    if part_of_main_loop.contains(&(y, x2)) {
                        if pipeline.pipes[y][x2] == '|' {
                            crossing_main_loop_count += 1;
                        }
                        if pipeline.pipes[y][x2] == 'L' {
                            saw_l = true;
                        }
                        if pipeline.pipes[y][x2] == 'F' {
                            saw_f = true;
                        }
                        if pipeline.pipes[y][x2] == '7' {
                            if saw_l {
                                crossing_main_loop_count += 1;
                            }
                            saw_f = false;
                            saw_l = false;
                        }
                        if pipeline.pipes[y][x2] == 'J' {
                            if saw_f {
                                crossing_main_loop_count += 1;
                            }
                            saw_f = false;
                            saw_l = false;
                        }
                    }
                }
                if crossing_main_loop_count % 2 == 1 {
                    inside_count += 1;
                    print!("I")
                } else {
                    print!("O")
                }
            } else {
                if part_of_main_loop.contains(&(y, x)) {
                    print!("{}", pipeline.pipes[y][x]);
                } else {
                    print!(" ");
                }
            }
        }
        println!();
    }

    println!("Day 10: 1 = {:?}", depth / 2);
    println!("Day 10: 2 = {:?}", inside_count);
}