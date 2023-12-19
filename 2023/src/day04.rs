use std::collections::HashSet;
use regex::Regex;

pub fn solve(lines: Vec<String>, _: i32) {
    let n = lines.len();
    let mut num_cards = Vec::new();
    for _ in lines.iter() {
        num_cards.push(1);
    }
    let mut total_score = 0;
    for i in 0..n {
        let line_parts = lines[i].split("|").collect::<Vec<&str>>();

        let whitespace = Regex::new(r"\s+").unwrap();
        let winning_number_strings = whitespace.split(line_parts[0].trim()).map(|s| s.to_string()).collect::<HashSet<String>>();
        let number_strings = whitespace.split(line_parts[1].trim()).map(|s| s.to_string()).collect::<Vec<String>>();

        let mut wins: usize = 0;
        for number in number_strings {
            if winning_number_strings.contains(&number) {
                wins += 1;
            }
        }

        for j in (i + 1)..(i + wins as usize + 1).min(num_cards.len()) {
            num_cards[j] += num_cards[i]
        }
        if wins > 0 {
            total_score += 2_i32.pow(wins as u32 - 1);
        }
    };

    let total_cards: usize = num_cards.iter().sum();

    println!("Day 04: 1 = {:?}", total_score);
    println!("Day 04: 2 = {:?}", total_cards);
}