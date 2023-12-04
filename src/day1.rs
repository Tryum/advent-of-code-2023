use advent_of_code_2023::text_to_string_vec;

const ONES: [&str; 10] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine"
];


pub fn find_calibration(calibration_value: &str, with_spelled_digits: bool) -> u32 {

    let lines = text_to_string_vec(calibration_value);

    let mut result = 0;
    for line in lines {
        let digits = extract_digits(line, with_spelled_digits);
        result += digits.0*10+digits.1;
    }


    return result;
}

fn extract_digits(calibration_line: &str, with_spelled_digits : bool) -> (u32, u32) {

    let mut first = None;
    let mut last = None;
    for i in 0..calibration_line.len() {
        let byte = calibration_line.as_bytes()[i];
        if byte.is_ascii_digit() {
            last = (byte as char).to_digit(10);
        }
        else if with_spelled_digits {
            for j in 0..ONES.len(){
                let digit = ONES[j];
                if calibration_line.len() >= i+digit.len() && calibration_line[i..i+digit.len()].eq(digit) {
                    last = Some(j.try_into().unwrap());
                    break;
                }
            }
        }
        if first == None {
            first = last;
        }
    }
    (first.unwrap(), last.unwrap())
}

#[cfg(test)]
mod tests{
    use crate::day1::extract_digits;
    use crate::day1::find_calibration;

    const DAY1_P1_EXAMPLE: &'static str =
"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    
    const DAY1_P2_EXAMPLE: &'static str =
"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    
    #[test]
    fn test_day1_p1(){
        assert_eq!(find_calibration(DAY1_P1_EXAMPLE, false), 142); 
    }

    #[test]
    fn test_day1_p2(){
        assert_eq!(find_calibration(DAY1_P2_EXAMPLE, true), 281); 
    }

    #[test]
    fn test_day1_p1_extract_digits(){
        let calibration_digits = extract_digits("1abc2", false);
        assert_eq!(calibration_digits, (1, 2));
        let calibration_digits = extract_digits("pqr3stu8vwx", false);
        assert_eq!(calibration_digits, (3, 8));
        let calibration_digits = extract_digits("a1b2c3d4e5f", false);
        assert_eq!(calibration_digits, (1, 5));
        let calibration_digits = extract_digits("treb7uchet", false);
        assert_eq!(calibration_digits, (7, 7));
    }

    #[test]
    fn test_day1_p2_extract_digits(){
        let calibration_digits = extract_digits("two1nine", true);
        assert_eq!(calibration_digits, (2, 9));
        let calibration_digits = extract_digits("eightwothree", true);
        assert_eq!(calibration_digits, (8, 3));
        let calibration_digits = extract_digits("abcone2threexyz", true);
        assert_eq!(calibration_digits, (1, 3));
        let calibration_digits = extract_digits("xtwone3four", true);
        assert_eq!(calibration_digits, (2, 4));
        let calibration_digits = extract_digits("4nineeightseven2", true);
        assert_eq!(calibration_digits, (4, 2));
        let calibration_digits = extract_digits("zoneight234", true);
        assert_eq!(calibration_digits, (1, 4));
        let calibration_digits = extract_digits("7pqrstsixteen", true);
        assert_eq!(calibration_digits, (7, 6));
    }
    

}