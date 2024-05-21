advent_of_code::solution!(2);
use std::collections::HashMap;
use itertools::Itertools;
use std::iter::zip;

pub fn part_one(input: &str) -> Option<u32> {
    let count_2_and_3 = input
    .lines().map(
        |line| {
            line.chars()
            .fold(
                HashMap::new(),
                |mut count_hash, char| {
                    count_hash
                    .entry(char)
                    .and_modify(|count| { *count += 1 })
                    .or_insert(1);
                    count_hash
                }
            )
            .values()
            .fold(
                (false, false),
                |is_2_and_3, count| {
                    (is_2_and_3.0 || (*count == 2), is_2_and_3.1 || (*count == 3))
                }
            )
        }
    ).fold(
        (0, 0),
        |count_2_and_3, is_2_and_3| {
            (count_2_and_3.0 + is_2_and_3.0 as u32, count_2_and_3.1 + is_2_and_3.1 as u32)
        }
    );

    Some(count_2_and_3.0 * count_2_and_3.1)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut answer = String::from("");
    for word_pair in input
    .lines()
    .map(
        |line_string| line_string.chars().collect_vec()
    )
    .combinations(2) {
        let not_unique_str = zip(word_pair[0].iter(), word_pair[1].iter())
        .fold(
            String::from(""),
            |mut not_unique_str, (first_word_letter, second_word_letter)| {
                if first_word_letter == second_word_letter {
                    not_unique_str.push(*first_word_letter);
                }
                not_unique_str
            }
        );

        if not_unique_str.len() == word_pair[0].len() - 1 {
            answer = not_unique_str;
            break;
        }

    }

    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(String::from("fgij")));
    }
}
