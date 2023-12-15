
#[derive(Debug, PartialEq)]
struct Number {
    value: u32,
    len: u32,
    x: u32,
    y: u32
}

#[derive(Debug, PartialEq)]
struct Symbole {
    value: char,
    x: u32,
    y: u32
}

fn locate_number(input: &str) -> Vec<Number>{
    let mut result = Vec::new();
    for (y, line) in input.lines().enumerate(){
        let mut size = 0;
        for(x, c) in line.chars().enumerate(){
            if size > 0 {
                size -= 1;
                continue;
            }
            if c.is_digit(10){
                let mut slice = line[x..].chars();
                while slice.next().is_some_and(|x| x.is_digit(10)) {
                    size +=1;
                }
                result.push(Number { value: line[x..x+size].parse().unwrap(), len: size as u32, x: x as u32, y: y as u32 })
            }
        }
    }
    //dbg!(&result);
    result
}

fn locate_symbole(input:&str) -> Vec<Symbole>{
    let mut result = Vec::new();
    for (y, line) in input.lines().enumerate(){
        for (x, c) in line.chars().enumerate(){
            if !c.is_alphanumeric() && c != '.'{
                result.push(Symbole { value: c, x: x as u32, y: y as u32 });
            }
        }
    }
    //dbg!(&result);
    result
}

fn is_adjacent(number: &Number, symbole: &Symbole) -> bool {
    if (number.y>0 && symbole.y < number.y-1) || symbole.y > number.y+1 {
        return false;
    }
    if (number.x > 0 && symbole.x < number.x-1) || symbole.x > number.x+number.len+1 {
        return false;
    }
    true
}

pub fn solve_day3_part1(input: &str) -> u32 {
    let mut result = 0;
    //let mut set = HashSet::new();
    let numbers = locate_number(input);
    let symboles = locate_symbole(input);
    'outer: for n in numbers.iter(){
        for s in symboles.iter(){
            if is_adjacent(n, s){
                result += n.value;
                continue 'outer;
            }
        }
        //println!("{} is not a part number", n.value);
    }
    result
}

pub fn _solve_day3_part2(_input: &str) -> u32 {
    0
}

pub fn _day3_p1_debug(input: &str) {
    let numbers = locate_number(input);
    let symboles = locate_symbole(input);

    let mut alone = Vec::new();

    'outer: for n in numbers.iter(){
        for s in symboles.iter(){
            if is_adjacent(n, s){
                continue 'outer;
            }
        }
        alone.push(n);
        //println!("{} is not a part number", n.value);
    }

    for (y, line) in input.lines().enumerate(){
        for (x, c) in line.chars().enumerate(){
            if c.is_digit(10){
                let mut solo = false;
                for a in alone.iter() {
                    if y as u32 == a.y && x as u32 >= a.x && x as u32 <= a.x+a.len {
                        solo = true;
                        break;
                    }
                }
                if solo {
                    //print!("{}", c.to_string().blue());
                    print!(" ");
                }
                else{
                    //print!("{}", c.to_string().red());
                    print!(" ");
                }
            }
            else if !c.is_alphanumeric() && c != '.'{
                //print!("{}", c.to_string().green());
                print!(" ");
            }
            else if c == '.'{
                print!(" ");
            }
            else {
                print!("{}", c);
            }
        }
        println!("");
    }
}

#[cfg(test)]
mod tests{
    use crate::day3::{Number, Symbole, is_adjacent, solve_day3_part1};

    use super::{locate_number, locate_symbole};

    const EXAMPLE : &'static str =
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    const EXAMPLE_ALT : &'static str =
"12.......*..
+.........34
.......-12..
..78........
..*....60...
78..........
.......23...
....90*12...
............
2.2......12.
.*.........*
1.1.......56";

    const EXAMPLE_ALT2 : &'static str =
"12.......*..
+.........34
.......-12..
..78........
..*....60...
78.........9
15.....23..$
8...90*12...
............
2.2......12.
.*.........*
1.1..503+.56
";

const EXAMPLE_ALT3 : &'static str =
"503+
";

const EXAMPLE_ALT4 : &'static str =
"....11
......
....22
33+...
......
44+.44
......
+55.55
.....+
";

const EXAMPLE_ALT5 : &'static str =
"........
.24..4..
......*.
";

const EXAMPLE_ALT6 : &'static str =
"....................
..-52..52-..52..52..
..................-.
";

    #[test]
    fn test_locate_number(){
        let result = locate_number(EXAMPLE);
        assert!(result.contains(&Number{value:467, len:3, x:0, y:0}));
        assert!(result.contains(&Number{value:114, len:3, x:5, y:0}));
        assert!(result.contains(&Number{value:35, len:2, x:2, y:2}));
        assert!(result.contains(&Number{value:633, len:3, x:6, y:2}));
    }

    #[test]
    fn test_locate_symbole(){
        let result = locate_symbole(EXAMPLE);
        assert!(result.contains(&Symbole{value:'*', x:3, y:1}));
        assert!(result.contains(&Symbole{value:'#', x:6, y:3}));
    }

    #[test]
    fn test_is_adjacent(){
        assert!(is_adjacent(&Number{value:467, len:3, x:0, y:0}, &Symbole{value:'*', x:3, y:1}));
        assert!(is_adjacent(&Number{value:35, len:2, x:2, y:2}, &Symbole{value:'*', x:3, y:1}));
        assert!(is_adjacent(&Number{value: 664,len: 3,x: 1,y: 9}, &Symbole {value: '$', x: 3, y: 8,}));
    }

    #[test]
    fn test_day3_p1() {
        assert_eq!(solve_day3_part1(EXAMPLE), 4361);
        assert_eq!(solve_day3_part1(EXAMPLE_ALT), 413);
        assert_eq!(solve_day3_part1(EXAMPLE_ALT2), 925);
        assert_eq!(solve_day3_part1(EXAMPLE_ALT3), 503);
        assert_eq!(solve_day3_part1(EXAMPLE_ALT4), 187);
        assert_eq!(solve_day3_part1(EXAMPLE_ALT5), 4);
        assert_eq!(solve_day3_part1(EXAMPLE_ALT6), 156);
    }

    #[test]
    fn test_day3_p2() {
    }

}