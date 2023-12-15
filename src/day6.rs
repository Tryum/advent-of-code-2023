

use std::str::Lines;

use itertools::Itertools;

pub fn solve_day6_part1(line: &mut Lines) -> u64 {
    let time = parse_time(line);
    let dist = parse_distance(line);
    assert_eq!(time.len(), dist.len());

    let mut sum = 1;
    for i in 0..time.len() {
        sum *= solve_part1(time[i], dist[i]);
    }
    sum
}

pub fn solve_day6_part2(line: &mut Lines) -> u64 {
    let time = parse_time(line);
    let dist = parse_distance(line);
    assert_eq!(time.len(), dist.len());

    let mut sum = 1;
    for i in 0..time.len() {
        sum *= solve_part1(time[i], dist[i]);
    }
    sum
}

fn parse_time(line: &mut Lines) -> Vec<u64> {
    let line = line.next().unwrap();
    assert!(line.starts_with("Time:"));
    line["Time:".len()+1..].split_ascii_whitespace().map(|x| x.parse::<u64>().unwrap()).collect_vec()
}

fn parse_distance(line: &mut Lines) -> Vec<u64> {
    let line = line.next().unwrap();
    assert!(line.starts_with("Distance:"));
    line["Distance:".len()+1..].split_ascii_whitespace().map(|x| x.parse::<u64>().unwrap()).collect_vec()
}

fn solve_part1(time: u64, dist: u64) -> u64 {
    let a = 1;
    let b = -(time as i64);
    let c = dist as i64;

    let delta = b*b - 4*a*c;

    let x1 = ((-b as f64 - (delta as f64).sqrt())/(2f64 *a as f64)).floor();
    let x2 = ((-b as f64 + (delta as f64).sqrt())/(2f64 *a as f64)).ceil();

    let result = x2 -x1 - 1f64;
    result as u64
}


#[cfg(test)]
mod tests{
    use crate::day6::{solve_day6_part1, parse_time, parse_distance, solve_part1};

    const EXAMPLE : &'static str =
"Time:      7  15   30
Distance:  9  40  200
";

    #[test]
    fn test_day6_p1() {
        assert_eq!(solve_day6_part1(&mut EXAMPLE.lines()), 288);
    }

    #[test]
    fn test_day6_p2() {
    }

    #[test]
    fn test_parse_time() {
        assert_eq!(parse_time(&mut "Time:      7  15   30".lines()), vec![7, 15, 30]);
    }

    #[test]
    fn test_parse_distance() {
        assert_eq!(parse_distance(&mut "Distance:  9  40  200".lines()), vec![9, 40, 200]);
    }

    #[test]
    fn test_solve_part1() {
        for (t, d, r) in [(7, 9, 4), (15, 40, 8),(30, 200, 9)] {
            assert_eq!(solve_part1(t, d), r);
        }

        assert_eq!(solve_part1(71530, 940200), 71503);
    }

}
