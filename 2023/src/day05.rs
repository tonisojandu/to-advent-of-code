use std::collections::HashSet;

struct Mapper {
    source: u64,
    destination: u64,
    range: u64,
}

impl Mapper {
    fn from(line: &str) -> Self {
        let parts = line.split(" ").collect::<Vec<&str>>();
        Mapper {
            source: parts[1].parse::<u64>().unwrap(),
            destination: parts[0].parse::<u64>().unwrap(),
            range: parts[2].parse::<u64>().unwrap(),
        }
    }

    fn reverse(&self, value: u64) -> Option<u64> {
        if value >= self.destination && value < self.destination + self.range {
            Some(self.source + value - self.destination)
        } else {
            None
        }
    }
}

struct MapperSet {
    mappers: Vec<Mapper>,
}

impl MapperSet {
    fn from(lines: &[String]) -> Self {
        let mut mappers = Vec::new();
        for line in lines.iter().skip(1) {
            mappers.push(Mapper::from(line));
        }
        MapperSet { mappers }
    }

    fn reverse_apply(&self, value: u64) -> u64 {
        for mapper in &self.mappers {
            if let Some(result) = mapper.reverse(value) {
                return result;
            }
        }
        return value;
    }
}

pub fn solve(lines: Vec<String>, _: i32) {
    let raw_seeds = lines[0].split(" ").skip(1).map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    let seeds = lines[0].split(" ").skip(1).map(|s| s.parse::<u64>().unwrap()).collect::<HashSet<u64>>();
    let more_seeds = raw_seeds.windows(2).step_by(2)
        .map(|pair| (pair[0], pair[0] + pair[1]))
        .collect::<Vec<(u64, u64)>>();

    let groups = lines.split(|line| line.len() == 0).collect::<Vec<&[String]>>();

    let mut sets = groups.iter().skip(1)
        .map(|group| MapperSet::from(*group))
        .collect::<Vec<MapperSet>>();
    sets.reverse();

    let mut location_iter: u64 = 0;

    let mut min_location = u64::MAX;
    let mut more_min_location = u64::MAX;

    loop {
        if location_iter % 1_000_000 == 0 {
            println!("Trying {}", location_iter);
        }
        let test_seed = sets.iter().fold(location_iter, |value, set| set.reverse_apply(value));
        if min_location == u64::MAX && seeds.contains(&test_seed) {
            println!("Found seed at {}", location_iter);
            min_location = location_iter;
        }
        if more_min_location == u64::MAX {
            match more_seeds.iter().find(|interval| test_seed >= interval.0 && test_seed < interval.1) {
                Some(_) => {
                    println!("Found more_seed at {}", location_iter);
                    more_min_location = location_iter;
                }
                None => {}
            }
        }
        if min_location != u64::MAX && more_min_location != u64::MAX {
            break;
        }
        location_iter += 1;
    }

    println!("Day 05: 1 = {:?}", min_location);
    println!("Day 05: 2 = {:?}", more_min_location);
}