use std::fs;

use crate::day1::Config;

mod day1;

fn main() {
    let data = fs::read_to_string("day1input.txt").unwrap();
    let sum: u32 = data
        .lines()
        .map(|line| Config::from_str(line).to_num())
        .sum();
    println!("Sum of the configs is: {sum}");
}
