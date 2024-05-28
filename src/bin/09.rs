use regex::Regex;
use std::{collections::HashMap, hash::Hash, iter};
advent_of_code::solution!(9);

struct Marble {
    clockwise: u32,
    anti_clockwise: u32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"(\d+) players; last marble is worth (\d+) points").unwrap();

    let (_, [total_number_of_players_string, last_marble_score_string]) = re.captures(input).unwrap().extract();

    let total_number_of_players = total_number_of_players_string.parse::<u32>().unwrap();
    let last_marble_score = last_marble_score_string.parse::<u32>().unwrap();

    let mut last_marble_score_found = 0;

    let mut marbles = HashMap::new();
    marbles.insert(0 as u32, Marble{clockwise: 0, anti_clockwise:0});

    let mut scores: Vec<u32> = iter::repeat(0).take(total_number_of_players as usize).collect();

    let mut marble_to_add: u32 = 1;

    let mut current_marble: u32 = 0;
    let mut player_number: u32= 0;

    while last_marble_score_found != last_marble_score {
        if current_marble % 23 == 0 {
            scores[player_number as usize] += current_marble;

            //Remove marble
        } else {
            let left_marble_number = marbles.get(&current_marble).unwrap().clockwise;
            let mut left_marble = *marbles.get(&left_marble_number).unwrap();
            left_marble.clockwise = marble_to_add;

            let right_marble_number = left_marble.clockwise;
            let right_marble = marbles.get(&right_marble_number).unwrap();
            right_marble.anti_clockwise = marble_to_add;

            marbles.insert(marble_to_add, Marble{clockwise: right_marble_number, anti_clockwise: left_marble_number});

            current_marble = marble_to_add;
        }

        marble_to_add += 1;
        player_number = (player_number + 1) % total_number_of_players;
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
