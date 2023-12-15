
type Pattern = Vec<String>;

fn solve_day13(input: &str, part2: bool) -> u64 {
    let patterns = scan_patterns(input);
    let mut h_line = 0u64;
    let mut v_line = 0u64;
    for p in patterns {
        if let Some(h) = detect_horizontal_reflect(&p, part2){
            h_line += h;
        }
        else if  let Some(v) = detect_vertical_reflect(&p, part2) {
            v_line += v;
        }
    }
    v_line + 100u64 * h_line 
}

pub fn solve_day13_part1(input: &str) -> u64 {
    solve_day13(input, false)
}

pub fn solve_day13_part2(input: &str) -> u64 {
    solve_day13(input, true)
}

fn scan_patterns(input: &str) -> Vec<Pattern> {
    let mut lines = input.lines();
    let mut patterns = Vec::new();
    let mut pattern = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            if ! pattern.is_empty() {
                patterns.push(pattern);
                pattern = Vec::new();
            }
            continue;
        }
        pattern.push(line.to_string());
    }
    patterns
}

fn detect_horizontal_reflect(pattern : &Pattern, part2: bool) -> Option<u64> {
    let height = pattern.len();
    'outer :for i in 0..height-1 {
        let mut ofo_count = 0;
        if part2 && has_one_difference(pattern[i].as_str(), pattern[i+1].as_str()) {
            ofo_count +=1;
        }
        if pattern[i] == pattern[i+1] || ofo_count == 1 {
            // line between i and i+1, need to backtrack and compare i-1 and i+2, etc...
            let loop_max = usize::min(i+1, height-i-1);
            for j in 1..loop_max {
                let off_by_one_again = part2 && has_one_difference(pattern[i-j].as_str(), pattern[i+1+j].as_str());
                if off_by_one_again {
                    ofo_count +=1;
                }
                if !(pattern[i-j] == pattern[i+1+j] || (off_by_one_again && ofo_count ==1)) {
                    continue 'outer;
                }
            }
            if part2 && ofo_count != 1 {
                continue;
            }
            return Some((i+1) as u64);
        }
    }
    None
}

fn detect_vertical_reflect(pattern : &Pattern, part2: bool) -> Option<u64> {
    let rotated_pattern = rotate_pattern(pattern);
    detect_horizontal_reflect(&rotated_pattern, part2)
}

fn rotate_pattern(pattern: &Pattern) -> Pattern {
    let height = pattern.len();
    let width = pattern[0].len();

    let mut result: Pattern = vec![String::new();width];

    for i in 0..height {
        for j in 0..width {
            result[j].push(pattern[i].chars().nth(j).unwrap());
        }
    }

    result
}

fn has_one_difference(s1: &str, s2: &str) -> bool {
    let mut found_one_difference = false;

    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 != c2 {
            if found_one_difference {
                return false;
            } else {
                found_one_difference = true
            }
        }
    }

    found_one_difference
}

#[cfg(test)]
mod tests{
    use crate::day13::{detect_vertical_reflect, solve_day13_part1, solve_day13_part2};

    use super::{scan_patterns, detect_horizontal_reflect};

    const EXAMPLE : &'static str =
"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#

";

    #[test]
    fn test_day13_p1() {
        assert_eq!(solve_day13_part1(EXAMPLE), 405);
    }

    #[test]
    fn test_day13_p2() {
        assert_eq!(solve_day13_part2(EXAMPLE), 400);
    }

    #[test]
    fn test_scan_patterns() {
        let patterns = scan_patterns(EXAMPLE);
        assert_eq!(patterns.len(), 2);
        assert_eq!(patterns[0].len(), 7);
        assert_eq!(patterns[0][1].len(), 9);
    }

    #[test]
    fn test_detect_horizontal_reflect() {
        let patterns = scan_patterns(EXAMPLE);
        assert_eq!(detect_horizontal_reflect(&patterns[0], false), None);
        assert_eq!(detect_horizontal_reflect(&patterns[1], false), Some(4));
    }

    #[test]
    fn test_detect_horizontal_reflect2() {
        let patterns = scan_patterns(EXAMPLE);
        assert_eq!(detect_horizontal_reflect(&patterns[0], true), Some(3));
        assert_eq!(detect_horizontal_reflect(&patterns[1], true), Some(1));

        const EXAMPLE2 : &'static str =
"#.##.###.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
..##..###

";
        let patterns = scan_patterns(EXAMPLE2);
        assert_eq!(detect_horizontal_reflect(&patterns[0], true), Some(6));
    }

    #[test]
    fn test_detect_vertical_reflect() {
        let patterns = scan_patterns(EXAMPLE);
        assert_eq!(detect_vertical_reflect(&patterns[0], false), Some(5));
    }
}