use std::collections::HashSet;

advent_of_code::solution!(5);

fn reduce_polymer_stack(mut start_stack: Vec<char>, mut add_stack: Vec<char>) -> u32 {
    while add_stack.len() > 0 {
        let start_value_option = start_stack.pop();
        let add_value = add_stack.pop().unwrap();

        if start_value_option.is_none() {
            start_stack.push(add_value)
        } else {
            let start_value = start_value_option.unwrap();
            if !(add_value.to_ascii_lowercase() == start_value.to_ascii_lowercase() && add_value.is_ascii_lowercase() != start_value.is_ascii_lowercase()) {
                start_stack.push(start_value);
                start_stack.push(add_value);
            }
        }
    }

    start_stack.len() as u32
}

fn reduce_polymer(letters: &Vec<char>) -> u32 {
    let start_stack = Vec::new();
    let add_stack = letters
    .into_iter()
    .rfold(
        Vec::new(),
        |mut letter_stack, this_letter| {
            letter_stack.push(*this_letter);
            letter_stack
        }
    );

    reduce_polymer_stack(start_stack, add_stack)
}

fn reduce_polymer_with_filter(letters: &Vec<char>, filter_letter: char) -> u32 {
    let start_stack = Vec::new();
    let add_stack = letters
    .into_iter()
    .rfold(
        Vec::new(),
        |mut letter_stack, this_letter| {
            if this_letter.to_ascii_lowercase() != filter_letter {
                letter_stack.push(*this_letter);
            }
            letter_stack
        }
    );

    reduce_polymer_stack(start_stack, add_stack)
}

pub fn part_one(input: &str) -> Option<u32> {
    let letters: Vec<char> = input
    .chars()
    .collect();

    Some(reduce_polymer(&letters))
}

pub fn part_two(input: &str) -> Option<u32> {
    let letters: Vec<char> = input
    .chars()
    .collect();

    let possible_letters = letters
    .iter()
    .fold(
        HashSet::new(),
        |mut possible_letters, this_letter| {
            possible_letters.insert(this_letter.to_ascii_lowercase());
            possible_letters
        }
    );

    let mut best_letter_count = 10000;
    for filter_letter in possible_letters.into_iter() {
        let letter_count = reduce_polymer_with_filter(&letters, filter_letter);

        if letter_count < best_letter_count{
            best_letter_count = letter_count;
        }
    }

    Some(best_letter_count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10));
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(3));
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 3));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
