use std::fs::{self};

fn main() {
    let file = "data/data.txt";

    let sum: u32 = fs::read_to_string(file)
        .unwrap()
        .lines()
        .map(String::from)
        .map(|line| calculate_line(line))
        .sum();

    println!("{}", sum);
}

fn calculate_line(line: String) -> u32 {
    let result: Vec<u32> = line
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    result.first().unwrap() * 10 + result.last().unwrap()
}
