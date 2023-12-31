use std::collections::HashMap;

use colored::Colorize;
use regex::Regex;

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
    let mut nodes = Vec::new();
    for (k, _) in map.clone() {
        if k.ends_with('A') {
            nodes.push(k);
        }
    }

    println!("String nodes : {:?}", nodes);

    let mut cycle = vec![HashMap::<&str,Vec<usize>>::new(); nodes.len()];
    let mut prev_nodes = nodes.clone();

    let mut last_step = vec![0;nodes.len()];

    loop {
        for (i, c) in instructions.chars().enumerate() {
            steps += 1;
            for (j, node) in nodes.iter_mut().enumerate() {
                let node_cycle = cycle[j].entry(&node).or_default();

                if node.ends_with('Z') {
                    println!("Ghost {} on exit node {} at step {} on inst {}. Steps in cycle {}", j, node, steps, i, steps - last_step[j]);
                    last_step[j] = steps;
                }


                // if node_cycle.contains(&i) {
                //     let node = if node.ends_with('Z') { node.red() } else { node.white() };
                //     println!("ghost {} cycles at step {}, i={}, node={}, prev_node={}", j, steps, i, node, prev_nodes[j]);
                // }
                // else {
                //     node_cycle.push(i);
                // }

                prev_nodes[j] = node;

                let path = map.get(node).unwrap();
                if c == 'L' {
                    *node = path.0;
                }
                else {
                    assert_eq!(c, 'R');
                    *node = path.1;
                }
            }
            steps += 1;

            let mut exit= true;
            for node in nodes.clone() {
                if ! node.ends_with('Z') {
                    exit = false;
                    break;
                }
            };

            if exit {
                return steps;
            }
            
            
        };
    };
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
    use crate::day8::solve_day8_part2;

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

const EXAMPLE3 : &'static str =
"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

    #[test]
    fn test_day8_p1() {
        assert_eq!(solve_day8_part1(EXAMPLE), 2);
        assert_eq!(solve_day8_part1(EXAMPLE2), 6);
    }

    #[ignore]
    #[test]
    fn test_day8_p2() {
        assert_eq!(solve_day8_part2(EXAMPLE3), 6);
    }

    #[test]
    fn test_parse_node() {
        assert_eq!(parse_node("AAA = (BBB, CCC)"), ("AAA", "BBB", "CCC"));
        assert_eq!(parse_node("CCC = (ZZZ, GGG)"), ("CCC", "ZZZ", "GGG"));
    }
}