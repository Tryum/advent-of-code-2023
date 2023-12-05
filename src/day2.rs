use advent_of_code_2023::text_to_string_vec;


#[derive(Debug, PartialEq)]
pub struct Hand {
    pub red: u32,
    pub green: u32,
    pub blue: u32
}

impl Hand {
    fn pow(&self) -> u32 {
        self.red*self.green*self.blue
    }
}

struct Game {
    pub id: u32,
    pub hands : Vec<Hand>
}

fn parse_input_line(input: &str) -> Game {
    let mut game = Game {
        id:0,
        hands: Vec::new()
    };

    let index_and_hands : Vec<&str> = input.split(":").collect();

    game.id = parse_game_index(index_and_hands[0].trim());

    let hands : Vec<&str> = index_and_hands[1].split(";").collect();

    for hand in hands {
        let hand = pasrse_hand(hand);
        game.hands.push(hand);
    }

    game
}

fn parse_game_index(input: &str) -> u32 {
    let digits = &input["Game ".len()..];
    return digits.parse().unwrap();
}

fn pasrse_hand(input: &str) -> Hand {
    let mut hand = Hand { red: 0, green: 0, blue: 0 };
    let cubes = input.split(",");

    for cube in cubes {
        let cube = cube.trim();
        let split_cube : Vec<&str> = cube.split_whitespace().collect();
        let count: u32 = split_cube[0].parse().unwrap();
        match split_cube[1] {
            "red" => hand.red = count,
            "green" => hand.green = count,
            "blue" => hand.blue = count,
            _ => {}
        }
    }

    hand
}

pub fn solve_day2_part1(input: &str, ref_hand: Hand) -> u32 {

    let mut sum = 0;
    let input_lines = text_to_string_vec(input);
    for line in input_lines {
        let game = parse_input_line(line);
        let mut valid_game = true;
        for hand in game.hands {
            if hand.blue > ref_hand.blue
            || hand.green > ref_hand.green
            || hand.red > ref_hand.red {
                valid_game = false;
                break;
            }
        }
        if valid_game {
            sum += game.id;
        }
    }

    sum
}

fn minimum_hand(input: &str) -> Hand {
    let mut result = Hand{ red: 0, green: 0, blue: 0};

    let game = parse_input_line(input);

    for hand in game.hands {
        if hand.red > result.red {
            result.red = hand.red;
        }
        if hand.green > result.green {
            result.green = hand.green;
        }
        if hand.blue > result.blue {
            result.blue = hand.blue;
        }
    }

    result
}

pub fn solve_day2_part2(input: &str) -> u32 {
    let mut sum = 0;
    let input_lines = text_to_string_vec(input);
    for line in input_lines{
        let hand = minimum_hand(&line);
        sum += hand.pow();
    }
    sum
}

#[cfg(test)]
mod tests{
    use advent_of_code_2023::text_to_string_vec;

    use crate::day2::{Hand, pasrse_hand, solve_day2_part1, solve_day2_part2, minimum_hand};

    use super::{parse_input_line, parse_game_index};

    const DAY2_P1_EXAMPLE : &'static str =
"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";


    #[test]
    fn parse_game_line() {
        let input_line_1 = parse_input_line(text_to_string_vec(DAY2_P1_EXAMPLE)[0]);
        assert_eq!(input_line_1.id, 1);
        assert_eq!(input_line_1.hands, [Hand{red:4,green:0,blue:3},Hand{red:1,green:2,blue:6},Hand{red:0,green:2,blue:0}]);

        let input_line_2 = parse_input_line(text_to_string_vec(DAY2_P1_EXAMPLE)[1]);
        assert_eq!(input_line_2.id, 2);
        assert_eq!(input_line_2.hands, [Hand{red:0,green:2,blue:1},Hand{red:1,green:3,blue:4},Hand{red:0,green:1,blue:1}]);

        let input_line_3 = parse_input_line(text_to_string_vec(DAY2_P1_EXAMPLE)[2]);
        assert_eq!(input_line_3.id, 3);
        assert_eq!(input_line_3.hands, [Hand{red:20,green:8,blue:6},Hand{red:4,green:13,blue:5},Hand{red:1,green:5,blue:0}]);
    }

    #[test]
    fn test_parse_game_index() {
        assert_eq!(parse_game_index("Game 5"), 5);
        assert_eq!(parse_game_index("Game 3"), 3);
        assert_eq!(parse_game_index("Game 12"), 12);
    }

    #[test]
    fn test_parse_hand() {
        assert_eq!(pasrse_hand("3 green, 4 blue, 1 red"), Hand{green:3, blue:4, red: 1});
        assert_eq!(pasrse_hand("7 red, 13 blue"), Hand{green:0, blue:13, red: 7});
    }

    #[test]
    fn test_day2_p1() {
        assert_eq!(solve_day2_part1(DAY2_P1_EXAMPLE, Hand{red: 12, green: 13, blue: 14}), 8);
    }

    #[test]
    fn test_minimum_hand(){
        let input_line_1 = text_to_string_vec(DAY2_P1_EXAMPLE)[0];
        assert_eq!(minimum_hand(input_line_1), Hand{red: 4, green: 2, blue:6});

        let input_line_2 = text_to_string_vec(DAY2_P1_EXAMPLE)[1];
        assert_eq!(minimum_hand(input_line_2), Hand{red: 1, green: 3, blue:4});
    }

    #[test]
    fn test_day2_p2() {
        assert_eq!(solve_day2_part2(DAY2_P1_EXAMPLE), 2286);
    }

}