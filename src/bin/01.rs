advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    input
        .split('\n')
        .map(obtain_number)
        .reduce(|acc, number| acc + number)
}

pub fn obtain_number(input: &str) -> u32 {
    let l = input
        .chars()
        .find(|&c| c.is_ascii_digit())
        .unwrap()
        .to_digit(10);
    let r = input
        .chars()
        .rfind(|&c| c.is_ascii_digit())
        .unwrap()
        .to_digit(10);

    if let (Some(l), Some(r)) = (l, r) {
        return l * 10 + r;
    }

    0
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
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }
}
