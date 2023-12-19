use std::io::BufRead;

use advent_of_code_2023::text_to_string_vec;

pub fn solve_day9_part1(input: &str) -> u64 {
    solve_part1(input) as u64
}

pub fn solve_day9_part2(input: &str) -> u64 {
    let mut inputs = Vec::new();
    for l in input.lines() {
        if l.is_empty() {
            continue;
        }
        inputs.push(parse_input(l));
    }

    let mut result = 0;
    for i in inputs {
        result += extrapolate2(i);
    }
    result as u64
}

fn derive_list(list: &Vec<i32>) -> Vec<i32>{
    let mut result = Vec::new();
    for i in 0..list.len()-1 {
        result.push(list[i+1]-list[i]);
    }
    result
}

fn extrapolate(list: Vec<i32>) -> i32 {
    let mut derived_list = Vec::new();
    derived_list.push(list.clone());
    let mut current = list;
    loop{
        let derived = derive_list(&current);
        
        derived_list.push(derived.clone());
        if derived.iter().all(|n| n == &0) {
            break;
        }
        current = derived;
    };

    let mut last_n = 0;

    for i in 1..derived_list.len() {
        let i = derived_list.len()-i-1;
        last_n = derived_list[i][derived_list[i].len()-1] + last_n;
    }

    last_n
}

fn extrapolate2(list: Vec<i32>) -> i32 {
    let mut derived_list = Vec::new();
    derived_list.push(list.clone());
    let mut current = list;
    loop{
        let derived = derive_list(&current);
        
        derived_list.push(derived.clone());
        if derived.iter().all(|n| n == &0) {
            break;
        }
        current = derived;
    };

    let mut last_n = 0;

    for i in 1..derived_list.len() {
        let i = derived_list.len()-i-1;
        last_n = derived_list[i][0] - last_n;
    }

    last_n
}

fn parse_input(input: &str) -> Vec<i32> {
    input.split_ascii_whitespace().map(| n | n.parse().unwrap()).collect()
}

fn solve_part1(input: &str) -> i32 {
    let mut inputs = Vec::new();
    for l in input.lines() {
        if l.is_empty() {
            continue;
        }
        inputs.push(parse_input(l));
    }

    let mut result = 0;
    for i in inputs {
        result += extrapolate(i);
    }
    result
}

#[cfg(test)]
mod tests{
    use crate::day9::{solve_day9_part1, derive_list, extrapolate, solve_day9_part2, extrapolate2};

    const EXAMPLE : &'static str =
"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    #[test]
    fn test_day9_p1() {
        assert_eq!(solve_day9_part1(EXAMPLE), 114);
    }

    #[test]
    fn test_day9_p2() {
        assert_eq!(solve_day9_part2(EXAMPLE), 2);
    }

    #[test]
    fn test_derive_list(){
        assert_eq!(derive_list(&vec![0, 3, 6, 9, 12, 15]), vec![3, 3, 3, 3, 3]);
    }

    #[test]
    fn test_extrapolate(){
        assert_eq!(extrapolate(vec![0, 3, 6, 9, 12, 15]), 18);
        assert_eq!(extrapolate(vec![10, 13, 16, 21, 30, 45]), 68);
    }

    #[test]
    fn test_extrapolate2(){
        assert_eq!(extrapolate2(vec![10, 13, 16, 21, 30, 45]), 5);
    }
}