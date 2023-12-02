pub fn solve(lines: Vec<String>, _: i32) {
    let mut sum = 0;
    let mut power_sum = 0;

    for i in 0..lines.len() {
        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_green = 0;

        let line_parts: Vec<&str> = lines[i].split(":").collect();
        let sets = line_parts[1].split(";");

        let mut still_possible = true;

        for set in sets {
            let color_descriptions = set.split(",");

            for color_description in color_descriptions {
                let description_parts: Vec<&str> = color_description.trim().split(" ").collect();
                let count = description_parts[0].parse::<i32>().unwrap();
                let color = description_parts[1];

                match color {
                    "red" => {
                        max_red = max_red.max(count);
                        if count > 12 { still_possible = false;  }
                    },
                    "green" => {
                        max_green = max_green.max(count);
                        if count > 13 { still_possible = false; }
                    },
                    "blue" => {
                        max_blue = max_blue.max(count);
                        if count > 14 { still_possible = false; }
                    },
                    _ => {}
                }
            }
        }

        if still_possible {
            sum += i + 1;
        }

        let mut power = 1;
        if max_red > 0 {
            power *= max_red;
        }
        if max_blue > 0 {
            power *= max_blue;
        }
        if max_green > 0 {
            power *= max_green;
        }
        power_sum += power;

    }

    println!("Day 02: 1 = {:?}", sum);
    println!("Day 02: 2 = {:?}", power_sum);
}