use regex::Regex;
use std::{collections::VecDeque, iter};
advent_of_code::solution!(9);


pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"(\d+) players; last marble is worth (\d+) points").unwrap();

    let (_, [total_number_of_players_string, last_marble_score_string]) = re.captures(input).unwrap().extract();

    let total_number_of_players = total_number_of_players_string.parse::<u32>().unwrap();
    let last_marble_score = last_marble_score_string.parse::<u32>().unwrap();

    let mut marbles = VecDeque::new();
    marbles.push_back(0 as u32);
    marbles.push_front(1 as u32);

    let mut scores: Vec<u32> = iter::repeat(0).take(total_number_of_players as usize).collect();

    for marble_to_add in 2..=last_marble_score {
        if marble_to_add % 23 == 0 {
            let player_number_size = (marble_to_add % total_number_of_players) as usize;
            scores[player_number_size] += marble_to_add;

            marbles.rotate_right(7);

            let marble_to_remove = marbles.pop_front().unwrap();

            scores[player_number_size] += marble_to_remove;
        } else {
            marbles.rotate_left(2);
            marbles.push_front(marble_to_add);
        }
    }

    let highest = scores
    .into_iter()
    .fold(0, |total, score| {
        if score > total {
            score
        } else {
            total
        }
    });
    Some(highest)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(\d+) players; last marble is worth (\d+) points").unwrap();

    let (_, [total_number_of_players_string, last_marble_score_string]) = re.captures(input).unwrap().extract();

    let total_number_of_players = total_number_of_players_string.parse::<u32>().unwrap();
    let last_marble_score = last_marble_score_string.parse::<u32>().unwrap() * 100;

    let mut marbles = VecDeque::new();
    marbles.push_back(0 as u32);
    marbles.push_front(1 as u32);

    let mut scores: Vec<u32> = iter::repeat(0).take(total_number_of_players as usize).collect();

    for marble_to_add in 2..last_marble_score + 1 {
        if marble_to_add % 23 == 0 {
            let player_number_size = (marble_to_add % total_number_of_players) as usize;
            scores[player_number_size] += marble_to_add;

            marbles.rotate_right(7);

            let marble_to_remove = marbles.pop_front().unwrap();

            scores[player_number_size] += marble_to_remove;
        } else {
            marbles.rotate_left(2);
            marbles.push_front(marble_to_add);
        }
    }

    let highest = scores
    .into_iter()
    .fold(0, |total, score| {
        if score > total {
            score
        } else {
            total
        }
    });
    Some(highest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8317));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(74765078));
    }
}
