use std::collections::HashSet;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref INPUT_SAMPLE: &'static str = include_str!("day-15-sample.txt");
    static ref INPUT: &'static str = include_str!("day-15-main.txt");
}

pub fn day_15_first() {
    let output = execute_first(&INPUT, 2000000);
    println!("Day 15-1: {}", output);
}

pub fn day_15_second() {
    let output = execute_second(&INPUT, 4000000);
    println!("Day 15-2: {}", output);
}

fn execute_first(input: &str, check_line: i64) -> usize {
    let mut sensors = vec!();
    let mut beacons = vec!();
    for line in input.lines() {
        let (sensor, beacon) = Sensor::from(line);
        sensors.push(sensor);
        beacons.push(beacon);
    }

    let mut no_beacon_points = HashSet::new();

    for sensor in sensors {
        let mut d = 0;
        while dist(sensor.x, sensor.y, sensor.x + d, check_line) <= sensor.no_sensor_in {
            no_beacon_points.insert((sensor.x + d, check_line));
            no_beacon_points.insert((sensor.x - d, check_line));
            d = d + 1;
        }
    }

    for beacon in beacons {
        no_beacon_points.remove(&beacon);
    }

    return no_beacon_points.len();
}

fn execute_second(input: &str, max: usize) -> i64 {
    let mut sensors = vec!();
    for line in input.lines() {
        let (sensor, _) = Sensor::from(line);
        sensors.push(sensor);
    }

    let search = |x, y| {
        if x < 0 || y < 0 || x > max as i64 || y > max as i64 {
            return false;
        }
        for sensor in &sensors {
            if dist(sensor.x, sensor.y, x, y) <= sensor.no_sensor_in {
                return false;
            }
        }
        return true;
    };

    let result: fn(i64, i64) -> i64 = |x, y| {
        return x * 4000000 + y
    };

    for i in 0..sensors.len() {
        let observing_sensor = &sensors[i];

        for d_x in 0..observing_sensor.no_sensor_in {
            let mut x = observing_sensor.x - d_x;
            let mut y = observing_sensor.y + (observing_sensor.no_sensor_in - d_x) + 1;
            if search(x, y) {
                return result(x, y);
            }

            x = observing_sensor.x - d_x;
            y = observing_sensor.y - (observing_sensor.no_sensor_in - d_x) - 1;
            if search(x, y) {
                return result(x, y);
            }

            x = observing_sensor.x - d_x;
            y = observing_sensor.y + (observing_sensor.no_sensor_in - d_x) + 1;
            if search(x, y) {
                return result(x, y);
            }

            x = observing_sensor.x + d_x;
            y = observing_sensor.y + (observing_sensor.no_sensor_in - d_x) - 1;
            if search(x, y) {
                return result(x, y);
            }

            x = observing_sensor.x - d_x - 1;
            y = observing_sensor.y;
            if search(x, y) {
                return result(x, y);
            }

            x = observing_sensor.x + d_x + 1;
            y = observing_sensor.y;
            if search(x, y) {
                return result(x, y);
            }
        }
    }

    panic!("No point found");
}

struct Sensor {
    x: i64,
    y: i64,
    no_sensor_in: i64,
}

impl Sensor {
    fn from(line: &str) -> (Sensor, (i64, i64)) {
        let re = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
        let caps = re.captures(line).unwrap();
        let x = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let y = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
        let beacon_x = caps.get(3).unwrap().as_str().parse::<i64>().unwrap();
        let beacon_y = caps.get(4).unwrap().as_str().parse::<i64>().unwrap();
        return (Sensor {
            x,
            y,
            no_sensor_in: dist(x, y, beacon_x, beacon_y),
        }, (beacon_x, beacon_y));
    }
}

fn dist(x: i64, y: i64, x2: i64, y2: i64) -> i64 {
    return (x - x2).abs() + (y - y2).abs();
}

#[test]
fn test_first() {
    // given when
    let output = execute_first(&INPUT_SAMPLE, 10);

    // then
    assert_eq!(output, 26);
}

#[test]
fn test_second() {
    // given when
    let output = execute_second(&INPUT_SAMPLE, 20);

    // then
    assert_eq!(output, 56000011);
}
