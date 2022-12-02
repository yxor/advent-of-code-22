use std::fs;
use std::io;

fn get_file_content(path: &str) -> String {
    let f = fs::File::open(path).expect("Error opening file.");

    return io::read_to_string(f).expect("Error opening input file.");
}

fn main() {
    let input = get_file_content("./input/day01.txt");
    let input_lines = input.split('\n');

    let mut calories_per_elf: Vec<i32> = (0..1).collect();

    for line in input_lines {
        match line.parse::<i32>() {
            Err(_) => {
                calories_per_elf.push(0);
            }
            Ok(calories) => {
                let last_elf = calories_per_elf.pop().expect("failed");
                calories_per_elf.push(last_elf + calories)
            }
        }
    }
    calories_per_elf.sort();

    let result = (0..3)
        .map(|_| calories_per_elf.pop().expect("failed"))
        .fold(0, |a, b| a + b);
    println!("{:?}", result) // 205370
}
