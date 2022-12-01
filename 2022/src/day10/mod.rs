use lazy_static::lazy_static;

lazy_static! {
    static ref INPUT_SAMPLE: &'static str = include_str!("day-10-sample.txt");
    static ref INPUT: &'static str = include_str!("day-10-main.txt");
}

pub fn day_10_first() {
    let output = execute_first(&INPUT);
    println!("Day 10-1: {}", output);
}

pub fn day_10_second() {
    let output = execute_second(&INPUT);
    println!("Day 10-2: \n\n{}", output);
}

fn execute_first(input: &str) -> i32 {
    return execute(input , 0, filter_relevant_x);
}

fn execute_second(input: &str) -> String {
    return execute(input, "".to_string(), draw_pixel);
}

fn execute<T>(input: &str, mut result_builder: T, process: fn(T, i32, i32) -> T) -> T {
    let mut x: i32 = 1;
    let mut cycle = 1;

    for line in input.lines() {
        let mut split = line.split_whitespace();
        let command = split.next().unwrap();

        match command {
            "noop" => {
                result_builder = process(result_builder, x, cycle);
                cycle += 1;
            }
            "addx" => {
                let value = split.next().unwrap().parse::<i32>().unwrap();
                result_builder = process(result_builder, x, cycle);
                cycle += 1;
                result_builder = process(result_builder, x, cycle);
                cycle += 1;
                x += value;
            }
            _ => panic!("Unknown command: {}", command),
        }
    }
    return result_builder;
}

fn filter_relevant_x(sum: i32, x: i32, cycle_num: i32) -> i32 {
    if (cycle_num - 20) % 40 == 0 {
        return sum + x * cycle_num;
    }
    return sum;
}

fn draw_pixel(picture: String, x: i32, cycle_num: i32) -> String {
    let crt_pos = (cycle_num - 1) % 40;
    let should_newline = cycle_num % 40 == 0;

    let new_pixel = if crt_pos >= (x - 1) && crt_pos <= (x + 1) {
        if should_newline { "#\n" } else { "#" }.to_string()
    }
    else {
        if should_newline { ".\n" } else { "." }.to_string()
    };

    return format!("{}{}", picture, new_pixel);

}

#[test]
fn test_first() {
    // given when
    let output = execute_first(&INPUT_SAMPLE);

    // then
    assert_eq!(output, 13140);
}

#[test]
fn test_second() {
    // given when
    let output = execute_second(&INPUT_SAMPLE);
    println!("{}", output);

    // then
    assert_eq!(output, "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
");
}
