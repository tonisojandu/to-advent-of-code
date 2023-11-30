use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref INPUT_SAMPLE: &'static str = include_str!("day-19-sample.txt");
    static ref INPUT: &'static str = include_str!("day-19-main.txt");
}

pub fn day_19_first() {
    let output = execute_first(&INPUT);
    println!("Day 19-1: {}", output);
}

pub fn day_19_second() {
    let output = execute_second(&INPUT);
    println!("Day 19-2: {}", output);
}

fn execute_first(input: &str) -> u64 {
    let blueprints: Vec<Blueprint> = input.lines().map(Blueprint::from).collect();

    let mut result = 0;

    // blueprints.iter().for_each(|blueprint| {
    //     let geodes = execute(0, &blueprint, Resources::new());
    //     result += blueprint.blueprint_number * geodes;
    // });

    blueprints.iter().for_each(|blueprint| {
        let geodes = execute_game(&blueprint);
        result += blueprint.blueprint_number * geodes;
    });

    return result;
}

fn execute_second(input: &str) -> i32 {
    return input.len() as i32;
}

fn execute_game(blueprint: &Blueprint) {
    let mut resources = Resources::new();
    for i in 0..24 {
        if resources.build_geode_robot(blueprint) {
            continue
        }
        if resources.build_obsidian_robot(blueprint) {
            continue
        }

    }

    return.res

}

fn execute(time: u64, blueprint: &Blueprint, mut resources: Resources) -> u64 {
    if time >= 24 {
        return resources.geode;
    }

    let mut max = 0;

    let mut ore_resources = resources.clone();
    if ore_resources.build_ore_robot(blueprint) {
        max = max.max(execute(time + 1, blueprint, ore_resources));
    }

    let mut clay_resources = resources.clone();
    if clay_resources.build_clay_robot(blueprint) {
        max = max.max(execute(time + 1, blueprint, clay_resources));
    }

    let mut obsidian_resources = resources.clone();
    if obsidian_resources.build_obsidian_robot(blueprint) {
        max = max.max(execute(time + 1, blueprint, obsidian_resources));
    }

    let mut geode_resources = resources.clone();
    if geode_resources.build_geode_robot(blueprint) {
        max = max.max(execute(time + 1, blueprint, geode_resources));
    }

    resources.pass_minute();
    max = max.max(execute(time + 1, blueprint, resources));

    return max;
}

struct Blueprint {
    blueprint_number: u64,
    ore_for_ore_robot: u64,
    ore_for_clay_robot: u64,
    ore_for_obsidian_robot: u64,
    clay_for_obsidian_robot: u64,
    ore_for_geode_robot: u64,
    clay_for_geode_robot: u64,
}

impl Blueprint {
    fn from(line: &str) -> Blueprint {
        let re = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
        let caps = re.captures(line).unwrap();
        Self {
            blueprint_number: caps[1].parse().unwrap(),
            ore_for_ore_robot: caps[2].parse().unwrap(),
            ore_for_clay_robot: caps[3].parse().unwrap(),
            ore_for_obsidian_robot: caps[4].parse().unwrap(),
            clay_for_obsidian_robot: caps[5].parse().unwrap(),
            ore_for_geode_robot: caps[6].parse().unwrap(),
            clay_for_geode_robot: caps[7].parse().unwrap(),
        }
    }
}

struct Resources {
    ore: u64,
    clay: u64,
    obsidian: u64,
    geode: u64,
    ore_robot: u64,
    clay_robot: u64,
    obsidian_robot: u64,
    geode_robot: u64,
}

impl Resources {
    fn new() -> Resources {
        Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            ore_robot: 1,
            clay_robot: 0,
            obsidian_robot: 0,
            geode_robot: 0,
        }
    }

    fn pass_minute(&mut self) {
        self.ore += self.ore_robot;
        self.clay += self.clay_robot;
        self.obsidian += self.obsidian_robot;
        self.geode += self.geode_robot;
    }

    fn build_ore_robot(&mut self, blueprint: &Blueprint) -> bool {
        if self.ore >= blueprint.ore_for_ore_robot {
            self.pass_minute();
            self.ore -= blueprint.ore_for_ore_robot;
            self.ore_robot += 1;
            return true;
        }
        return false;
    }

    fn build_clay_robot(&mut self, blueprint: &Blueprint) -> bool {
        if self.ore >= blueprint.ore_for_clay_robot {
            self.pass_minute();
            self.ore -= blueprint.ore_for_clay_robot;
            self.clay_robot += 1;
            return true;
        }
        return false;
    }

    fn build_obsidian_robot(&mut self, blueprint: &Blueprint) -> bool {
        if self.ore >= blueprint.ore_for_obsidian_robot && self.clay >= blueprint.clay_for_obsidian_robot {
            self.pass_minute();
            self.ore -= blueprint.ore_for_obsidian_robot;
            self.clay -= blueprint.clay_for_obsidian_robot;
            self.obsidian_robot += 1;
            return true;
        }
        return false;
    }

    fn build_geode_robot(&mut self, blueprint: &Blueprint) -> bool {
        if self.ore >= blueprint.ore_for_geode_robot && self.obsidian >= blueprint.clay_for_geode_robot {
            self.pass_minute();
            self.ore -= blueprint.ore_for_geode_robot;
            self.obsidian -= blueprint.clay_for_geode_robot;
            self.geode_robot += 1;
            return true;
        }
        return false;
    }
}

impl Clone for Resources {
    fn clone(&self) -> Self {
        Self {
            ore: self.ore,
            clay: self.clay,
            obsidian: self.obsidian,
            geode: self.geode,
            ore_robot: self.ore_robot,
            clay_robot: self.clay_robot,
            obsidian_robot: self.obsidian_robot,
            geode_robot: self.geode_robot,
        }
    }
}

#[test]
fn test_first() {
    // given when
    let output = execute_first(&INPUT_SAMPLE);

    // then
    assert_eq!(output, 33);
}

#[test]
fn test_second() {
    // given when
    let output = execute_second(&INPUT_SAMPLE);

    // then
    assert_ne!(output, -1);
}
