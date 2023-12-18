use std::{collections::HashSet, cmp::max};

use advent_of_code_2023::{Direction, Position};

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Beam {
    pos: Position,
    dir: Direction
}

pub fn solve_day16_part1(input: &str) -> usize {
    let map = read_map(input);
    day16_solver(&map, &Beam{
        pos: Position {
            x:0,
            y:0
        },
        dir: Direction::East
    })
}

pub fn solve_day16_part2(input: &str) -> usize {
    let map = read_map(input);
    let width = map[0].len();
    let height = map.len();

    let mut best_score = 0;

    for i in 0..width {
        let beam = Beam { pos: Position::new(i as i32, 0), dir: Direction::South};
        let energized_tiles = day16_solver(&map, &beam);
        //println!("{:?} : {}", beam, energized_tiles);
        best_score = max(energized_tiles, best_score);
        let beam = Beam { pos: Position::new(i as i32, (height-1) as i32), dir: Direction::North};
        let energized_tiles = day16_solver(&map, &beam);
        //println!("{:?} : {}", beam, energized_tiles);
        best_score = max(energized_tiles, best_score);

    }
    for i in 0..height {
        let beam = Beam { pos: Position::new(0, i as i32), dir: Direction::West};
        let energized_tiles = day16_solver(&map, &beam);
        //println!("{:?} : {}", beam, energized_tiles);
        best_score = max(energized_tiles, best_score);
        let beam = Beam { pos: Position::new((width-1) as i32, i as i32), dir: Direction::East};
        let energized_tiles = day16_solver(&map, &beam);
        //println!("{:?} : {}", beam, energized_tiles);
        best_score = max(energized_tiles, best_score);
    }
    best_score
}

fn read_map(input: &str) -> Vec<&[u8]> {
    let mut map = Vec::new();

    for l in input.lines() {
        if l.is_empty() {
            continue;
        }
        map.push(l.as_bytes());   
    }
    map
}

fn day16_solver(map: &Vec<&[u8]>, initial_beam : &Beam) ->usize {
    let mut energized_tiles = HashSet::new();
    
    let mut beams = solve_beam(&map[initial_beam.pos.x as usize][initial_beam.pos.y as usize], &initial_beam);

    let width = map[0].len() as i32;
    let height = map.len() as i32;

    loop {
        let mut beam = match beams.pop() {
            Some(beam) => beam,
            None => break,
        };

        if ! energized_tiles.insert(beam) {
            continue;
        }
        match beam.dir {
            Direction::North => beam.pos.y -= 1,
            Direction::East => beam.pos.x += 1,
            Direction::South => beam.pos.y += 1,
            Direction::West => beam.pos.x -= 1,
        }

        if beam.pos.x < 0 || beam.pos.x >= width
        || beam.pos.y < 0 || beam.pos.y >= height {
            continue;
        }
        let mut result_beams = solve_beam(&map[beam.pos.y as usize][beam.pos.x as usize], &beam);
        beams.append(&mut result_beams);
    }

     energized_tiles.iter().map(|b| b.pos).collect::<HashSet<_>>().len()
}

fn solve_beam(cell: &u8, beam: &Beam) -> Vec<Beam> {
    let mut beams = Vec::new();
    let mut beam = beam.clone();
    match cell {
        b'.' => {},
        b'|' => {
            match beam.dir {
                Direction::East|Direction::West => {
                    beams.push(Beam {
                        pos : beam.pos.clone(),
                        dir : Direction::North
                    });
                    beams.push(Beam {
                        pos : beam.pos.clone(),
                        dir : Direction::South
                    });
                    return beams;
                },
                _ => {}
            }
        },
        b'-' => {
            match beam.dir {
                Direction::North|Direction::South => {
                    beams.push(Beam {
                        pos : beam.pos.clone(),
                        dir : Direction::East
                    });
                    beams.push(Beam {
                        pos : beam.pos.clone(),
                        dir : Direction::West
                    });
                    return beams;
                },
                _ => {}
            }
        },
        b'\\' => {
            beam.dir = match beam.dir {
                Direction::North => Direction::West,
                Direction::East => Direction::South,
                Direction::South => Direction::East,
                Direction::West => Direction::North,
            }
        },
        b'/' => {
            beam.dir = match beam.dir {
                Direction::North => Direction::East,
                Direction::East => Direction::North,
                Direction::South => Direction::West,
                Direction::West => Direction::South,
            }
        },
        _ => {}
    }
    vec![beam]
}


#[cfg(test)]
mod tests{
    use crate::day16::{solve_beam, Beam, Position, Direction, solve_day16_part2};

    use super::solve_day16_part1;

    const EXAMPLE : &'static str =
r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"#;

    #[test]
    fn test_day16_p1() {
        assert_eq!(solve_day16_part1(EXAMPLE), 46);
    }

    #[test]
    fn test_day16_p2() {
        assert_eq!(solve_day16_part2(EXAMPLE), 51);
    }

    #[test]
    fn test_solve_beam() {
        let beam_east = Beam { pos: Position::new(0, 0), dir: Direction::East };
        let beam_west = Beam { pos: Position::new(0, 0), dir: Direction::West };
        let beam_north = Beam { pos: Position::new(0, 0), dir: Direction::North };
        let beam_south = Beam { pos: Position::new(0, 0), dir: Direction::South };

        assert_eq!(solve_beam(&b'.', &beam_east), vec![beam_east]);
        assert_eq!(solve_beam(&b'.', &beam_west), vec![beam_west]);
        assert_eq!(solve_beam(&b'.', &beam_north), vec![beam_north]);
        assert_eq!(solve_beam(&b'.', &beam_south), vec![beam_south]);

        assert_eq!(solve_beam(&b'|', &beam_east), vec![beam_north, beam_south]);
        assert_eq!(solve_beam(&b'|', &beam_west), vec![beam_north, beam_south]);
        assert_eq!(solve_beam(&b'|', &beam_north), vec![beam_north]);
        assert_eq!(solve_beam(&b'|', &beam_south), vec![beam_south]);

        assert_eq!(solve_beam(&b'\\', &beam_east), vec![beam_south]);
        assert_eq!(solve_beam(&b'\\', &beam_west), vec![beam_north]);
        assert_eq!(solve_beam(&b'\\', &beam_north), vec![beam_west]);
        assert_eq!(solve_beam(&b'\\', &beam_south), vec![beam_east]);

        assert_eq!(solve_beam(&b'/', &beam_east), vec![beam_north]);
        assert_eq!(solve_beam(&b'/', &beam_west), vec![beam_south]);
        assert_eq!(solve_beam(&b'/', &beam_north), vec![beam_east]);
        assert_eq!(solve_beam(&b'/', &beam_south), vec![beam_west]);
    }
}