use std::str::Lines;

use itertools::Itertools;

#[derive(PartialEq, Debug)]
struct MappingRange {
    range: (u64, u64),
    delta: i64
}

fn parse_seeds(input: &mut Lines) -> Vec<u64> {
    const SEEDS_STR: &str = "seeds:";
    let seeds_line = input.next().unwrap();
    assert!(seeds_line.starts_with(SEEDS_STR));
    let seeds_str = &seeds_line[SEEDS_STR.len()..];
    seeds_str.trim().split_ascii_whitespace().map(|x| x.parse::<u64>().unwrap()).collect()
}

fn parse_range(input: &mut Lines) -> Vec<MappingRange> {
    let header = input.next().unwrap();
    assert!(header.ends_with("map:"));
    //println!("{}", header);
    let mut output = Vec::new();
    loop {
        let line = input.next().unwrap();
        if line.is_empty() {
            break;
        }
        let values : Vec<u64> = line.trim().split_ascii_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();
        output.push(MappingRange { range: (values[1], values[1]+values[2]-1), delta: values[0] as i64 -values[1] as i64 });
    }
    output.sort_by(|a, b| a.range.cmp(&b.range));
    output
}

fn get_target_value_form_map(value: u64, range: &Vec<MappingRange>) -> u64 {
    for r in range {
        if r.range.0 <= value && value <= r.range.1 {
            return ((value as i64) + r.delta) as u64;
        }
    }
    value
}

fn get_seed_ranges(input: &mut Lines) -> Vec<(u64, u64)> {
    let mut result : Vec<(u64, u64)> = Vec::new();
    let seeds = parse_seeds(input);
    let mut iter = seeds.iter();
    while let Some((x, y)) = iter.next_tuple() {
        result.push((*x, *x+*y-1));
    }
    result
}

fn get_target_value_form_map2(input_range: Vec<(u64, u64)>, target_range: &Vec<MappingRange>)-> Vec<(u64,u64)> {
    let mut result = Vec::new();

    let mut target_it = target_range.iter();
    let mut input_range = input_range.clone();
    let mut input_it = input_range.drain(..);
    let mut cur_range = input_it.next().unwrap();
    let mut target = target_it.next().unwrap();

    loop {
        if cur_range.1 < target.range.0 {
            result.push(cur_range);
            cur_range = match input_it.next() {
                Some(range) => range,
                None => break,
            };
            continue;
        }
        else if cur_range.0 > target.range.1 {
            target = match target_it.next() {
                Some(mapping_range) => mapping_range,
                None => {
                    result.push(cur_range);
                    result.extend_from_slice(input_it.as_slice());
                    break;
                },
            }
        }
        else {
            if cur_range.0 < target.range.0 {
                result.push((cur_range.0, target.range.0-1));
                cur_range = (target.range.0, cur_range.1);
            }

            if cur_range.1 <= target.range.1 {
                let new_range = ((cur_range.0 as i64 +target.delta) as u64, (cur_range.1 as i64 + target.delta) as u64);
                result.push(new_range);

                cur_range = match input_it.next() {
                    Some(range) => range,
                    None => break,
                };
            }
            else {
                let new_range = ((cur_range.0 as i64 +target.delta) as u64, (target.range.1 as i64 + target.delta) as u64);
                result.push(new_range);
                cur_range = (target.range.1+1, cur_range.1);
            }
        }
    }

    result.sort();
    result
}

pub fn solve_day5_part1(input: &mut Lines) -> u64 {

    let seeds = parse_seeds(input);

    let mut result = u64::MAX;

    input.next();
    let seed_to_soil_map = parse_range(input);
    let soil_to_fertilizer_map = parse_range(input);
    let fertilizer_to_water_map = parse_range(input);
    let water_to_light_map = parse_range(input);
    let light_to_temperature_map = parse_range(input);
    let temperature_to_humidity_map = parse_range(input);
    let humidity_to_location_map = parse_range(input);


    for seed in seeds {
        let soil = get_target_value_form_map(seed, &seed_to_soil_map);
        let fert = get_target_value_form_map(soil, &soil_to_fertilizer_map);
        let water = get_target_value_form_map(fert, &fertilizer_to_water_map);
        let light = get_target_value_form_map(water, &water_to_light_map);
        let temp = get_target_value_form_map(light, &light_to_temperature_map);
        let humi = get_target_value_form_map(temp, &temperature_to_humidity_map);
        let loc = get_target_value_form_map(humi, &humidity_to_location_map);
        
        if loc < result { result = loc;}
    }
    result
}

pub fn solve_day5_part2(input: &mut Lines) -> u64 {
    let mut seed_ranges = get_seed_ranges(input);
    seed_ranges.sort();
    input.next();

    let seed_to_soil_map = parse_range(input);
    let soil_to_fertilizer_map = parse_range(input);
    let fertilizer_to_water_map = parse_range(input);
    let water_to_light_map = parse_range(input);
    let light_to_temperature_map = parse_range(input);
    let temperature_to_humidity_map = parse_range(input);
    let humidity_to_location_map = parse_range(input);

    for i in 0..seed_ranges.len()-1 {
        assert!(seed_ranges[i].1 < seed_ranges[i+1].0);
    }

    for i in 0..seed_to_soil_map.len()-1 {
        assert!(seed_to_soil_map[i].range.1 < seed_to_soil_map[i+1].range.0);
    }

    for i in 0..soil_to_fertilizer_map.len()-1 {
        assert!(soil_to_fertilizer_map[i].range.1 < soil_to_fertilizer_map[i+1].range.0);
    }

    for i in 0..fertilizer_to_water_map.len()-1 {
        assert!(fertilizer_to_water_map[i].range.1 < fertilizer_to_water_map[i+1].range.0);
    }

    for i in 0..water_to_light_map.len()-1 {
        assert!(water_to_light_map[i].range.1 < water_to_light_map[i+1].range.0);
    }

    for i in 0..light_to_temperature_map.len()-1 {
        assert!(light_to_temperature_map[i].range.1 < light_to_temperature_map[i+1].range.0);
    }

    for i in 0..temperature_to_humidity_map.len()-1 {
        assert!(temperature_to_humidity_map[i].range.1 < temperature_to_humidity_map[i+1].range.0);
    }

    for i in 0..humidity_to_location_map.len()-1 {
        assert!(humidity_to_location_map[i].range.1 < humidity_to_location_map[i+1].range.0);
    }

    let soil = get_target_value_form_map2(seed_ranges, &seed_to_soil_map);
    let fert = get_target_value_form_map2(soil, &soil_to_fertilizer_map);
    let water = get_target_value_form_map2(fert, &fertilizer_to_water_map);
    let light = get_target_value_form_map2(water, &water_to_light_map);
    let temp = get_target_value_form_map2(light, &light_to_temperature_map);
    let humi = get_target_value_form_map2(temp, &temperature_to_humidity_map);
    let loc = get_target_value_form_map2(humi, &humidity_to_location_map);

    //dbg!(&loc);

    loc[0].0
}

#[cfg(test)]
mod tests{
    use crate::day5::{parse_seeds, parse_range, MappingRange, get_target_value_form_map, get_seed_ranges, solve_day5_part2};

    use super::solve_day5_part1;

    const EXAMPLE : &'static str =
"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4

";

const RANGE_INPUT : &'static str = 
"seed-to-soil map:
50 98 2
52 50 48

";

    #[test]
    fn test_day5_p1() {
        assert_eq!(solve_day5_part1(&mut EXAMPLE.lines()), 35);
    }

    #[test]
    fn test_day5_p2() {
        assert_eq!(solve_day5_part2(&mut EXAMPLE.lines()), 46);
    }

    #[test]
    fn test_parse_seeds() {
        assert_eq!(parse_seeds(&mut "seeds: 79 14 55 13".lines()), vec![79, 14, 55, 13]);
    }

    #[test]
    fn test_parse_range() {
        assert_eq!(parse_range(&mut RANGE_INPUT.lines()), vec![ MappingRange{delta: 2, range: (50,97)}, MappingRange{delta: -48 , range: (98,99) }]);
        //assert_eq!(parse_range(&mut RANGE_INPUT2.lines()), vec![MappingRange{delta: -48 , range: (98,99) }, MappingRange{delta: 2, range: (50,97)}]);
    }

    #[test]
    fn test_get_target_value_form_map(){
        assert_eq!(get_target_value_form_map(79, &vec![MappingRange{delta: -48 , range: (98,99) }, MappingRange{delta: 2, range: (50,97)}]), 81);
        assert_eq!(get_target_value_form_map(14, &vec![MappingRange{delta: -48 , range: (98,99) }, MappingRange{delta: 2, range: (50,97)}]), 14);
        assert_eq!(get_target_value_form_map(55, &vec![MappingRange{delta: -48 , range: (98,99) }, MappingRange{delta: 2, range: (50,97)}]), 57);
        assert_eq!(get_target_value_form_map(13, &vec![MappingRange{delta: -48 , range: (98,99) }, MappingRange{delta: 2, range: (50,97)}]), 13);
    }

    #[test]
    fn test_get_seed_ranges(){
        assert_eq!(get_seed_ranges(&mut "seeds: 79 14 55 13".lines()), vec![(79, 92), (55, 67)]);
    }

}