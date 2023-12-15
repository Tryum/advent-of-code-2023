use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Hand {
    hand : String,
    bid: u64
}

fn card_score(c : char) -> u32 {
    match c {
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'J' => 10,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => 0
    }
}

fn card_score2(c : char) -> u32 {
    match c {
        'J' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => 0
    }
}

pub fn solve_day7_part1(input: &str) -> u64 {
    let mut hands = Vec::<Hand>::new();

    for l in input.lines() {
        if l.is_empty() {continue;}
        hands.push(parse_hand_and_bid(l));
    }

    hands.sort_by(|a, b| compare_hands(&a.hand, &b.hand));

    let mut sum = 0;
    for (i, h) in hands.iter().enumerate() {
        sum += h.bid * (i as u64 +1);
    }

    sum
}

pub fn solve_day7_part2(input: &str) -> u64 {
    let mut hands = Vec::<Hand>::new();

    for l in input.lines() {
        if l.is_empty() {continue;}
        hands.push(parse_hand_and_bid(l));
    }

    hands.sort_by(|a, b| compare_hands2(&a.hand, &b.hand));

    let mut sum = 0;
    for (i, h) in hands.iter().enumerate() {
        sum += h.bid * (i as u64 +1);
    }

    sum
}

fn parse_hand(hand: &str) -> HandType {
    const CARDS : &'static str = "AKQJT98765432";

    let mut pairs = 0;
    let mut three_of_a_kind = 0;

    for c in  CARDS.as_bytes() {
        match hand.as_bytes().iter().filter(|&n| n == c).count() {
            5 => return HandType::FiveOfAKind,
            4 => return HandType::FourOfAKind,
            3 => three_of_a_kind+=1,
            2 => pairs+=1,
            _ => ()
        }
    }
    if three_of_a_kind == 1 && pairs == 1 { return HandType::FullHouse; }
    if three_of_a_kind == 1 { return HandType::ThreeOfAKind; }
    if pairs == 2 { return HandType::TwoPair; }
    if pairs == 1 { return HandType::OnePair; }
    return HandType::HighCard;
}

fn parse_hand2(hand: &str) -> HandType {

    let mut pairs = 0;
    let mut three_of_a_kind = 0;
    let mut four_of_a_kind = 0;

    let mut card_count = HashMap::new();

    for c in  hand.as_bytes() {
        *card_count.entry(*c).or_insert(0) += 1;
    }

    let jokers = card_count.remove(&b'J').unwrap_or(0);
    for (_card, count) in card_count {
        match count {
            5 => return HandType::FiveOfAKind,
            4 => four_of_a_kind +=1,
            3 => three_of_a_kind+=1,
            2 => pairs+=1,
            _ => ()
        }
    }

    if four_of_a_kind == 1 {
        if jokers == 1 { return HandType::FiveOfAKind } else { return HandType::FourOfAKind; }
    }
    if three_of_a_kind == 1 && pairs == 1 { return HandType::FullHouse; }
    if three_of_a_kind == 1 {
        if jokers == 2 { return HandType::FiveOfAKind; }
        else if jokers == 1 { return HandType::FourOfAKind; }
        else { return HandType::ThreeOfAKind; }
    }
    if pairs == 2 { 
        if jokers == 1 { return HandType::FullHouse; }
        else { return HandType::TwoPair; }
    }
    if pairs == 1 {
        match jokers {
            3 => return HandType::FiveOfAKind,
            2 => return HandType::FourOfAKind,
            1 => return HandType::ThreeOfAKind,
            _ => return HandType::OnePair
        }
    }
    match jokers {
        5 => return HandType::FiveOfAKind,
        4 => return HandType::FiveOfAKind,
        3 => return HandType::FourOfAKind,
        2 => return HandType::ThreeOfAKind,
        1 => return HandType::OnePair,
        _ => ()
    }
    return HandType::HighCard;
}

fn compare_hands(hand1 : &str, hand2: &str) -> Ordering {
    let hand1_type = parse_hand(hand1);
    let hand2_type = parse_hand(hand2);

    if hand1_type == hand2_type {
        for i in 0..5 {
            let score1 = card_score(hand1.chars().nth(i).unwrap());
            let score2 = card_score(hand2.chars().nth(i).unwrap());
            if score1 == score2 {
                continue;
            }
            else {
                return if score1 > score2 { Ordering::Greater} else { Ordering::Less};
            }
        }
        return Ordering::Equal;
    }
    else {
        return if hand1_type > hand2_type { Ordering::Greater} else { Ordering::Less};
    }
}

fn compare_hands2(hand1 : &str, hand2: &str) -> Ordering {
    let hand1_type = parse_hand2(hand1);
    let hand2_type = parse_hand2(hand2);

    if hand1_type == hand2_type {
        for i in 0..5 {
            let score1 = card_score2(hand1.chars().nth(i).unwrap());
            let score2 = card_score2(hand2.chars().nth(i).unwrap());
            if score1 == score2 {
                continue;
            }
            else {
                return if score1 > score2 { Ordering::Greater} else { Ordering::Less};
            }
        }
        return Ordering::Equal;
    }
    else {
        return if hand1_type > hand2_type { Ordering::Greater} else { Ordering::Less};
    }
}

fn parse_hand_and_bid(input: &str) -> Hand {
    let (hand, bid) = input.split_at(input.find(' ').unwrap());

    Hand{
        hand: hand.to_string(),
        bid : bid.trim().parse().unwrap()
    }
}

#[cfg(test)]
mod tests{
    use std::cmp::Ordering;

    use crate::day7::{parse_hand, HandType, compare_hands, solve_day7_part2, parse_hand2, compare_hands2};

    use super::solve_day7_part1;

    const EXAMPLE : &'static str =
"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    #[test]
    fn test_day7_p1() {
        assert_eq!(solve_day7_part1(EXAMPLE), 6440);
    }

    #[test]
    fn test_day7_p2() {
        assert_eq!(solve_day7_part2(EXAMPLE), 5905);
    }

    #[test]
    fn test_parse_hand() {
        assert_eq!(parse_hand("AAAAA"), HandType::FiveOfAKind);
        assert_eq!(parse_hand("AA8AA"), HandType::FourOfAKind);
        assert_eq!(parse_hand("23332"), HandType::FullHouse);
        assert_eq!(parse_hand("TTT98"), HandType::ThreeOfAKind);
        assert_eq!(parse_hand("23432"), HandType::TwoPair);
        assert_eq!(parse_hand("A23A4"), HandType::OnePair);
        assert_eq!(parse_hand("23456"), HandType::HighCard);
    }

    #[test]
    fn test_compare_hands() {
        assert_eq!(compare_hands("AAAAA", "AA8AA"), Ordering::Greater);
        assert_eq!(compare_hands("KK677", "KTJJT"), Ordering::Greater);
        assert_eq!(compare_hands("QQQJA", "T55J5"), Ordering::Greater);
    }

    #[test]
    fn test_parse_hand2() {
        assert_eq!(parse_hand2("QQQQJ"), HandType::FiveOfAKind);
        assert_eq!(parse_hand2("QQQJJ"), HandType::FiveOfAKind);
        assert_eq!(parse_hand2("QQJJJ"), HandType::FiveOfAKind);

        assert_eq!(parse_hand2("QJJJJ"), HandType::FiveOfAKind);
        assert_eq!(parse_hand2("Q2JJJ"), HandType::FourOfAKind);
        assert_eq!(parse_hand2("Q23JJ"), HandType::ThreeOfAKind);
        assert_eq!(parse_hand2("Q234J"), HandType::OnePair);

        assert_eq!(parse_hand2("QQKKJ"), HandType::FullHouse);
        assert_eq!(parse_hand2("QQQ2J"), HandType::FourOfAKind);
        assert_eq!(parse_hand2("QQ23J"), HandType::ThreeOfAKind);
    }

    #[test]
    fn test_compare_hands2() {
        assert_eq!(compare_hands2("QQQQ2", "JKKK2"), Ordering::Greater);
    }
}