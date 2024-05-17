advent_of_code::solution!(1);
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<i32> {
    input
    .lines()
    .map(|str_v| str_v.parse::<i32>().unwrap())
    .reduce(|total, int_v| total + int_v)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut frequency = 0;
    let mut frequencies = HashSet::new();

    for int_value in input
    .lines()
    .map(|str_v| str_v.parse::<i32>().unwrap())
    .cycle() {
        frequency += int_value;
        if frequencies.contains(&frequency) {
            break
        } else {
            frequencies.insert(frequency);
        }
    }
    Some(frequency)
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
