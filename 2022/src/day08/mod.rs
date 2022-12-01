use lazy_static::lazy_static;

lazy_static! {
    static ref INPUT_SAMPLE: &'static str = include_str!("day-08-sample.txt");
    static ref INPUT: &'static str = include_str!("day-08-main.txt");
}

pub fn day_08_first() {
    let output = execute_first(&INPUT);
    println!("Day 08-1: {}", output);
}

pub fn day_08_second() {
    let output = execute_second(&INPUT);
    println!("Day 08-2: {}", output);
}

fn execute_first(input: &str) -> u32 {
    let (length, width, tree_map) = read_trees(input);

    let going_right = iterate_right(length, width, &tree_map, std::cmp::max);
    let going_left = iterate_left(length, width, &tree_map, std::cmp::max);
    let going_down = iterate_down(length, width, &tree_map, std::cmp::max);
    let going_up = iterate_up(length, width, &tree_map, std::cmp::max);

    let mut visible_count = 0;
    for x in 1..(width - 1) {
        for y in 1..(length - 1) {
            if tree_map[y][x] > going_right[y][x] as u8 ||
                tree_map[y][x] > going_left[y][x] as u8 ||
                tree_map[y][x] > going_down[y][x] as u8 ||
                tree_map[y][x] > going_up[y][x] as u8 {
                visible_count += 1;
            }
        }
    }

    return (visible_count + (2 * length) + (2 * width) - 4) as u32;
}

/*
  I did not figure out how to dynamic program this :'(
  Not sure if this is even possible because neighbouring tree values are not sufficient to calculate
  the tree values. I failed to figure out a recursive function for this because, in some
  cases, previous local maximums need to be included for higher tree line of sights. However, not
  necessarily for neighbouring tree ones, because they may be lower than those previous local
  maximum ("blunt saw blades"). Thus, in some cases you have to look further than the neighbour
  again to calculate the tree value. So I just brute forced it to get it over with.
 */
fn execute_second(input: &str) -> u32 {
    let (length, width, tree_map) = read_trees(input);

    let max_y = length - 1;
    let max_x = width - 1;

    let mut max = 0;
    for y in 1..max_y {
        for x in 1..max_x {
            let height = tree_map[y][x];

            let mut look_right = 1;
            for x_i in (x+1)..max_x {
                if tree_map[y][x_i] < height {
                    look_right += 1;
                } else {
                    break;
                }
            }

            let mut look_left = 1;
            for x_i in (1..x).rev() {
                if tree_map[y][x_i] < height {
                    look_left += 1;
                } else {
                    break;
                }
            }

            let mut look_down = 1;
            for y_i in (y+1)..max_y {
                if tree_map[y_i][x] < height {
                    look_down += 1;
                } else {
                    break;
                }
            }

            let mut look_up = 1;
            for y_i in (1..y).rev() {
                if tree_map[y_i][x] < height {
                    look_up += 1;
                } else {
                    break;
                }
            }

            let result = look_right * look_left * look_down * look_up;
            if result > max {
                max = result;
            }
        }
    }
    return max;
}

fn read_trees(input: &str) -> (usize, usize, Vec<Vec<u8>>) {
    let lines: Vec<&str> = input.lines().collect();
    let length = lines.len();
    let width = lines[0].len();

    let mut tree_map = Vec::<Vec<u8>>::new();

    for line in lines {
        let mut row = Vec::<u8>::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as u8);
        }
        tree_map.push(row);
    }

    return (length, width, tree_map);
}

fn iterate_right(length: usize, width: usize, height_map: &Vec<Vec<u8>>, evaluator: fn(u32, u32) -> u32) -> Vec<Vec<u32>> {
    let mut min_height_map = empty_map(length, width);
    for y in 1..(length - 1) {
        for x in 1..(width - 1) {
            min_height_map[y][x] = evaluator(height_map[y][x - 1] as u32, min_height_map[y][x - 1]);
        }
    }
    return min_height_map;
}

fn iterate_left(length: usize, width: usize, height_map: &Vec<Vec<u8>>, evaluator: fn(u32, u32) -> u32) -> Vec<Vec<u32>> {
    let mut min_height_map = empty_map(length, width);
    for y in 1..(length - 1) {
        for x in (1..(width - 1)).rev() {
            min_height_map[y][x] = evaluator(height_map[y][x + 1] as u32, min_height_map[y][x + 1]);
        }
    }
    return min_height_map;
}

fn iterate_down(length: usize, width: usize, height_map: &Vec<Vec<u8>>, evaluator: fn(u32, u32) -> u32) -> Vec<Vec<u32>> {
    let mut min_height_map = empty_map(length, width);
    for x in 1..(width - 1) {
        for y in 1..(length - 1) {
            min_height_map[y][x] = evaluator( height_map[y - 1][x] as u32, min_height_map[y - 1][x]);
        }
    }
    return min_height_map;
}

fn iterate_up(length: usize, width: usize, height_map: &Vec<Vec<u8>>, evaluator: fn(u32, u32) -> u32) -> Vec<Vec<u32>> {
    let mut min_height_map = empty_map(length, width);
    for x in 1..(width - 1) {
        for y in (1..(length - 1)).rev() {
            min_height_map[y][x] = evaluator(height_map[y + 1][x] as u32, min_height_map[y + 1][x]);
        }
    }
    return min_height_map;
}

fn empty_map(length: usize, width: usize) -> Vec<Vec<u32>> {
    let mut map = Vec::<Vec<u32>>::new();
    for _ in 0..length {
        let mut row = Vec::<u32>::new();
        for _ in 0..width {
            row.push(0);
        }
        map.push(row);
    }
    return map;
}

#[test]
fn test_first() {
    // given when
    let output = execute_first(&INPUT_SAMPLE);

    // then
    assert_eq!(output, 21);
}

#[test]
fn test_second() {
    // given when
    let output = execute_second(&INPUT_SAMPLE);

    // then
    assert_eq!(output, 8);
}
