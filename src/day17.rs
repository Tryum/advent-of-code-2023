use std::{collections::{VecDeque, HashMap}, arch::x86_64::_CMP_NEQ_OS, io};

use advent_of_code_2023::{Position, Direction};
use colored::Colorize;
use priority_queue::{PriorityQueue, DoublePriorityQueue};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Move {
    pos: Position,
    dir: Direction,
    straight: u8
}

pub fn solve_day17_part1(input: &str) -> u64 {
    let map = read_map(input);
    solve_day17(&map, 0, 3).into()
}

pub fn solve_day17_part2(input: &str) -> u64 {
    let map = read_map(input);
    solve_day17(&map, 4, 10).into()
}

fn read_map(input: &str) -> Vec<Vec<u8>> {
    let mut map = Vec::new();

    for l in input.lines() {
        if l.is_empty() {
            continue;
        }
        map.push(l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect());
    }
    map
}

#[cfg(not(debug_assertions))]
fn debug_day17(map: &Vec<Vec<u8>>, pos: &Position) {
}

#[cfg(debug_assertions)]
fn debug_day17(map: &Vec<Vec<u8>>, pos: &Position) {
    for (i, l) in map.iter().enumerate() {
        for (j, c) in l.iter().enumerate() {
            if i == pos.y as usize && j == pos.x as usize{
                print!("{}", c.to_string().on_red());
            }
            else {
                print!("{}", c);
            }
            
        }
        println!()
    }
    println!();
}

fn solve_day17(map: &Vec<Vec<u8>>, min_straight: u8, max_straight: u8) -> u32 {

    let width = map[0].len();
    let height = map.len();

    let mut queue = DoublePriorityQueue::new();
    queue.push( Move {
        pos: Position::new(0, 0),
        dir: Direction::East,
        straight: 0
    }, 0);
    queue.push( Move {
        pos: Position::new(0, 0),
        dir: Direction::South,
        straight: 0
    }, 0);

    let mut result = u32::MAX;

    let mut heat_so_far = HashMap::new();
    let mut move_count = 0;

    while let Some((m, heat))  = queue.pop_min() {

        // println!("{:?} : {}", m, heat);
        // debug_day17(&map, &m.pos);
        move_count += 1;

        if m.pos.x == (width-1) as i32 && m.pos.y == (height-1) as i32 && m.straight >= min_straight {
            println!("{:?} : {}", m, heat);
            println!("total moves : {move_count}");
            return heat;
        }

        let mut new_moves = get_moves(width, height, &m, min_straight, max_straight);

        for n in &mut new_moves {
            
            let new_heat = heat + map[n.pos.y as usize][n.pos.x as usize] as u32;
            // println!("{:?}<{}> : {} -> {:?}<{}> : {}", m.pos, m.straight, heat, n.pos, n.straight, new_heat);
            let heat = heat_so_far.entry((n.pos, n.dir, n.straight)).or_insert(u32::MAX);
            if new_heat < *heat {
                *heat = new_heat;
                queue.push(*n, new_heat);
            }
        }
        // let mut buffer = String::new();
        // let _ = io::stdin().read_line(&mut buffer);
    }
    

    result
}

fn get_moves(width: usize, height: usize, m: &Move, min_straight: u8, max_straight: u8) -> Vec<Move> {
    let mut moves = Vec::new();
    for i in [-1, 0, 1] {
        let mut new_move = *m;
        new_move.dir = new_move.dir + i;

        if i != 0 && m.straight < min_straight {
            continue;
        }

        new_move.straight = if i != 0 {
            1
        }
        else{
            new_move.straight + 1
        };

        match new_move.dir {
            Direction::North => new_move.pos.y-=1,
            Direction::East => new_move.pos.x+=1,
            Direction::South => new_move.pos.y+=1,
            Direction::West => new_move.pos.x-=1,
        }

        if new_move.straight <= max_straight
        && new_move.pos.x >= 0
        && new_move.pos.x < width as i32
        && new_move.pos.y >= 0
        && new_move.pos.y < height as i32 {
            moves.push(new_move);
        }
    }

    moves
}

#[cfg(test)]
mod tests{
    use crate::day17::solve_day17_part2;

    use super::solve_day17_part1;

    const EXAMPLE : &'static str =
"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";

    const EXAMPLE2 : &'static str =
"111111111111
999999999991
999999999991
999999999991
999999999991
";

    #[test]
    fn test_day17_p1() {
        assert_eq!(solve_day17_part1(EXAMPLE), 102);
    }

    #[test]
    fn test_day17_p2() {
        //assert_eq!(solve_day17_part2(EXAMPLE), 94);
        assert_eq!(solve_day17_part2(EXAMPLE2), 71);
    }
}