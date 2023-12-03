use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

pub fn solve(lines: Vec<String>, _: i32) {
    let field: Vec<Vec<char>> = lines.iter().map(|line| {
        line.chars().collect::<Vec<char>>()
    }).collect();

    let mut ratios: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

    let width = field[0].len();
    let height = field.len();

    let mut sum = 0;

    let mut number_buffer = vec![];
    let mut encountered_symbol  = false;
    let mut encountered_gears: HashSet<(usize, usize)> = HashSet::new();
    for y in 0..height {
        for x in 0..width {
            // reset buffer if cause for it
            if !field[y][x].is_digit(10) || x == 0 {
                if number_buffer.len() > 0 {
                    let number = number_buffer.iter().collect::<String>().parse::<i32>().unwrap();

                    if encountered_symbol {
                        sum += number;
                    }
                    for gear in &encountered_gears {
                        let ratio = match ratios.get_mut(gear) {
                            Some(ratio) => ratio,
                            None => {
                                ratios.insert(*gear, vec![]);
                                ratios.get_mut(gear).unwrap()
                            }
                        };
                        ratio.push(number);
                    }

                }
                number_buffer.clear();
                encountered_symbol = false;
                encountered_gears.clear();
            }

            // fill buffer
            if field[y][x].is_digit(10) {
                number_buffer.push(field[y][x]);
            }

            // check for adjacent symbols
            if number_buffer.len() > 0 {
                for (o_x, o_y) in adjacent_coords(x, y, width, height) {
                    if !field[o_y][o_x].is_digit(10) && field[o_y][o_x] != '.' {
                        encountered_symbol = true;
                        if field[o_y][o_x] == '*' {
                            encountered_gears.insert((o_x, o_y));
                        }
                    }
                }
            }
        }
    }
    if number_buffer.len() > 0 && encountered_symbol {
        sum += number_buffer.iter().collect::<String>().parse::<i32>().unwrap();
    }

    let mut ratio_sum = 0;
    for (_, ratio) in ratios {
        if ratio.len() > 1 {
            ratio_sum += ratio.iter().fold(1, |acc, e| acc * e);
        }
    }

    println!("Day 03: 1 = {:?}", sum);
    println!("Day 03: 2 = {:?}", ratio_sum);
}

fn adjacent_coords(x: usize, y: usize, width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = vec![];
    for c_x in max(0, (x as i32)-1)..min(width as i32, (x as i32)+2) {
        for c_y in max(0, (y as i32)-1)..min(height as i32, (y as i32)+2) {
            if !(c_x == (x as i32) && c_y == (y as i32)) {
                result.push((c_x as usize, c_y as usize));
            }
        }
    }
    return result;
}