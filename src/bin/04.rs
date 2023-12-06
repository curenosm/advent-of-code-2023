use advent_of_code::template::tools::{clean_row, parse_number_list};
advent_of_code::solution!(4);

#[derive(Debug)]
pub struct Card {
    id: u32,
    own_numbers: [u32; 5],
    winner_numbers: [u32; 8],
}

pub fn part_one(input: &str) -> Option<u32> {
    let cards = input
        .trim()
        .split('\n')
        .map(clean_row)
        .map(parse_card)
        .collect::<Vec<Card>>();

    print_cards(&cards);

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

pub fn parse_card(card_str: &str) -> Card {
    let mut own_numbers = [0; 5];
    let mut winner_numbers = [0; 8];
    let mut split_line = card_str.split(':').map(clean_row);
    let mut card_part = split_line
        .next()
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty());
    let id = card_part.nth(1).unwrap().parse::<u32>().unwrap();
    let mut numbers_part = split_line.next().unwrap().trim().split('|').map(clean_row);
    let own = parse_number_list(numbers_part.next().unwrap());
    let winners = parse_number_list(numbers_part.next().unwrap());

    for i in 0..8 {
        if i < 5 {
            own_numbers[i] = own[i];
        }
        winner_numbers[i] = winners[i];
    }

    Card {
        id,
        own_numbers,
        winner_numbers,
    }
}

pub fn print_cards(cards: &[Card]) {
    cards.iter().for_each(|card| {
        println!("Card {}:", card.id);
        println!("\tOwn numbers: {:?}", card.own_numbers);
        println!("\tWinner numbers: {:?}", card.winner_numbers);
        println!();
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, 13);
    }
}
