use std::ops::RemAssign;

use advent_of_code_2023::text_to_string_vec;
use colored::control;

pub fn solve_day12_part1(input: &str) -> u64 {
    let mut sum = 0;
    let mut count = 0;
    for l in input.lines() {
        if l.is_empty() {
            continue;
        }
        let result = get_arrangement_count2(l);
        sum += result;
        count+=1;
        //println!("line {count}, result {result}");
    }
    sum
}

pub fn solve_day12_part2(input: &str) -> u64 {
    let mut sum = 0;
    let mut count = 0;
    for l in input.lines() {
        if l.is_empty() {
            continue;
        }
        let new_rule = unfold_record(l);

        let result = get_arrangement_count2(new_rule.as_str());
        sum += result;
        count+=1;
        println!("line {count}, result {result}");
    }
    sum
}

fn get_arrangement_count(input: &str) -> u64 {
    let (pattern, control) = parse_input(input);
    let patterns = generate_patterns(pattern.len());
    let mut valid_result = 0;
    for p in patterns {
        if validate_picross_line(p.as_str(), &control) && validate_pattern(&pattern, p.as_str()) {
            valid_result += 1;
        }
    }
    valid_result
}

fn get_arrangement_count2(input: &str) -> u64 {
    let (pattern, control) = parse_input(input);
    let patterns = generate_patterns2(pattern);

    let mut valid_patterns = Vec::new();
    let mut queue = Vec::new();
    queue.push(&patterns);
    while let Some(p) = queue.pop() {
        if p.left.is_none() {
            if validate_picross_line(&p.pattern, &control) {
                valid_patterns.push(p.pattern.clone());
            }
        }
        else {
            let left_pattern = p.left.as_ref().unwrap().as_ref();
            let right_pattern = p.right.as_ref().unwrap().as_ref();
            queue.push(left_pattern);
            queue.push(right_pattern);
        }
    }

    valid_patterns.len() as u64
}

fn validate_picross_line(input: &str, control: &Vec<u32> ) -> bool {
    let mut count = 0;
    let mut result = Vec::new();
    for c in input.chars() {
        if c == '#' {
            count += 1;
        }
        else if count != 0 {
            result.push(count);
            count = 0;
        }
    }
    if count != 0 {
        result.push(count);
    }
    control == &result
}

fn parse_input(input: &str) -> (&str, Vec<u32>) {
    let mut iter = input.split(" ");
    let pattern = iter.next().unwrap();
    let control = iter.next().unwrap().split(',').map(|f| f.parse().unwrap()).collect();

    (pattern, control)
}

fn generate_patterns(size: usize) -> Vec<String> {
    let mut result = Vec::new();
    let format = 
    for i in 0..2_usize.pow(size as u32) {
        let mut binary = format!("{:b}", i);
        for i in binary.len()..size {
            binary.insert(0, '0');
        }
        binary = binary.replace("0", ".").replace("1", "#");
        result.push(binary);
    };
    result
}

struct Pattern {
    pattern : String,
    left: Option<Box<Self>>,
    right: Option<Box<Self>>
}

fn generate_patterns2(input: &str) -> Pattern {
    let mut p = Pattern {
        pattern : input.to_string(),
        left : None,
        right: None
    };

    for (i, c) in p.pattern.chars().enumerate() {
        if c == '?' {
            let mut left_str = p.pattern.clone();
            left_str.replace_range(i..i+1, ".");
            p.left = Some(Box::new(generate_patterns2(&left_str)));

            let mut right_str = p.pattern.clone();
            right_str.replace_range(i..i+1, "#");
            p.right = Some(Box::new(generate_patterns2(&right_str)));
            break;
        }
    }

    p
}

fn validate_pattern(control: &str, pattern: &str) -> bool {
    if pattern.len() != control.len() {
        return false;
    }

    for i in 0..pattern.len() {
        if control.chars().nth(i) != Some('?') && control.chars().nth(i) != pattern.chars().nth(i) {
            return false;
        }
    }
    true
}

fn unfold_record(input: &str) -> String {
    let mut iter = input.split(" ");
    let pattern = iter.next().unwrap();
    let control = iter.next().unwrap();

    let result = format!("{pattern}?{pattern}?{pattern}?{pattern}?{pattern} {control},{control},{control},{control},{control}");
    result
}

enum GroupType {
    Unknown,
    Blank,
    Plain
}

struct Group<'a> {
    pattern : &'a str,
    group_type : GroupType
}

fn split_groups(input: &str) -> Vec<Group> {
    let mut result = Vec::new();
    let mut prev_index = 0;
    let mut prev_char = input.chars().nth(0);
    for (i, c) in input.chars().enumerate() {
    }
    result
}

fn find_combination(input: &str) -> u32 {
    let (pattern, rules) = parse_input(input);



    0
}

#[cfg(test)]
mod tests{
    use crate::day12::{get_arrangement_count, validate_picross_line, parse_input, generate_patterns, solve_day12_part1, solve_day12_part2, unfold_record, find_combination};

    use super::validate_pattern;

    const EXAMPLE : &'static str =
"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

    #[test]
    fn test_day12_p1() {
        assert_eq!(solve_day12_part1(EXAMPLE), 21);
    }

    #[ignore = "too long"]
    #[test]
    fn test_day12_p2() {
        assert_eq!(solve_day12_part2(EXAMPLE), 525152);
    }

    #[test]
    fn test_get_arrangement_count() {
        assert_eq!(get_arrangement_count("???.### 1,1,3"), 1);
        assert_eq!(get_arrangement_count(".??..??...?##. 1,1,3"), 4);
        assert_eq!(get_arrangement_count("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
        assert_eq!(get_arrangement_count("????.#...#... 4,1,1"), 1);
        assert_eq!(get_arrangement_count("????.######..#####. 1,6,5"), 4);
        assert_eq!(get_arrangement_count("?###???????? 3,2,1"), 10);
    }

    #[test]
    fn test_validate_picross_line() {
        assert_eq!(validate_picross_line("#.#.#", &vec![1,1,1]), true);
        assert_eq!(validate_picross_line("#.#.#", &vec![1,3,1]), false);
        assert_eq!(validate_picross_line("##....#.####", &vec![2,1,4]), true);
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input("?###???????? 3,2,1"),("?###????????", vec![3,2,1]));
        assert_eq!(parse_input("?#?#?#?#?#?#?#? 1,3,1,6"),("?#?#?#?#?#?#?#?", vec![1,3,1,6]));
    }

    #[test]
    fn test_generate_patterns(){
        assert_eq!(generate_patterns(3), vec!["...", "..#", ".#.", ".##", "#..", "#.#", "##.", "###"]);
    }

    #[test]
    fn test_validate_pattern() {
        assert_eq!(validate_pattern("?###????????", ".###.##.#..."), true);
        assert_eq!(validate_pattern("?###????????", "#.##.##.#..."), false);
    }

    #[test]
    fn test_unfold_record() {
        assert_eq!(unfold_record(".# 1"), ".#?.#?.#?.#?.# 1,1,1,1,1".to_string());
    }

    #[ignore = "another day"]
    #[test]
    fn test_find_combination(){
        assert_eq!(find_combination("?###???????? 3,2,1"), 10);
    }
}