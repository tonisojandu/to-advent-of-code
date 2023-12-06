use regex::Regex;

pub fn solve(lines: Vec<String>, _: i32) {
    let whitespace = Regex::new(r"\s+").unwrap();
    let times = whitespace.split(&lines[0]).skip(1).map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let distances = whitespace.split(&lines[1]).skip(1).map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let num_races = times.len();

    let mut product: u64 = 1;

    for race in 0..num_races {
        let time = times[race];
        let distance = distances[race];

        let delta = calculate_wins(time as f64, distance as f64);

        product = product * (delta as u64);
    }

    let real_time: i64 = lines[0].replace("Time:", "").replace(" ", "").parse().unwrap();
    let real_distance: i64 = lines[1].replace("Distance:", "").replace(" ", "").parse().unwrap();

    let real_delta = calculate_wins(real_time as f64, real_distance as f64);

    println!("Day 06: 1 = {:?}", product);
    println!("Day 06: 2 = {:?}", real_delta);
}

fn calculate_wins(time: f64, distance: f64) -> i64 {
    let d = time * time / 4.0 - distance;
    let limit_1 = time / 2.0 - d.sqrt();
    let limit_2 = time / 2.0 + d.sqrt();

    let mut delta = 0;
    for i in ((-limit_2).floor() as i64)..((-limit_1).ceil() as i64) {
        if (i as f64) > (-limit_2) && (i as f64) < (-limit_1) {
            delta += 1;
        }
    }
    delta
}