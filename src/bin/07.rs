use advent_of_code::template::tools::clean_row;
use std::collections::HashMap;
advent_of_code::solution!(7);

#[derive(Debug)]
pub struct Hand {
    cards: Vec<Card>,
    bet: u32,
}

#[derive(Debug)]
pub struct Card {
    character: char,
    value: u32,
}

pub fn get_mapper() -> HashMap<char, u32> {
    [
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('J', 11),
        ('Q', 12),
        ('K', 13),
        ('A', 14),
    ]
    .iter()
    .cloned()
    .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input
        .trim()
        .split('\n')
        .map(clean_row)
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    let hands = lines
        .iter()
        .map(|line| parse_hand(line))
        .collect::<Vec<Hand>>();

    println!("{:#?}", hands);

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    part_one(input)
}

pub fn parse_hand(line: &str) -> Hand {
    let mut values = line
        .split(' ')
        .map(clean_row)
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    let cards = values
        .first()
        .unwrap()
        .chars()
        .map(parse_card)
        .collect::<Vec<Card>>();

    let bet = values.get(1).unwrap().parse::<u32>().unwrap();

    Hand { cards, bet }
}

pub fn parse_card(c: char) -> Card {
    Card {
        character: c,
        value: *get_mapper().get(&c).unwrap(),
    }
}

pub fn compare_hands(hand_one: Hand, hand_two: Hand) -> bool {
    false
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
