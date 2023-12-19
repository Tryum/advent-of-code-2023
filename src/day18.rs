use std::{collections::{HashSet, HashMap}, cell::Cell};

use advent_of_code_2023::{Direction, Position};
use image::{GrayImage, ImageBuffer, GenericImage, Luma, RgbImage, Rgb};
use itertools::Itertools;

enum CellType {
    InWall,
    Inside,
    Outside
}

pub fn solve_day18_part1(input: &str) -> u64 {
    let mut contour =  HashMap::new();
    let mut cur_pos = Position::new(0, 0);
    contour.insert(cur_pos, CellType::InWall);
    let mut map_min = Position::new(i32::MAX, i32::MAX);
    let mut map_max = Position::new(i32::MIN, i32::MIN);
    let mut poly = Vec::new();
    poly.push(cur_pos);
    let mut perimeter = 0;
    for l in input.lines() {
        if l.is_empty() {
            continue;
        }
        let (dir, length, _) = parse_line(l);
        match dir {
            Direction::North => cur_pos.y-=length as i32,
            Direction::East => cur_pos.x+=length as i32 ,
            Direction::South => cur_pos.y+=length as i32,
            Direction::West => cur_pos.x-=length as i32,
        }
        map_min.x = map_min.x.min(cur_pos.x);
        map_min.y = map_min.y.min(cur_pos.y);
        map_max.x = map_max.x.max(cur_pos.x);
        map_max.y = map_max.y.max(cur_pos.y);
        perimeter+= length;
        poly.push(cur_pos);
    }
    //println!("{:?}", poly);
    assert_eq!(poly[0], poly[poly.len()-1]);
    let _ = poly.pop();
    //works because origin point is set two times.
    let mut area = shoe_lace(&poly);
    // println!("Poly : {:?}, area : {area}", poly);
    // println!("Perimeter: {perimeter}");
    // println!("{}", area + perimeter as f64 *0.5f64 -1f64);
    let width = (map_max.x+1-map_min.x) as usize;
    let height = (map_max.y+1-map_min.y) as usize;
    let surface = 0;
    area as u64 + perimeter as u64 /2 +1
}

pub fn solve_day18_part2(input: &str) -> u64 {
    let mut contour =  HashMap::new();
    let mut cur_pos = Position::new(0, 0);
    contour.insert(cur_pos, CellType::InWall);
    let mut map_min = Position::new(i32::MAX, i32::MAX);
    let mut map_max = Position::new(i32::MIN, i32::MIN);
    let mut poly = Vec::new();
    poly.push(cur_pos);
    let mut perimeter = 0;
    for l in input.lines() {
        if l.is_empty() {
            continue;
        }
        let (dir, length) = parse_line2(l);
        match dir {
            Direction::North => cur_pos.y-=length as i32,
            Direction::East => cur_pos.x+=length as i32 ,
            Direction::South => cur_pos.y+=length as i32,
            Direction::West => cur_pos.x-=length as i32,
        }
        map_min.x = map_min.x.min(cur_pos.x);
        map_min.y = map_min.y.min(cur_pos.y);
        map_max.x = map_max.x.max(cur_pos.x);
        map_max.y = map_max.y.max(cur_pos.y);
        perimeter+= length;
        poly.push(cur_pos);
    }
    //println!("{:?}", poly);
    assert_eq!(poly[0], poly[poly.len()-1]);
    let _ = poly.pop();
    //works because origin point is set two times.
    let mut area = shoe_lace(&poly);
    // println!("Poly : {:?}, area : {area}", poly);
    // println!("Perimeter: {perimeter}");
    // println!("{}", area + perimeter as f64 *0.5f64 -1f64);
    let width = (map_max.x+1-map_min.x) as usize;
    let height = (map_max.y+1-map_min.y) as usize;
    let surface = 0;
    area as u64 + perimeter as u64 /2 +1
}

fn parse_line(input: &str)-> (Direction, usize, &str) {
    let (dir, length, color) = input.split_ascii_whitespace().next_tuple().unwrap();
    let dir = match dir {
        "U" => Direction::North,
        "R" => Direction::East,
        "D" => Direction::South,
        "L" => Direction::West,
        _ => unimplemented!()
    };
    let length = length.parse().unwrap();
    (dir, length, color)
}

fn parse_line2(input: &str)-> (Direction, usize) {
    let (_, _, color) = input.split_ascii_whitespace().next_tuple().unwrap();
    let color = &color[1..color.len()-1];
    assert_eq!(color.len(), 7);
    assert!(color.starts_with("#"));
    let color = &color[1..color.len()];
    let length = &color[0..5];
    let dir = &color[5..];

    let dir = match dir {
        "3" => Direction::North,
        "0" => Direction::East,
        "1" => Direction::South,
        "2" => Direction::West,
        _ => unimplemented!()
    };
    let length = usize::from_str_radix(length, 16).unwrap();
    (dir, length)
}

fn shoe_lace(vertices: &Vec<Position>) -> f64 {
    let mut area: i64 = 0;
    for i in 0..vertices.len() {
        let j = (i+1)%vertices.len();
        area += vertices[i].x as i64 * vertices[j].y as i64 ;
        area -= vertices[j].x as i64 * vertices[i].y as i64 ;
    }
    area.abs() as f64 * 0.5
}

#[cfg(test)]
mod tests{
    use advent_of_code_2023::{Direction, Position};

    use crate::day18::{solve_day18_part1, parse_line, parse_line2, solve_day18_part2, shoe_lace};

    const EXAMPLE : &'static str =
"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";

    #[test]
    fn test_day18_p1() {
        assert_eq!(solve_day18_part1(EXAMPLE), 62);
    }

    #[test]
    fn test_day18_p2() {
        assert_eq!(solve_day18_part2(EXAMPLE), 952408144115);
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("U 2 (#caa173)"), (Direction::North, 2, "(#caa173)"));
    }

    #[test]
    fn test_parse_line2() {
        assert_eq!(parse_line2("U 2 (#caa173)"), (Direction::North, 829975));
    }

    #[test]
    fn test_shoe_lace() {
        assert_eq!(shoe_lace(&vec![Position::new(1, 6),Position::new(3, 1), Position::new(7, 2), Position::new(4, 4), Position::new(8, 5)]), 16.5f64);
        assert_eq!(shoe_lace(&vec![Position::new(0, 0),Position::new(10, 0), Position::new(10, 10), Position::new(0, 10)]), 100f64);
    }
}