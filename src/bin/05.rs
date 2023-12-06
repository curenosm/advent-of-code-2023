use advent_of_code::template::tools::{clean_row, parse_number_list};
use std::collections::HashMap;
advent_of_code::solution!(5);

pub struct RoundMapping {
    range_mappings: Vec<RangeMapping>,
}

pub struct RangeMapping {
    destination: u32,
    source: u32,
    length: u32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let seeds = get_seeds(input);
    let round_mappings = get_round_mappings(input);
    let all_maps = get_all_maps(&round_mappings);
    let composed_map = get_composed_map(all_maps);
    let mut locations: Vec<u32> = Vec::new();

    for i in seeds.iter() {
        locations.push(*composed_map.get(i).unwrap());
    }

    locations.iter().min().copied()
}

pub fn part_two(input: &str) -> Option<u32> {
    part_one(input)
}

pub fn get_seeds(input: &str) -> Vec<u32> {
    let all_lines = input
        .trim()
        .split('\n')
        .map(clean_row)
        .collect::<Vec<&str>>();

    let mut seeds_line = all_lines[0]
        .split(':')
        .map(clean_row)
        .collect::<Vec<&str>>();

    parse_number_list(seeds_line[1])
}

pub fn get_round_mappings(input: &str) -> Vec<RoundMapping> {
    let res = input
        .trim()
        .split('\n')
        .map(clean_row)
        .collect::<Vec<&str>>();

    println!("{:?}", res);

    Vec::new()
}

pub fn parse_round_mapping(chunk: Vec<&str>) -> RoundMapping {
    let mut range_mappings: Vec<RangeMapping> = Vec::new();

    for i in chunk.iter().skip(1) {
        range_mappings.push(parse_range_mapping(parse_number_list(i)));
    }

    RoundMapping { range_mappings }
}

pub fn get_all_maps(map_structs: &Vec<RoundMapping>) -> Vec<HashMap<u32, u32>> {
    let mut maps = Vec::new();

    maps
}

pub fn get_composed_map(maps: Vec<HashMap<u32, u32>>) -> HashMap<u32, u32> {
    HashMap::new()
}

pub fn parse_range_mapping(numbers: Vec<u32>) -> RangeMapping {
    RangeMapping {
        destination: numbers[0],
        source: numbers[1],
        length: numbers[2],
    }
}

pub fn apply_mapping(map: &HashMap<u32, u32>, numbers: &[u32]) -> Vec<u32> {
    numbers
        .iter()
        .map(|n| *map.get(n).unwrap())
        .collect::<Vec<u32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, 35);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, 35);
    }
}
