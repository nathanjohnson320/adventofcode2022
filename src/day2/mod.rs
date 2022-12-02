// total score =
// shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors)
// +  score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won)
const ROCK: &str = "rock";
const PAPER: &str = "paper";
const SCISSORS: &str = "scissors";
const WIN: &str = "win";
const LOSE: &str = "lose";
const DRAW: &str = "draw";

pub fn part1() {
    let input = super::files::read_lines("src/day2/input.txt");
    let rows: Vec<Vec<&str>> = input
        .iter()
        .map(|row| row.split(' ').map(|s| convert_to_play(s)).collect())
        .collect();

    let sum: i32 = rows.iter().map(|row| calculate_score(row)).sum();
    println!("{:?}", sum);
}

pub fn part2() {
    let input = super::files::read_lines("src/day2/input.txt");
    let rows: Vec<Vec<&str>> = input
        .iter()
        .map(|row| row.split(' ').map(|s| convert_to_play_2(s)).collect())
        .collect();

    let sum: i32 = rows.iter().map(|row| calculate_score_2(row)).sum();
    println!("{:?}", sum);
}

fn calculate_score(row: &Vec<&str>) -> i32 {
    let them = row[0];
    let you = row[1];

    // draws are 3
    if you == them {
        return 3 + play_to_score(you);
    }

    let choices = ["rock", "paper", "scissors"];
    let them_index = choices.iter().position(|&x| x == them).unwrap();
    let you_index = choices.iter().position(|&x| x == you).unwrap();

    if them_index == choices.len() - 1 && you_index == 0 {
        return 6 + play_to_score(you);
    }

    if you_index == choices.len() - 1 && them_index == 0 {
        return play_to_score(you);
    }
    if them_index > you_index {
        return play_to_score(you);
    } else {
        return 6 + play_to_score(you);
    }
}

fn calculate_score_2(row: &Vec<&str>) -> i32 {
    let them = row[0];
    let result = row[1];
    let you = find_play(them, result);

    if you == them {
        return 3 + play_to_score(you);
    }

    let choices = ["rock", "paper", "scissors"];
    let them_index = choices.iter().position(|&x| x == them).unwrap();
    let you_index = choices.iter().position(|&x| x == you).unwrap();

    if them_index == choices.len() - 1 && you_index == 0 {
        return 6 + play_to_score(you);
    }

    if you_index == choices.len() - 1 && them_index == 0 {
        return play_to_score(you);
    }
    if them_index > you_index {
        return play_to_score(you);
    } else {
        return 6 + play_to_score(you);
    }
}

fn find_play<'a>(their_play: &'a str, desired_result: &'a str) -> &'a str {
    match desired_result {
        WIN => {
            if their_play == ROCK {
                PAPER
            } else if their_play == PAPER {
                SCISSORS
            } else {
                ROCK
            }
        }
        LOSE => {
            if their_play == ROCK {
                SCISSORS
            } else if their_play == PAPER {
                ROCK
            } else {
                PAPER
            }
        }
        DRAW => their_play,
        _ => "BAD",
    }
}

fn play_to_score(play: &str) -> i32 {
    match play {
        "rock" => 1,
        "paper" => 2,
        "scissors" => 3,
        _ => 0,
    }
}

fn convert_to_play(play: &str) -> &str {
    match play {
        "A" => ROCK,
        "B" => PAPER,
        "C" => SCISSORS,
        "X" => ROCK,
        "Y" => PAPER,
        "Z" => SCISSORS,
        _ => "bad",
    }
}

fn convert_to_play_2(play: &str) -> &str {
    match play {
        "A" => ROCK,
        "B" => PAPER,
        "C" => SCISSORS,
        "X" => LOSE,
        "Y" => DRAW,
        "Z" => WIN,
        _ => "bad",
    }
}
