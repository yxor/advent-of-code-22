use std::fs;
use std::io;

const ROCK: i32 = 0;
const PAPER: i32 = 1;
const SCISSOR: i32 = 2;

const WIN_SCORE: i32 = 6;
const DRAW_SCORE: i32 = 3;
const LOSE_SCORE: i32 = 0;

fn get_file_content(path: &str) -> String {
    let f = fs::File::open(path).expect("Error opening file.");
    return io::read_to_string(f).expect("Error opening input file.");
}

fn get_move(m: char) -> i32 {
    match m {
        'A' => ROCK,
        'X' => ROCK,
        'B' => PAPER,
        'Y' => PAPER,
        'C' => SCISSOR,
        'Z' => SCISSOR,
        _ => panic!("Unexpected input"),
    }
}

fn main() {
    let input = get_file_content("./input/day02.txt");

    let result: i32 = input
        .split('\n')
        .map(|line| {
            let mut chars = line.chars();

            let other_move = get_move(chars.nth(0).expect("Invalid input"));
            let my_move = get_move(chars.nth(1).expect("Invalid input"));

            if (other_move + 1) % 3 == my_move {
                // I win
                return my_move + 1 + WIN_SCORE;
            }

            if my_move == other_move {
                // Draw
                return my_move + 1 + DRAW_SCORE;
            }

            // I lose
            return my_move + 1 + LOSE_SCORE;
        })
        .sum();

    println!("{:?}", result) // 12794
}
