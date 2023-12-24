advent_of_code::solution!(2);

const R_NUM: u32 = 12;
const G_NUM: u32 = 13;
const B_NUM: u32 = 14;

pub struct Game {
    id: u32,
    rounds: Vec<Round>,
}

pub struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .split('\n')
        .map(parse_game)
        .filter(is_valid)
        .map(|game| game.id)
        .reduce(|acc, cur| acc + cur)
}

pub fn is_valid(game: &Game) -> bool {
    game.rounds
        .iter()
        .all(|round| round.red <= R_NUM && round.green <= G_NUM && round.blue <= B_NUM)
}

pub fn parse_game(game: &str) -> Game {
    let mut split_line = game.split(':');
    let mut game_part = split_line.next().unwrap().trim().split(' ');
    let id = game_part.nth(1).unwrap().parse::<u32>().unwrap();
    let rounds = split_line
        .next()
        .unwrap()
        .trim()
        .split(';')
        .map(parse_round)
        .collect::<Vec<Round>>();

    Game { id, rounds }
}

pub fn parse_round(round: &str) -> Round {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for part in round.split(',') {
        let mut part_split = part.trim().split(' ');
        let num = part_split.next().unwrap().parse::<u32>().unwrap();

        match part_split.next().unwrap() {
            "red" => red = num,
            "green" => green = num,
            "blue" => blue = num,
            _ => panic!("Invalid color"),
        }
    }

    Round { red, green, blue }
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
        assert_eq!(result, Some(3059));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3059));
    }
}
