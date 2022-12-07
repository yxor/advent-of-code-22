use std::fs;
use std::io;

fn get_file_content(path: &str) -> String {
    let f = fs::File::open(path).expect("Error opening file.");
    return io::read_to_string(f).expect("Error opening input file.");
}

fn main() {
    let input = get_file_content("./input/day03.txt");

    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let result: i32 = input
        .lines()
        .map(|line| {
            let (first_half, second_half) = line.split_at(line.len() / 2);
            let common_char = first_half
                .chars()
                .find(|&c| second_half.contains(c))
                .unwrap();

            return alphabet.chars().position(|c| c == common_char).unwrap() as i32 + 1;
        })
        .sum();

    println!("{}", result) // 7831
}
