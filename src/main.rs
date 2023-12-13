use std::fs;

use day1::find_calibration;

use crate::{day2::{Hand, solve_day2_part1, solve_day2_part2}, day3::{solve_day3_part1, day3_p1_debug}, day4::{solve_day4_part1, solve_day4_part2}, day5::{solve_day5_part1, solve_day5_part2}, day6::{solve_day6_part1, solve_day6_part2}, day7::{solve_day7_part1, solve_day7_part2}, day8::solve_day8_part1};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

fn main() {
    let content = fs::read_to_string("./data/day1p1.txt").unwrap();
    println!("day 1 part 1 : {}", find_calibration(content.as_str(), false));
    println!("day 1 part 2 : {}", find_calibration(content.as_str(), true));

    let content= fs::read_to_string("./data/day2.txt").unwrap();
    println!("day 2 part 1 : {}", solve_day2_part1(content.as_str(), Hand{red: 12, green: 13, blue: 14}));
    println!("day 2 part 2 : {}", solve_day2_part2(content.as_str()));

    let content = fs::read_to_string("./data/day3.txt").unwrap();
    println!("day 3 part 1: {}", solve_day3_part1(content.as_str()));
    //day3_p1_debug(&content);

    let content = fs::read_to_string("./data/day4.txt").unwrap();
    println!("day 4 part 1: {}", solve_day4_part1(content.as_str()));
    println!("day 4 part 2: {}", solve_day4_part2(content.as_str()));

    let content = fs::read_to_string("./data/day5.txt").unwrap();
    println!("day 5 part 1: {}", solve_day5_part1(&mut content.as_str().lines()));
    println!("day 5 part 2: {}", solve_day5_part2(&mut content.as_str().lines()));

    let content = fs::read_to_string("./data/day6.txt").unwrap();
    println!("day 6 part 1: {}", solve_day6_part1(&mut content.as_str().lines()));
    let content = fs::read_to_string("./data/day6p2.txt").unwrap();
    println!("day 6 part 2: {}", solve_day6_part2(&mut content.as_str().lines()));

    let content = fs::read_to_string("./data/day7.txt").unwrap();
    println!("day 7 part 1: {}", solve_day7_part1(content.as_str()));
    println!("day 7 part 2: {}", solve_day7_part2(content.as_str()));

    let content = fs::read_to_string("./data/day8.txt").unwrap();
    println!("day 8 part 1: {}", solve_day8_part1(content.as_str()));
}
