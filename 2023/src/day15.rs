use std::collections::VecDeque;

struct Lens {
    label: String,
    focal_length: i64,
}

struct LensBox {
    number: usize,
    stack: VecDeque<Lens>,
}

impl LensBox {
    fn new(number: usize) -> LensBox {
        LensBox {
            number,
            stack: VecDeque::new(),
        }
    }

    fn offer(&mut self, offered: Lens) {
        match self.stack.iter_mut().find(|lens| lens.label == offered.label) {
            Some(found) => {
                found.focal_length = offered.focal_length;
            }
            None => {
                self.stack.push_back(offered);
            }
        }
    }

    fn remove(&mut self, label: String) {
        match self.stack.iter().position(|lens| lens.label == label) {
            Some(found) => {
                self.stack.remove(found);
            }
            None => {}
        }
    }

    fn calculate_power(&self) -> i64 {
        self.stack.iter()
            .enumerate()
            .map(|(i, lens)| { (self.number as i64 + 1) * (i + 1) as i64 * lens.focal_length })
            .sum()
    }
}

fn hash(to_hash: &String) -> usize {
    let mut result = 0;
    to_hash.chars().for_each(|c| {
        result += c as usize;
        result *= 17;
        result %= 256;
    });
    return result;
}


pub fn solve(lines: Vec<String>, _: i32) {
    let part1sum: usize = lines[0].split(",").map(|s| hash(&s.to_string())).sum();

    let mut boxes: Vec<LensBox> = (0..256).map(|i| LensBox::new(i as usize)).collect();

    lines[0].split(",").for_each(|s| {
        if s.ends_with("-") {
            let label = s[0..s.len() - 1].to_string();
            let hash = hash(&label);
            boxes[hash].remove(label);
        } else {
            let mut split = s.split("=");
            let label = split.next().unwrap().to_string();
            let focal_length = split.next().unwrap().parse::<i64>().unwrap();
            let hash = hash(&label);
            boxes[hash].offer(Lens {
                label,
                focal_length,
            });
        }
    });

    let part2sum: i64 = boxes.iter().map(|b| b.calculate_power()).sum();

    println!("Day 15: 1 = {:?}", part1sum);
    println!("Day 15: 2 = {:?}", part2sum);
}