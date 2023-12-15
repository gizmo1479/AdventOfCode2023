use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const FILEPATH: &str = "input.txt";
fn calc_calibr(s: String) -> u32 {
    let parts: Vec<u32> = s.chars().filter_map(|s| s.to_digit(10)).collect();

    match parts.len() {
        0 => 0,
        1 => parts[0] * 10 + parts[0],
        x => parts[0] * 10 + parts[x - 1],
    }
}

fn main() {
    println!("Hello, world!");
    let mut cal_val = 0;

    let file = File::open(FILEPATH).unwrap();
    let r = BufReader::new(file);

    for line in r.lines() {
        cal_val += calc_calibr(line.unwrap());
    }

    println!("Final value: {}", cal_val);
}
