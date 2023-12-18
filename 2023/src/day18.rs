use std::collections::HashSet;

pub fn solve(lines: Vec<String>, part: i32) {
    let mut edges: Vec<(i64, i64, i64, i64, usize)> = vec![];
    let mut critical_y: HashSet<i64> = HashSet::new();

    let mut max_x = i64::MIN;
    let mut max_y = i64::MIN;
    let mut min_x = i64::MAX;
    let mut min_y = i64::MAX;

    let mut currently_at = (0, 0);
    let mut index_of_line = 0;
    lines.iter().for_each(|line| {
        let new_coords = if part == 1 {
            let mut split = line.split(" ");

            let direction: char = split.next().unwrap().chars().next().unwrap();
            let distance: i64 = split.next().unwrap().parse::<i64>().unwrap();

            match direction {
                'U' => (currently_at.0 - distance, currently_at.1),
                'D' => (currently_at.0 + distance, currently_at.1),
                'L' => (currently_at.0, currently_at.1 - distance),
                'R' => (currently_at.0, currently_at.1 + distance),
                _ => panic!("Unknown direction {}", direction),
            }
        } else {
            let raw = line.split(" ").skip(2).next().unwrap();
            let hex = &raw[2..7];
            let direction = &raw[7..8];

            let distance: i64 = i64::from_str_radix(hex, 16).unwrap();

            match direction {
                "0" => (currently_at.0, currently_at.1 + distance), // right
                "1" => (currently_at.0 + distance, currently_at.1), // down
                "2" => (currently_at.0, currently_at.1 - distance), // left
                "3" => (currently_at.0 - distance, currently_at.1), // up
                _ => panic!("Unknown direction {}", direction),
            }
        };

        edges.push((currently_at.0, currently_at.1, new_coords.0, new_coords.1, index_of_line));
        critical_y.insert(currently_at.0);
        critical_y.insert(currently_at.0 + 1);
        index_of_line += 1;
        currently_at = new_coords;

        max_y = max_y.max(currently_at.0);
        max_x = max_x.max(currently_at.1);
        min_y = min_y.min(currently_at.0);
        min_x = min_x.min(currently_at.1);
    });

    let mut prioritized_edges = edges.clone();
    prioritized_edges.sort_by(|a, b| {
        let a_smaller_x = a.1.min(a.3);
        let b_smaller_x = b.1.min(b.3);
        let a_smaller_y = a.0.min(a.2);
        let b_smaller_y = b.0.min(b.2);
        if a_smaller_x == b_smaller_x {
            return a_smaller_y.cmp(&b_smaller_y);
        }
        return a_smaller_x.cmp(&b_smaller_x);
    });

    let mut inside = 0;

    let mut last_critical_row_result: i64 = 0;
    for y in min_y..(max_y + 1) {
        if critical_y.contains(&y) {
            last_critical_row_result = 0;
            let mut intersections = 0;
            let mut last_intersection = i64::MIN;
            for i in 0..prioritized_edges.len() {
                let edge = prioritized_edges[i];
                let is_horizontal = edge.0 == edge.2;
                let is_vertical = edge.1 == edge.3;
                if is_vertical {
                    let does_intersect = edge.0.min(edge.2) < y && y < edge.0.max(edge.2);
                    if does_intersect {
                        intersections += 1;
                        if intersections % 2 == 0 {
                            last_critical_row_result += edge.1 - last_intersection + 1;
                        }
                        last_intersection = edge.1;
                    }
                } else if is_horizontal {
                    let does_intersect = edge.0 == y;
                    if does_intersect {
                        let previous_index = if edge.4 == 0 { edges.len() - 1 } else { edge.4 - 1 };
                        let next_index = if edge.4 == edges.len() - 1 { 0 } else { edge.4 + 1 };

                        let previous_edge = edges[previous_index];
                        let next_edge = edges[next_index];

                        let previous_going_up = previous_edge.0 < y || previous_edge.2 < y;
                        let previous_going_down = previous_edge.0 > y || previous_edge.2 > y;
                        let next_going_up = next_edge.0 < y || next_edge.2 < y;
                        let next_going_down = next_edge.0 > y || next_edge.2 > y;

                        if previous_going_up == previous_going_down && previous_going_down == next_going_up && next_going_up == next_going_down {
                            panic!("All directions are the same");
                        }
                        if (previous_going_up && next_going_down) || (previous_going_down && next_going_up) {
                            intersections += 1;
                            if intersections % 2 == 0 {
                                last_critical_row_result += edge.1.max(edge.3) - last_intersection + 1
                            }
                            last_intersection = edge.1.min(edge.3);
                        } else if intersections % 2 == 0 {
                            last_critical_row_result += edge.1.max(edge.3) - edge.1.min(edge.3) + 1;
                        }
                    }
                }
            }
        }
        inside += last_critical_row_result;
    }
    println!("Day 18: {} = {:?}", part, inside);
}