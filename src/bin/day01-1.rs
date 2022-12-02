use std::fs;
use std::io;

fn get_file_content(path: &str) -> String {
    let f = fs::File::open(path).expect("Error opening file.");

    return io::read_to_string(f).expect("Error opening input file.");
}

fn main() {
    let input = get_file_content("./input/day01.txt");
    let input_lines = input.split('\n');

    let mut most_calories = 0;
    let mut current_calories = 0;
    for line in input_lines {
        match line.parse::<i32>() {
            Err(_) => {
                if current_calories > most_calories {
                    most_calories = current_calories;
                }
                current_calories = 0;
            }
            Ok(calories) => current_calories += calories,
        }
    }

    if current_calories > most_calories {
        most_calories = current_calories;
    }

    println!("{}", most_calories) // 68802
}
