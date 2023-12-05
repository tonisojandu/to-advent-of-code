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

    fn apply(&self, value: u64) -> Option<u64> {
        if value >= self.source && value < self.source + self.range {
            Some(self.destination + value - self.source)
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
        MapperSet { mappers, }
    }

    fn apply(&self, value: u64) -> u64 {
        for mapper in &self.mappers {
            if let Some(result) = mapper.apply(value) {
                return result;
            }
        }
        return value;
    }
}

pub fn solve(lines: Vec<String>, _: i32) {
    let seeds = lines[0].split(" ").skip(1).map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    let more_seeds = seeds.windows(2).step_by(2).flat_map(|pair| pair[0]..(pair[0] + pair[1])).collect::<Vec<u64>>();

    let groups = lines.split(|line| line.len() == 0).collect::<Vec<&[String]>>();

    let sets = groups.iter().skip(1)
        .map(|group| MapperSet::from(*group))
        .collect::<Vec<MapperSet>>();

    let min_location = seeds.iter().map(|seed| {
        sets.iter().fold(*seed, |value, set| set.apply(value))
    }).min().unwrap();

    let total_seeds = more_seeds.len();
    let more_min_location = more_seeds.iter().enumerate().map(|(index, seed)| {
        if index % 1_000_000 == 0 {
            println!("{} / {}", index, total_seeds);
        }
        sets.iter().fold(*seed, |value, set| set.apply(value))
    }).min().unwrap();

    println!("Day 05: 1 = {:?}", min_location);
    println!("Day 05: 2 = {:?}", more_min_location);
}