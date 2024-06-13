use std::collections::HashSet;

use regex::Regex;
advent_of_code::solution!(12);


pub fn part_one(input: &str) -> Option<u32> {
    let initial_re = Regex::new(r"initial state: ([\.#]+)").unwrap();
    let re = Regex::new(r"([\.#]+) => [\.#]").unwrap();

    let mut lines = input.lines();

    let initial_line = lines.next();

    let (_, [initial_state_str]) = initial_re.captures(initial_line.unwrap()).unwrap().extract();

    let (state_hash, min_pot, max_pot) = initial_state_str
    .chars()
    .enumerate()
    .fold(
        (HashSet::new(), i32::max_value(), i32::min_value()),
        |(mut state_hash, min_pot, max_pot), (index, letter)| {
            if letter == '#' {
                state_hash.insert(index as i32);
                if min_pot > index as i32 {
                    return (state_hash, index as i32, max_pot)
                } else if max_pot < index as i32 {
                    return (state_hash, min_pot, index as i32)
                }
            }
            (state_hash, min_pot, max_pot)
        }
    )

    let growth_patterns = lines
    .fold(
        HashSet::new(),
        |mut growth_patterns, line| {
            let capture = re.captures(line);
            if !capture.is_none() {
                let (_, [pattern, result]) = capture.unwrap().extract();
                if result == "#" {
                    let pattern_binary = pattern
                    .chars()
                    .enumerate()
                    .fold(
                        0,
                        |pattern_binary, (index, letter)| {
                            if letter == '#' {
                                return pattern_binary + i32::pow( 2, 4 - index as u32)
                            }
                            pattern_binary
                        }
                    );

                    growth_patterns.insert(pattern_binary);
                }
            }
            growth_patterns
        }
    );

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
