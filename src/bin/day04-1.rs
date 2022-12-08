use std::fs;
use std::io;

fn get_file_content(path: &str) -> String {
    let f = fs::File::open(path).expect("Error opening file.");
    return io::read_to_string(f).expect("Error opening input file.");
}

struct Range {
    start: i32,
    end: i32,
}

fn range_fully_contains(a: Range, b: Range) -> bool {
    /*
    ...aaaaaaaaaaaa......
    .........bbbbb.......
    */
    (a.start <= b.start && a.end >= b.end) || 
    /*
    ...aaaaa........
    ..bbbbbbbb......
    */
    (b.start <= a.start && b.end >= a.end)
}

fn main() {
    let input = get_file_content("./input/day04.txt");

    let result: i32 = input
        .lines()
        .map(|line| {
            let ranges: Vec<&str> = line.split(',').collect();
            let first_range: Vec<i32> = ranges[0]
                .split('-')
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            let second_range: Vec<i32> = ranges[1]
                .split('-')
                .map(|s| s.parse::<i32>().unwrap())
                .collect();

            return match range_fully_contains(
                Range {
                    start: first_range[0],
                    end: first_range[1],
                },
                Range {
                    start: second_range[0],
                    end: second_range[1],
                },
            ) {
                true => 1,
                false => 0,
            };
        })
        .sum();

    println!("{:?}", result) // 562
}
