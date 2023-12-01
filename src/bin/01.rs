advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(input
        .split("\n")
        .map(|line| obtain_number(line))
        .reduce(|acc, number| acc + number)?)
}

pub fn obtain_number(input: &str) -> u32 {

    let digit1 = input.chars().find(|&c| c.is_digit(10)).unwrap().to_digit(10);
    let digit2 = input.chars().rfind(|&c| c.is_digit(10)).unwrap().to_digit(10);

    if let (Some(digit1), Some(digit2)) = (digit1, digit2) {
        return digit1 * 10 + digit2;
    }

    0
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input
        .split("\n")
        .map(|line| obtain_number(line))
        .reduce(|acc, number| acc + number)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, 142);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, 142);
    }
}
