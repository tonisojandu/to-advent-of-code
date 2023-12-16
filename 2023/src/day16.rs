use std::collections::{HashMap, HashSet};
use hashlink::LinkedHashMap;

pub fn solve(lines: Vec<String>, _: i32) {
    let map = lines.iter().map(|line| {
        line.chars().collect::<Vec<char>>()
    }).collect::<Vec<Vec<char>>>();

    let context = Context::new(map);

    let first_result = visit(0, 0, 'r', &mut context.clone(), &mut LinkedHashMap::new());

    let mut max_result = first_result;

    for y in 1..context.height {
        let mut result = visit(y, 0, 'r', &mut context.clone(), &mut LinkedHashMap::new());
        max_result = max_result.max(result);

        result = visit(y, context.width - 1, 'l', &mut context.clone(), &mut LinkedHashMap::new());
        max_result = max_result.max(result);
    }

    for x in 0..context.width {
        let mut result = visit(0, x, 'd', &mut context.clone(), &mut LinkedHashMap::new());
        max_result = max_result.max(result);

        result = visit(context.height - 1, x, 'u', &mut context.clone(), &mut LinkedHashMap::new());
        max_result = max_result.max(result);
    }

    println!("Day 16: 1 = {:?}", first_result);
    println!("Day 16: 2 = {:?}", max_result);
}

#[derive(Clone)]
struct Context {
    map: Vec<Vec<char>>,
    height: i64,
    width: i64,
    energized: HashSet<(i64, i64)>,
    result_cache: HashMap<(i64, i64, char), i64>,
}

impl Context {
    fn new(map: Vec<Vec<char>>) -> Context {
        let height = map.len() as i64;
        let width = map[0].len() as i64;
        Context {
            map,
            height,
            width,
            energized: HashSet::new(),
            result_cache: HashMap::new(),
        }
    }
}

fn visit(y: i64, x: i64, direction: char, context: &mut Context, path: &mut LinkedHashMap<(i64, i64, char), bool>) -> i64 {
    // out-of-bounds
    if y < 0 || y >= context.height || x < 0 || x >= context.width {
        return 0;
    }

    // detected loop
    if path.contains_key(&(y, x, direction)) {
        return 0;
    }

    // is memoized
    if context.result_cache.contains_key(&(y, x, direction)) {
        return context.result_cache[&(y, x, direction)];
    }

    let mut contributed = if !context.energized.contains(&(y, x)) { 1 } else { 0 };
    context.energized.insert((y, x));
    path.insert((y, x, direction), true);

    contributed += match context.map[y as usize][x as usize] {
        '.' => {
            match direction {
                'u' => visit(y - 1, x, 'u', context, path),
                'd' => visit(y + 1, x, 'd', context, path),
                'l' => visit(y, x - 1, 'l', context, path),
                'r' => visit(y, x + 1, 'r', context, path),
                _ => panic!("Unknown direction {}", direction),
            }
        }
        '/' => {
            match direction {
                'u' => visit(y, x + 1, 'r', context, path),
                'd' => visit(y, x - 1, 'l', context, path),
                'l' => visit(y + 1, x, 'd', context, path),
                'r' => visit(y - 1, x, 'u', context, path),
                _ => panic!("Unknown direction {}", direction),
            }
        }
        '\\' => {
            match direction {
                'u' => visit(y, x - 1, 'l', context, path),
                'd' => visit(y, x + 1, 'r', context, path),
                'l' => visit(y - 1, x, 'u', context, path),
                'r' => visit(y + 1, x, 'd', context, path),
                _ => panic!("Unknown direction {}", direction),
            }
        }
        '|' => {
            match direction {
                'u' => visit(y - 1, x, 'u', context, path),
                'd' => visit(y + 1, x, 'd', context, path),
                'l' | 'r' => {
                    visit(y - 1, x, 'u', context, path) + visit(y + 1, x, 'd', context, path)
                }
                _ => panic!("Unknown direction {}", direction),
            }
        }
        '-' => {
            match direction {
                'l' => visit(y, x - 1, 'l', context, path),
                'r' => visit(y, x + 1, 'r', context, path),
                'u' | 'd' => {
                    visit(y, x - 1, 'l', context, path) + visit(y, x + 1, 'r', context, path)
                }
                _ => panic!("Unknown direction {}", direction),
            }
        }
        _ => panic!("Unknown char {}", context.map[y as usize][x as usize]),
    };

    context.result_cache.insert((y, x, direction), contributed);
    return contributed;
}