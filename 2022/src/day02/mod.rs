use lazy_static::lazy_static;

lazy_static! {
    static ref INPUT_SAMPLE: &'static str = include_str!("day-02-sample.txt");
    static ref INPUT: &'static str = include_str!("day-02-main.txt");
}

pub fn day_02_first() {
    let output = execute_first(&INPUT);
    println!("Day 02-1: {}", output);
}

pub fn day_02_second() {
    let output = execute_second(&INPUT);
    println!("Day 02-2: {}", output);
}

enum Vote {
    ROCK,
    PAPER,
    SCISSORS,
}

enum Result {
    WIN,
    LOSE,
    DRAW,
}

impl Vote {
    fn from(s: &str) -> Vote {
        match s {
            "A" => Vote::ROCK,
            "B" => Vote::PAPER,
            "C" => Vote::SCISSORS,
            "X" => Vote::ROCK,
            "Y" => Vote::PAPER,
            "Z" => Vote::SCISSORS,
            _ => panic!("Invalid vote: {}", s),
        }
    }

    fn score(&self) -> i32 {
        match &self {
            Vote::ROCK => 1,
            Vote::PAPER => 2,
            Vote::SCISSORS => 3,
        }
    }

    fn competition(&self, other: &Vote) -> i32 {
        let self_score = self.score();
        let other_score = other.score();

        // scissors loses to rock
        if self_score == 3 && other_score == 1 {
            return 0;
        }

        // rock wins over scissors
        if self_score == 1 && other_score == 3 {
            return 6;
        }

        // loss
        if self_score < other_score {
            return 0;
        }

        // win
        if self_score > other_score {
            return 6;
        }

        // draw
        return 3;
    }

    fn from_result(result: &Result, opponent_vote: &Vote) -> Vote {
        match (result, opponent_vote) {
            (Result::WIN, Vote::ROCK) => Vote::PAPER,
            (Result::WIN, Vote::PAPER) => Vote::SCISSORS,
            (Result::WIN, Vote::SCISSORS) => Vote::ROCK,
            (Result::DRAW, Vote::ROCK) => Vote::ROCK,
            (Result::DRAW, Vote::PAPER) => Vote::PAPER,
            (Result::DRAW, Vote::SCISSORS) => Vote::SCISSORS,
            (Result::LOSE, Vote::ROCK) => Vote::SCISSORS,
            (Result::LOSE, Vote::PAPER) => Vote::ROCK,
            (Result::LOSE, Vote::SCISSORS) => Vote::PAPER,
        }
    }
}

impl Result {
    fn from(s: &str) -> Result {
        match s {
            "X" => Result::LOSE,
            "Y" => Result::DRAW,
            "Z" => Result::WIN,
            _ => panic!("Invalid result: {}", s),
        }
    }
}

fn execute_first(input: &str) -> i32 {
    let lines = input.split("\n");

    let mut score = 0;

    for line in lines {
        if line.trim().len() != 0 {
            let votes = line.split(" ").collect::<Vec<&str>>();

            let opponent_vote = Vote::from(votes[0]);
            let my_vote = Vote::from(votes[1]);

            score += my_vote.score();
            score += my_vote.competition(&opponent_vote);
        }
    }

    return score;
}

fn execute_second(input: &str) -> i32 {
    let mut score = 0;

    for line in input.lines() {
        if line.trim().len() != 0 {
            let votes = line.split(" ").collect::<Vec<&str>>();

            let opponent_vote = Vote::from(votes[0]);
            let expected_result = Result::from(votes[1]);
            let my_vote = Vote::from_result(&expected_result, &opponent_vote);

            score += my_vote.score();
            score += my_vote.competition(&opponent_vote);
        }
    }

    return score;
}

#[test]
fn test_first() {
    // given when
    let output = execute_first(&INPUT_SAMPLE);

    // then
    assert_eq!(output, 15);
}

#[test]
fn test_second() {
    // given when
    let output = execute_second(&INPUT_SAMPLE);

    // then
    assert_eq!(output, 12);
}
