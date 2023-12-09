pub fn solve(lines: Vec<String>, _: i32) {
    let mut total_past: i64 = 0;
    let mut total_future: i64 = 0;
    lines.iter().for_each(|line|{
        let numbers = line.split(" ").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        let nexts = find_next(&numbers);
        total_past += nexts.0;
        total_future += nexts.1;
    });
    println!("Day 09: 1 = {:?}", total_future);
    println!("Day 09: 2 = {:?}", total_past);
}

fn find_next(numbers: &Vec<i64>) -> (i64, i64) {
    let mut diffs: Vec<i64> = vec![];
    for i in 1..numbers.len() {
        diffs.push(numbers[i] - numbers[i - 1]);
    }
    if diffs.iter().all(|diff| *diff == 0) {
        return (numbers[0], numbers[0]);
    }
    let next_diff = find_next(&diffs);
    return (numbers[0] - next_diff.0, numbers[numbers.len() - 1] + next_diff.1);
}