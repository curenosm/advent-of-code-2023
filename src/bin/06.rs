use advent_of_code::template::tools::{clean_row, parse_number_list};
advent_of_code::solution!(6);

#[derive(Debug)]
pub struct Race {
    id: u32,
    time: u32,
    distance: u32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let races = parse_races(input);

    println!("{:#?}", races);

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    part_one(input)
}

pub fn parse_races(input: &str) -> Vec<Race> {
    let all_lines = input
        .trim()
        .split('\n')
        .map(clean_row)
        .collect::<Vec<&str>>();

    let mut times = all_lines[0]
        .split(':')
        .map(clean_row)
        .collect::<Vec<&str>>();

    let mut distances = all_lines[1]
        .split(':')
        .map(clean_row)
        .collect::<Vec<&str>>();

    let times_res = parse_number_list(times[1]);
    let distances_res = parse_number_list(distances[1]);
    let mut result = Vec::new();

    for i in 0..times_res.len() {
        result.push(Race {
            id: i as u32,
            time: times_res[i],
            distance: distances_res[i],
        });
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
