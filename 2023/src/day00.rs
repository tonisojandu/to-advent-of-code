use crate::common::Day;

pub struct Day00 {
    lines: Vec<String>,
}

impl Day for Day00 {
    fn spawn(lines: Vec<String>) -> Self {
        return Day00 { lines };
    }

    fn solve(&self, part: i32) {
        println!("Day 00: {} = {:?}", part, self.lines.len());
    }
}