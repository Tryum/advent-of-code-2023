use std::{mem, io::{self, Write}};

use advent_of_code_2023::text_to_string_vec;
use colored::{self, Colorize};

pub fn solve_day14_part1(input: &str) -> usize {
    let mut rocks = str_to_u8_vec_vec(input);
    tilt_rocks(&mut rocks);
    let final_rocks = u8_vec_vec_to_string_vec(&rocks);
    let mut score = 0;
    let mut line_score = final_rocks.len();
    for l in final_rocks {
        for c in l.chars() {
            if c == 'O' {
                score += line_score;
            }
        }
        line_score -= 1;
    }
    score
}

pub fn solve_day14_part2(input: &str) -> usize {
    let mut rocks = str_to_u8_vec_vec(input);
    let mut cycle = 0;
    let mut scores = Vec::new();
    loop{
        for i in 0..4 {
            tilt_rocks(&mut rocks);
            rocks = rotate_rocks(&rocks);            
        }
        let final_rocks = u8_vec_vec_to_string_vec(&rocks);
        let mut score = 0;
        let mut line_score = final_rocks.len();
        for l in final_rocks {
            for c in l.chars() {
                if c == 'O' {
                    score += line_score;
                }
            }
            line_score -= 1;
        }
        scores.push(score);
        cycle+=1;
        println!("{cycle} : {score}");

        if cycle == 1000000 {
            break score;
        }
        // let mut buffer = String::new();
        // let _ = io::stdin().read_line(&mut buffer);
        
    }
}

fn dbg_rocks(rocks:& Vec<Vec<u8>>, i: usize, j: usize, k: Option<usize>) {
    //print!("{}[2J", 27 as char);
    for i2 in 0..rocks.len() {
        for j2 in 0..rocks[i2].len() {
            let c = if i2 == i && j2 == j {
                (rocks[i2][j2] as char).to_string().red()
            }
            else if Some(i2) == k && j2 == j {
                (rocks[i2][j2] as char).to_string().blue()
            }
            else {
                (rocks[i2][j2] as char).to_string().white()
            };
            print!("{}", c);
        }
        println!("");
    }
    println!("");
    let mut buffer = String::new();
    //let _ = io::stdin().read_line(&mut buffer);
}

fn str_to_u8_vec_vec(input: &str) -> Vec<Vec<u8>>{
    let mut u8_vec_vec : Vec<Vec<u8>> = Vec::new();

    for l in input.lines() {
        if l.is_empty() {
            continue;
        }
        u8_vec_vec.push(l.to_string().as_bytes().to_vec());
    }

    u8_vec_vec
}

fn tilt_rocks(input: &mut Vec<Vec<u8>>) {
    for i in 0..input.len() {
        if input[i].is_empty() {
            continue;
        }
        for j in 0..input[i].len() {
            //dbg_rocks(&temp_map, i, j, None);
            match input[i][j] {
                b'O' => (),
                b'.' => {
                    for k in i+1..input.len() {
                        //dbg_rocks(&temp_map, i, j, Some(k));
                        match input[k][j] {
                            b'O' => {
                                
                                input[i][j] = b'O';
                                input[k][j] = b'.';
                                break;
                            },
                            b'.' => (),
                            b'#' => break,
                            _ => ()
                        };
                    };
                },
                b'#' => (),
                _ => ()
            };
        };
    };
}

fn u8_vec_vec_to_string_vec(input: &Vec<Vec<u8>>) -> Vec<String> {
    let mut result = Vec::new();
    for l in input.iter() {
        result.push(String::from_utf8(l.to_vec()).unwrap());
    }
    result
}

fn rotate_rocks(input: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let height = input.len();
    let width = input[0].len();

    let mut result: Vec<Vec<u8>> = vec![vec![0u8;height];width];

    for i in 0..height {
        for j in 0..width {
            result[j][height-1-i] = input[i][j];
        }
    }
    result
}

#[cfg(test)]
mod tests{
    use crate::day14::{str_to_u8_vec_vec, u8_vec_vec_to_string_vec, solve_day14_part2};

    use super::{tilt_rocks, solve_day14_part1, rotate_rocks};

    const EXAMPLE : &'static str =
"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";

    const EXAMPLE_RESULT : &'static str = 
"OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....
";

    #[test]
    fn test_day14_p1() {
        assert_eq!(solve_day14_part1(EXAMPLE), 136);
    }

    #[ignore]
    #[test]
    fn test_day14_p2() {
        assert_eq!(solve_day14_part2(EXAMPLE), 64);
    }

    #[test]
    fn test_rotate_rocks(){
const INPUT_ROT : &'static str =
"URR
U.D
LLD
";

const CONTROL_ROT : &'static str =
"LUU
L.R
DDR
";
        let mut input = str_to_u8_vec_vec(INPUT_ROT);
        let control = str_to_u8_vec_vec(CONTROL_ROT);

        input = rotate_rocks(&mut input);

        assert_eq!(input, control);
    }

    #[test]
    fn test_tilt_rocks(){
        println!("{}", EXAMPLE);
        
        let mut rocks = str_to_u8_vec_vec(EXAMPLE);
        tilt_rocks(&mut rocks);

        let result = u8_vec_vec_to_string_vec(&rocks);

        dbg!(&result);

        let mut control = Vec::new();
        for l in EXAMPLE_RESULT.lines() {
            control.push(l.to_string());
        }

        assert_eq!(result, control);
    }
}