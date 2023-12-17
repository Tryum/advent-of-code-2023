use std::fs;

use day1::find_calibration;

use crate::{day2::{Hand, solve_day2_part1, solve_day2_part2}, day3::solve_day3_part1, day4::{solve_day4_part1, solve_day4_part2}, day5::{solve_day5_part1, solve_day5_part2}, day6::{solve_day6_part1, solve_day6_part2}, day7::{solve_day7_part1, solve_day7_part2}, day8::{solve_day8_part1, solve_day8_part2}, day13::{solve_day13_part1, solve_day13_part2}, day14::{solve_day14_part1, solve_day14_part2}, day15::{solve_day15_part1, solve_day15_part2}, day12::solve_day12_part1, day16::{solve_day16_part1, solve_day16_part2}};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

mod day12;
mod day13;
mod day14;
mod day15;
mod day16;

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
    //println!("day 8 part 1: {}", solve_day8_part1(content.as_str()));
    //println!("day 8 part 2: {}", solve_day8_part2(content.as_str()));

    let content = fs::read_to_string("./data/day12.txt").unwrap();
    //println!("day 12 part 1: {}", solve_day12_part1(content.as_str()));

    let content = fs::read_to_string("./data/day13.txt").unwrap();
    println!("day 13 part 1: {}", solve_day13_part1(content.as_str()));
    println!("day 13 part 2: {}", solve_day13_part2(content.as_str()));

    let content = fs::read_to_string("./data/day14.txt").unwrap();
    // println!("day 14 part 1: {}", solve_day14_part1(content.as_str()));
    // println!("day 14 part 2: {}", solve_day14_part2(content.as_str()));

    let content = fs::read_to_string("./data/day15.txt").unwrap();
    println!("day 15 part 1: {}", solve_day15_part1(content.as_str()));
    println!("day 15 part 2: {}", solve_day15_part2(content.as_str()));

    let content = fs::read_to_string("./data/day16.txt").unwrap();
    println!("day 16 part 1: {}", solve_day16_part1(content.as_str()));
    println!("day 16 part 2: {}", solve_day16_part2(content.as_str()));
}
