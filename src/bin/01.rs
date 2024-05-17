advent_of_code::solution!(1);
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<i32> {
    let mut total = 0;

    for string_value in input.lines() {
        let int_value = string_value.parse::<i32>().unwrap();
        total += int_value;
    }

    Some(total)

}

pub fn part_two(input: &str) -> Option<i32> {
    let mut frequency = 0;

    let mut frequencies = HashSet::new();

    loop {
        let mut found_answer = false;
        for string_value in input.lines() {
            let int_value = string_value.parse::<i32>().unwrap();
            frequency += int_value;

            if frequencies.contains(&frequency) {
                found_answer = true;
                break
            } else {
                frequencies.insert(frequency);
            }
        }

        if found_answer {
            break Some(frequency)
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
