use std::fs;

use day1::find_calibration;

use crate::{day2::{Hand, solve_day2_part1, solve_day2_part2}, day3::{solve_day3_part1, day3_p1_debug}};

mod day1;
mod day2;
mod day3;

fn main() {
    let content = fs::read_to_string("./data/day1p1.txt").unwrap();
    println!("day 1 part 1 : {}", find_calibration(content.as_str(), false));
    println!("day 1 part 2 : {}", find_calibration(content.as_str(), true));

    let content= fs::read_to_string("./data/day2.txt").unwrap();
    println!("day 2 part 1 : {}", solve_day2_part1(content.as_str(), Hand{red: 12, green: 13, blue: 14}));
    println!("day 2 part 2 : {}", solve_day2_part2(content.as_str()));

    let content = fs::read_to_string("./data/day3.txt").unwrap();
    println!("day 3 part 1: {}", solve_day3_part1(content.as_str()));
    day3_p1_debug(&content);
}
