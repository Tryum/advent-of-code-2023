use std::ops;


pub fn text_to_string_vec(input: &str) -> Vec<&str>{
    return input.lines().collect();
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West
}

impl ops::Add<i32> for Direction {
    type Output = Direction;
    fn add(self, rhs: i32) -> Direction {
        let mut dir_as_int: i32 = match self {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
        };

        dir_as_int += 4 + rhs;
        dir_as_int %= 4;

        match dir_as_int {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            _ => panic!("Never happens!")
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position {
            x: x,
            y: y
        }
    }
}

#[cfg(test)]
mod tests{
    use crate::text_to_string_vec;

    #[test]
    fn testtext_to_string_vec() {
        let input ="abc\ndef\nghi";

        let result = text_to_string_vec(input);

        assert_eq!(result, vec!["abc", "def", "ghi"]);

    }

}