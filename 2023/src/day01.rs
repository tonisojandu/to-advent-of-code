pub fn solve(lines: Vec<String>, part: i32) {
    let sum: u32 = lines.iter()
        .filter(|line| line.len() > 0)
        .map(|line| {
            if part == 2 {
                line.replace("one", "one1one")
                    .replace("two", "two2two")
                    .replace("three", "three3three")
                    .replace("four", "four4four")
                    .replace("five", "five5five")
                    .replace("six", "six6six")
                    .replace("seven", "seven7seven")
                    .replace("eight", "eight8eight")
                    .replace("nine", "nine9nine")
                    .replace("zero", "zero0zero")
            } else {
                line.to_string()
            }
        })
        .map(
            |line| {
                let only_digits = line.chars()
                    .filter(|c| c.is_digit(10))
                    .collect::<Vec<char>>();
                only_digits[0].to_digit(10).unwrap() * 10
                    + only_digits[only_digits.len() - 1].to_digit(10).unwrap()
            }
        ).sum();
    println!("Day 01: {} = {:?}", part, sum);
}