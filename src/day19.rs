use std::{process::id, collections::HashMap, ops::{Sub, Add}};

use itertools::Itertools;
use num_traits::PrimInt;

#[derive(PartialEq, Eq, Debug)]
struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
    exit: &'a str
}

#[derive(PartialEq, Eq, Debug)]
struct Rule<'a> {
    property: char,
    value:u32,
    operator: char,
    target: &'a str
}

#[derive(Clone, Copy, Debug)]
struct MinMax<T> {
    min : T,
    max : T
}

impl<T:PrimInt> MinMax<T> {
    fn new(min: T, max: T) -> Self {
        Self{
            min: min,
            max: max
        }
    }

    fn diff(self) -> T {
        T::from(1).unwrap() + self.max - self.min
    }
    
}

#[derive(Clone, Copy, Debug)]
struct Xmas {
    x : MinMax<u32>,
    m : MinMax<u32>,
    a : MinMax<u32>,
    s : MinMax<u32>,
}

impl Xmas {
    fn new() -> Self {
        Xmas {
            x : MinMax{ min: 1, max: 4000 },
            m : MinMax{ min: 1, max: 4000 },
            a : MinMax{ min: 1, max: 4000 },
            s : MinMax{ min: 1, max: 4000 },
        }
    }
}

pub fn solve_day19_part1(input: &str) -> u64 {
    part1_solver(input)
}

pub fn solve_day19_part2(input: &str) -> u64 {
    part2_solver(input)
}


fn part1_solver(input: &str) -> u64 {
    let mut line_iter = input.lines();
    let mut workflows = HashMap::new();
    let mut parts = Vec::new();
    let mut accepted_parts = Vec::new();
    while let Some(l) = line_iter.next() {
        if l.is_empty(){
            break;
        }
        let workflow = parse_workflow(l);
        workflows.insert(workflow.name, workflow);
    }
    while let Some(l) = line_iter.next() {
        if l.is_empty(){
            break;
        }
        let part = parse_part(l);
        parts.push(part);
    }

    let in_wf = workflows.get("in").unwrap();
    for p in &parts {
        let mut wf = in_wf;
        let mut next = in_wf.exit;
        loop {
            for r in &wf.rules {
                next = wf.exit;
                match r.operator {
                    '<' => {
                        if p[&r.property] < r.value {
                            next = r.target;
                            break;
                        }
                    },
                    '>' => {
                        if p[&r.property] > r.value {
                            next = r.target;
                            break;
                        }
                    },
                    _ => unimplemented!()
                }
            }
            match next {
                "A" => {
                    accepted_parts.push(p);
                    break;
                },
                "R" => {
                    break;
                },
                new_wf =>{
                    wf = workflows.get(new_wf).unwrap();
                }
            }
        }
    }
    let mut result = 0;
    for a in accepted_parts {
        let values : u32 = a.values().collect_vec().into_iter().sum();
        result += values as u64;
    }

    result
}

fn split_range<T:PrimInt>(range: &MinMax<T>, operator: char, value: T) -> (Option<MinMax<T>>, Option<MinMax<T>>) {
    match operator {
        '<' => {
            if range.min < value && value < range.max {
                let value_minus_one = value-T::from(1).unwrap();
                return (Some(MinMax::new(range.min, value_minus_one)), Some(MinMax::new(value, range.max)));
            }
            else if range.max < value {
                return (Some(*range), None);
            }
            else {
                return (None, None);
            }
        },
        '>' => {
            if range.min < value && value < range.max {
                let value_plus_one = value + T::from(1).unwrap();
                return (Some(MinMax::new(value_plus_one, range.max)), Some(MinMax::new(range.min, value)));
            }
            else if range.min > value {
                return (Some(*range), None);
            }
            else {
                return (None, None);
            }
        },
        _ => unimplemented!()
    }
}

fn part2_solver(input: &str) -> u64 {
    let mut line_iter = input.lines();
    let mut workflows = HashMap::new();
    while let Some(l) = line_iter.next() {
        if l.is_empty(){
            break;
        }
        let workflow = parse_workflow(l);
        workflows.insert(workflow.name, workflow);
    }

    let xmas = Xmas::new();
    let mut rules = Vec::new();
    rules.push(("in", xmas));

    let mut accepted_range = Vec::new();

    while let Some(rule) = rules.pop() {

        match rule.0 {
            "A" => {
                accepted_range.push(rule.1);
                continue;
            },
            "R" => {
                continue;
            }
            _ => {}
        }

        let wf = workflows.get(rule.0).unwrap();
        let mut xmas = rule.1;

        for r in &wf.rules {
            match r.property {
                'x' => {
                    let (yes, no) = split_range(&xmas.x, r.operator, r.value);
                    if let Some(range) = yes {
                        let mut xmas = xmas;
                        xmas.x = range;
                        rules.push((r.target, xmas));
                    }
                    if let Some(range) = no {
                        xmas.x = range;
                    }
                    else {
                        break;
                    }
                },
                'm' => {
                    let (yes, no) = split_range(&xmas.m, r.operator, r.value);
                    if let Some(range) = yes {
                        let mut xmas = xmas;
                        xmas.m = range;
                        rules.push((r.target, xmas));
                    }
                    if let Some(range) = no {
                        xmas.m = range;
                    }
                    else {
                        break;
                    }

                },
                'a' => {
                    let (yes, no) = split_range(&xmas.a, r.operator, r.value);
                    if let Some(range) = yes {
                        let mut xmas = xmas;
                        xmas.a = range;
                        rules.push((r.target, xmas));
                    }
                    if let Some(range) = no {
                        xmas.a = range;
                    }
                    else {
                        break;
                    }

                },
                's' => {
                    let (yes, no) = split_range(&xmas.s, r.operator, r.value);
                    if let Some(range) = yes {
                        let mut xmas = xmas;
                        xmas.s = range;
                        rules.push((r.target, xmas));
                    }
                    if let Some(range) = no {
                        xmas.s = range;
                    }
                    else {
                        break;
                    }

                },
                _ => unimplemented!()
            }
        }
        rules.push((wf.exit, xmas));
    }

    let mut result = 0;
    for r in accepted_range {
        result += r.x.diff() as u64 * r.m.diff() as u64 * r.a.diff() as u64 * r.s.diff() as u64;
    }
    
    result
}

fn parse_rule(input: &str, op_idx: usize) -> Rule {
    let sr_name = &input[0..op_idx];
    let colon_idx = input.find(':').unwrap();
    let value = &input[op_idx+1..colon_idx];
    let target = &input[colon_idx+1..];
    let op = input.chars().nth(op_idx).unwrap();
    Rule{
        property: sr_name.chars().nth(0).unwrap(),
        value : value.parse().unwrap(),
        operator: op,
        target: target
    }
}

fn parse_workflow(input: &str) -> Workflow {
    let idx = input.find('{').unwrap();
    let name = &input[0..idx];
    let subrules = &input[idx+1..input.len()-1];
    let subrules = subrules.split(',');
    let mut subrules_vec = Vec::new();
    let mut exit: &str = input;
    for sr in subrules {
        if let Some(op) = sr.find('>') {
            subrules_vec.push(parse_rule(sr, op));
        }
        else if let Some(op) = sr.find('<') {
            subrules_vec.push(parse_rule(sr, op));
        }
        else {
            exit = sr;
        }
    }

    assert_ne!(exit, input);

    Workflow{
        name: name,
        rules: subrules_vec,
        exit: exit
    }
}

fn parse_part(input: &str) -> HashMap<char, u32> {
    let prop_iter = input[1..input.len()-1].split(',');
    let mut result = HashMap::new();
    for p in prop_iter {
        let (prop, value) = p.split('=').next_tuple().unwrap();
        assert_eq!(prop.len(), 1);
        let prop = prop.chars().nth(0).unwrap();
        let value = value.parse().unwrap();
        result.insert(prop, value);
    }
    result
}

#[cfg(test)]
mod tests{
    use std::collections::HashMap;

    use crate::day19::{parse_rule, Rule, parse_workflow, Workflow, parse_part, solve_day19_part1, solve_day19_part2, MinMax};

    const EXAMPLE : &'static str =
"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";

    #[test]
    fn test_day19_p1() {
        assert_eq!(solve_day19_part1(EXAMPLE), 19114);
    }

    #[test]
    fn test_day19_p2() {
        assert_eq!(solve_day19_part2(EXAMPLE), 167409079868000);
    }

    #[test]
    fn test_parse_rule() {
        assert_eq!(parse_rule("a<2006:qkq", 1), Rule{property: 'a', operator : '<', value: 2006, target: "qkq"});
        assert_eq!(parse_rule("a>3333:R", 1), Rule{property: 'a', operator : '>', value: 3333, target: "R"});
    }

    #[test]
    fn test_parse_workflow() {
        assert_eq!(parse_workflow("qqz{s>2770:qs,m<1801:hdj,R}"), Workflow {
            name: "qqz",
            rules: vec![
                Rule{property: 's', operator : '>', value: 2770, target: "qs"},
                Rule{property: 'm', operator : '<', value: 1801, target: "hdj"}
            ],
            exit: "R"
        });
    }

    #[test]
    fn test_parse_part() {
        let result = parse_part("{x=787,m=2655,a=1222,s=2876}");
        let mut control = HashMap::new();
        control.insert('x', 787);
        control.insert('m', 2655);
        control.insert('a', 1222);
        control.insert('s', 2876);
        assert_eq!(result, control);
    }

    #[test]
    fn test_MinMax_diff() {
        assert_eq!(MinMax::new(1, 4000).diff(), 4000);
    }
}