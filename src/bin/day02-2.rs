use std::fs;
use std::io;

const ROCK: i32 = 0;
const PAPER: i32 = 1;
const SCISSOR: i32 = 2;

const LOSE: char = 'X';
const DRAW: char = 'Y';
const WIN: char = 'Z';

const WIN_SCORE: i32 = 6;
const DRAW_SCORE: i32 = 3;
const LOSE_SCORE: i32 = 0;

fn get_file_content(path: &str) -> String {
    let f = fs::File::open(path).expect("Error opening file.");
    return io::read_to_string(f).expect("Error opening input file.");
}

fn get_expected_result(opponent_move: i32, outcome: char) -> i32 {
    match outcome {
        LOSE => (opponent_move + 5) % 3 + 1 + LOSE_SCORE,
        DRAW => opponent_move + 1 + DRAW_SCORE,
        WIN => (opponent_move + 1) % 3 + 1 + WIN_SCORE,
        _ => panic!("Unexpected input"),
    }
}

fn get_move(m: char) -> i32 {
    match m {
        'A' => ROCK,
        'B' => PAPER,
        'C' => SCISSOR,
        _ => panic!("Unexpected input"),
    }
}

fn main() {
    let input = get_file_content("./input/day02.txt");

    let result: i32 = input
        .split('\n')
        .map(|line| {
            let mut chars = line.chars();
            let opponent_move = get_move(chars.nth(0).expect("Invalid input"));
            return get_expected_result(opponent_move, chars.nth(1).expect("Invalid input"));
        })
        .sum();

    println!("{:?}", result) // 14979
}
