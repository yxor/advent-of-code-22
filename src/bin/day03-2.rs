use std::collections::HashSet;
use std::fs;
use std::io;

fn get_file_content(path: &str) -> String {
    let f = fs::File::open(path).expect("Error opening file.");
    return io::read_to_string(f).expect("Error opening input file.");
}

fn main() {
    let input = get_file_content("./input/day03.txt");

    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let lines = input.lines();

    let line_char_sets = lines.into_iter().map(|line| {
        let char_set: HashSet<char> = HashSet::from_iter(line.chars().into_iter());
        return char_set;
    });

    let mut result = 0;

    let mut temp_hash_set: HashSet<char> = HashSet::new();

    for (i, line) in line_char_sets.enumerate() {
        if temp_hash_set.is_empty() {
            temp_hash_set.clone_from(&line);
        } else {
            let current_hash_set = temp_hash_set.clone();
            let intersection = current_hash_set.intersection(&line);
            temp_hash_set.clear();
            intersection.for_each(|value| {
                temp_hash_set.insert(*value);
            });
        }
        if (i + 1) % 3 == 0 {
            result += alphabet
                .chars()
                .position(|c| c == *temp_hash_set.iter().next().unwrap())
                .unwrap()
                + 1;

            temp_hash_set.clear();
        }
    }

    println!("{:?}", result) // 2683
}
