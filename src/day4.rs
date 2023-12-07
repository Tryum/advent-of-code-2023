use std::collections::HashMap;


pub fn solve_day4_part1(input: &str) -> u32 {
    let mut result = 0;
    for l in input.lines(){
        let numbers = get_card_numbers(l);
        let winning_numbers = get_my_winning_numbers(numbers.0, numbers.1);
        if winning_numbers.len() > 0 {
            result += u32::pow(2, winning_numbers.len() as u32 -1);
        }
    }
    result
}

pub fn solve_day4_part2(input: &str) -> u32 {
    let mut scratch_card_count = 0;
    let mut card_count = HashMap::new();
    for (i, l) in input.lines().enumerate(){
        let numbers = get_card_numbers(l);
        let winning_numbers = get_my_winning_numbers(numbers.0, numbers.1);
        let total_count = 1 + card_count.get(&i).unwrap_or(&0);

        let mut next_cards = String::new();
        scratch_card_count += total_count;
        for (j, _) in winning_numbers.iter().enumerate() {
            let k = i+j+1;
            next_cards += format!("{}, ", k+1).as_str();
            *card_count.entry(k).or_insert(0) += total_count;
        }
        //println!("Card {} (+{}) has {} matching numbers, so you'll win {} copy each of the next {} cards : {}", i+1, total_count-1, winning_numbers.len(), total_count, winning_numbers.len(), next_cards);
    }
    scratch_card_count
}

fn get_numbers(input: &str) -> Vec<u32> {
    let mut result = Vec::new();
    for n in input.trim().split_ascii_whitespace() {
        result.push(n.parse().unwrap());
    }    
    result
}

fn get_winning_numbers(input: &str) -> Vec<u32> {
    let semi_colon = input.find(":").unwrap();
    let numbers = &input[semi_colon+1..];

    get_numbers(numbers)
}

fn get_card_numbers(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut iter = input.splitn(2, '|');
    let winning_numbers = iter.next().unwrap();
    let my_nubers = iter.next().unwrap();

    (get_winning_numbers(winning_numbers), get_numbers(my_nubers))
}

fn get_my_winning_numbers(winning_numbers : Vec<u32>, my_numbers: Vec<u32>) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();
    for n in my_numbers.iter(){
        if winning_numbers.contains(n){
            result.push(*n);
        }
    }
    result
}

#[cfg(test)]
mod tests{
    use crate::day4::{get_winning_numbers, solve_day4_part1, get_card_numbers, get_my_winning_numbers, solve_day4_part2};

    const EXAMPLE : &'static str =
"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    #[test]
    fn test_day4_p1() {
        assert_eq!(solve_day4_part1(EXAMPLE), 13);
    }

    #[test]
    fn test_day4_p2() {
        assert_eq!(solve_day4_part2(EXAMPLE), 30);
    }

    #[test]
    fn test_get_winning_numbers(){
        assert_eq!(get_winning_numbers("Card 1: 41 48 83 86 17 "), vec![41, 48, 83, 86, 17]);
    }

    #[test]
    fn test_get_card_numbers(){
        assert_eq!(get_card_numbers("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),(vec![41, 48, 83, 86, 17], vec![83,86,6,31,17,9,48,53]));
    }

    #[test]
    fn test_get_my_winning_numbers(){
        let mut truth: Vec<u32> = vec![48, 83, 17, 86];
        truth.sort();
        let mut result = get_my_winning_numbers(vec![41, 48, 83, 86, 17], vec![83,86,6,31,17,9,48,53]).clone();
        result.sort();

        assert_eq!(result, truth);
    }

}