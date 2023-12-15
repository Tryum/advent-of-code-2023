use std::collections::HashMap;

use advent_of_code_2023::text_to_string_vec;

pub fn solve_day15_part1(input: &str) -> u64 {
    let mut iter = input.trim().split(',');
    let mut sum = 0;
    while let Some(s) = iter.next() {
        sum += get_hash(s) as u64; 
    }
    sum
}

pub fn solve_day15_part2(input: &str) -> u64 {
    let mut iter = input.trim().split(',');
    let mut sum = 0;
    let mut boxes : Vec<Vec<(&str, u8)>> = vec![Vec::new();256];
    while let Some(s) = iter.next() {
        if s.ends_with('-') {
            let (index, label) = parse_dash_operation(s);
            let lense_box = &mut boxes[index];
            if let Some(entry_index) = lense_box.iter().position(|(key, value)| *key == label) {
                lense_box.remove(entry_index);
            }
        }
        else {
            let (index, focal, label) = parse_equal_operation(s);
            let lense_box = &mut boxes[index];
            if let Some(entry_index) = lense_box.iter().position(|(key, value)| *key == label) {
                lense_box[entry_index] = (label, focal);
            }
            else {
                lense_box.push((label, focal));
            }

        }
    }

    for (i, b) in boxes.iter().enumerate() {
        for (j, f) in b.iter().enumerate() {
            let focal_power = (i+1) * (j+1) * (f.1 as usize);
            sum += focal_power as u64;
        }
    }

    sum
}

fn get_hash(input: &str) -> u8 {
    let mut current_value: u64 = 0;

    for c in input.chars() {
        current_value += c as u64;
        current_value *= 17;
        current_value %= 256;
    }

    current_value as u8
}

fn parse_equal_operation(input: &str) -> (usize, u8, &str) {
    let index = input.find('=').unwrap();
    let label = &input[0..index];
    let focal_length : u8 = input[index+1..].parse().unwrap();
    let hash = get_hash(label);
    (hash as usize, focal_length, label)
}

fn parse_dash_operation(input: &str) -> (usize, &str) {
    let label = &input[0..input.len()-1];
    (get_hash(&label) as usize, &input[0..input.len()-1])
}

#[cfg(test)]
mod tests{
    use crate::day15::{get_hash, solve_day15_part1, parse_equal_operation, parse_dash_operation};

    use super::solve_day15_part2;

    const EXAMPLE : &'static str =
"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
";

    #[test]
    fn test_day15_p1() {
        assert_eq!(solve_day15_part1(EXAMPLE), 1320);
    }

    #[test]
    fn test_day15_p2() {
        assert_eq!(solve_day15_part2(EXAMPLE), 145);
    }

    #[test]
    fn test_get_hash() {
        assert_eq!(get_hash("HASH"), 52);
        assert_eq!(get_hash("rn=1"), 30);
        assert_eq!(get_hash("cm-"), 253);
    }

    #[test]
    fn test_parse_equal_operation() {
        assert_eq!(parse_equal_operation("rn=1"), (0, 1, "rn"));
        assert_eq!(parse_equal_operation("qp=3"), (1, 3, "qp"));
    }

    #[test]
    fn test_parse_dash_operation() {
        assert_eq!(parse_dash_operation("qp-"), (1, "qp"));
        assert_eq!(parse_dash_operation("pc-"), (3, "pc"));
    }
}