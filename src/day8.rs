use std::collections::HashMap;

use advent_of_code_2023::text_to_string_vec;

pub fn solve_day8_part1(input: &str) -> u64 {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap();
    lines.next();
    let mut map = HashMap::new();
    while let Some(line) = lines.next() {
        if line.is_empty() { break };
        let (root, left, right) = parse_node(line);
        map.insert(root, (left, right));
    };
    let mut steps = 0;
    let mut node = "AAA";
    loop {
        for c in instructions.chars() {
            steps += 1;
            let path = map.get(node).unwrap();
            if c == 'L' {
                node = path.0;
            }
            else {
                assert_eq!(c, 'R');
                node = path.1;
            }
            if node == "ZZZ" {
                return steps;
            }
        }
    }
}

pub fn solve_day8_part2(input: &str) -> u64 {
    0
}

fn parse_node(input: &str) -> (&str, &str, &str) {
    let mut input_split = input.split("=");
    let source = input_split.next().unwrap().trim();
    let left_right = input_split.next().unwrap().trim();
    let left_right = &left_right[1..left_right.len()-1];
    let mut left_right = left_right.split(',');
    let left = left_right.next().unwrap().trim();
    let right = left_right.next().unwrap().trim();

    (source, left, right)
}

#[cfg(test)]
mod tests{
    use super::{parse_node, solve_day8_part1};

    const EXAMPLE : &'static str =
"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

const EXAMPLE2 : &'static str =
"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

    #[test]
    fn test_day8_p1() {
        assert_eq!(solve_day8_part1(EXAMPLE), 2);
        assert_eq!(solve_day8_part1(EXAMPLE2), 6);
    }

    #[test]
    fn test_day8_p2() {
    }

    #[test]
    fn test_parse_node() {
        assert_eq!(parse_node("AAA = (BBB, CCC)"), ("AAA", "BBB", "CCC"));
        assert_eq!(parse_node("CCC = (ZZZ, GGG)"), ("CCC", "ZZZ", "GGG"));
    }
}