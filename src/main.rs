use std::fs;

use day1::find_calibration;

mod day1;

fn main() {
    let contents = fs::read_to_string("./data/day1p1.txt").unwrap();
    println!("day 1 part 1 : {}", find_calibration(contents.as_str(), false));
    println!("day 1 part 2 : {}", find_calibration(contents.as_str(), true));
}
