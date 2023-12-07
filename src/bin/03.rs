use advent_of_code::template::tools::{clean_row, parse_number_list};
advent_of_code::solution!(3);

#[derive(Debug)]
pub struct MatrixNumber {
    value: u32,
    starts: (usize, usize),
    is_part: bool,
}

const SYMBOLS_TO_FIND: [char; 4] = ['#', '*', '+', '$'];

pub fn part_one(input: &str) -> Option<u32> {
    let lines = to_matrix(input);
    let all_numbers = get_all_numbers(&lines);
    println!("All numbers: {:#?}", all_numbers);

    let mut part_numbers: Vec<MatrixNumber> = Vec::new();

    println!("{:#?}", lines);

    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if SYMBOLS_TO_FIND.contains(c) {
                println!("Found symbol {} at ({}, {})", c, i, j);

                let adj = get_adjacent_numbers(&lines, i, j, &part_numbers);
                println!("Adjacent numbers: {:#?}", adj);

                part_numbers.push(MatrixNumber {
                    value: 0,
                    starts: (i, j),
                    is_part: false,
                });
            }
        }
    }

    part_numbers
        .iter()
        .map(|pn| pn.value)
        .reduce(|acc, cur| acc + cur)
}

pub fn get_all_numbers(matrix: &[Vec<char>]) -> Vec<MatrixNumber> {
    let mut all_numbers: Vec<MatrixNumber> = Vec::new();

    for (i, line) in matrix.iter().enumerate() {
        let mut helper_str = String::new();

        for (j, character) in line.iter().enumerate() {
            // When the current character is not a digit you'll parse
            // the current string and add it to all_numbers.

            if character.is_ascii_digit() {
                helper_str.push(*character);
            } else if !helper_str.is_empty() {
                // After that, empty the accumulator string and continue
                // until you find all the numbers,

                all_numbers.push(MatrixNumber {
                    value: helper_str.parse::<u32>().unwrap(),
                    starts: (i, j - helper_str.len()),
                    is_part: false,
                });

                helper_str = String::new();
            }
        }
    }

    all_numbers
}

pub fn get_adjacent_numbers(
    matrix: &[Vec<char>],
    row: usize,
    col: usize,
    part_numbers: &[MatrixNumber],
) -> Vec<MatrixNumber> {
    Vec::new()
}

pub fn to_matrix(input: &str) -> Vec<Vec<char>> {
    input
        .split('\n')
        .map(clean_row)
        .map(|r| r.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

pub fn print_matrix(matrix: &[Vec<char>]) {
    matrix.iter().for_each(|line| {
        line.iter().for_each(|c| print!("{}", c));
        println!();
    })
}

pub fn part_two(input: &str) -> Option<u32> {
    part_one(input)
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
