use advent_of_code_2023::{text_to_string_vec, Position};

pub fn solve_day10_part1(input: &str) -> u64 {
    let mut map = str_to_u8_vec_vec(input);

    let mut start : Position;
    for (i, l) in map.iter().enumerate() {
        for (j, b) in l.iter().enumerate() {
            if b == &b'S' {
                start = Position {
                    x : j as i32,
                    y: i as i32
                };
                break
            }
        }
    }

    

    0
}

fn find_farthest_point(map: &Vec<Vec<u8>>, start: Position, prev: Position) -> Option<u32> {
    let len = 0;
    let mut current = start;
    loop {
        let mut should_be_prev : Position;
        let mut next : Position;
        match map[current.y as usize][current.x as usize] {
            b'S' => {
                return Some((len+1)/2);
            },
            b'|' => {
                should_be_prev = current.north();
                next = current.south();
            },
            b'-' => {
                should_be_prev = current.west()

            },
            b'L' => {

            },
            b'J' => {

            },
            b'7' => {

            },
            b'F' => {

            },
            b'.' => {
                break;
            }
            _ => unimplemented!()
        }
    }
    None
}

pub fn solve_day10_part2(input: &str) -> u64 {
    0
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

#[cfg(test)]
mod tests{
    use crate::day10::solve_day10_part1;

    const EXAMPLE : &'static str =
".....
.F-7.
.|.|.
.L-J.
.....
";
    const EXAMPLE2 : &'static str =
"..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";

    #[test]
    fn test_day10_p1() {
        assert_eq!(solve_day10_part1(EXAMPLE), 4);
        assert_eq!(solve_day10_part1(EXAMPLE2), 8);
    }

    #[test]
    fn test_day10_p2() {
    }
}