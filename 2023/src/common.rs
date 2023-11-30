pub trait Day {
    fn spawn(lines: Vec<String>) -> Self;

    fn solve(&self, part: i32);

}