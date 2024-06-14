use std::{collections::HashSet, collections::VecDeque};

use regex::Regex;
advent_of_code::solution!(12);


pub fn part_one(input: &str) -> Option<i32> {
    let initial_re = Regex::new(r"initial state: ([\.#]+)").unwrap();
    let re = Regex::new(r"([\.#]+) => ([\.#])").unwrap();

    let mut lines = input.lines();

    let initial_line = lines.next();

    let (_, [initial_state_str]) = initial_re.captures(initial_line.unwrap()).unwrap().extract();

    let (mut state_hash, mut min_pot, mut max_pot) = initial_state_str
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
    );

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
                                return pattern_binary + i32::pow( 2, index as u32)
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

    for _ in 0..20 {
        (state_hash, min_pot, max_pot) = ((min_pot - 2)..=(max_pot + 2))
        .fold(
            (HashSet::new(), i32::max_value(), i32::min_value()),
            |(mut new_state_hash, min_pot, max_pot), index| {
                let this_pattern_binary = (-2..=2)
                .enumerate()
                .fold(
                    0,
                    |this_pattern_binary, (enumerate_index, index_diff) | {
                        if state_hash.contains(&(index + index_diff)) {
                            return this_pattern_binary + i32::pow( 2, enumerate_index as u32)
                        }
                            this_pattern_binary
                    }
                );
                if growth_patterns.contains(&this_pattern_binary) {
                    new_state_hash.insert(index);
                    if index < min_pot {
                        return (new_state_hash, index, index)
                    } else {
                        return (new_state_hash, min_pot, index)
                    }
                }
                (new_state_hash, min_pot, max_pot)
            }
        );
    }

    let total = state_hash
    .into_iter()
    .fold(
        0,
        |total, value| {
            total + value
        }
    );

    Some(total)
}

pub fn part_two(input: &str) -> Option<i64> {
    let initial_re = Regex::new(r"initial state: ([\.#]+)").unwrap();
    let re = Regex::new(r"([\.#]+) => ([\.#])").unwrap();

    let mut lines = input.lines();

    let initial_line = lines.next();

    let (_, [initial_state_str]) = initial_re.captures(initial_line.unwrap()).unwrap().extract();

    let (mut state_hash, mut min_pot, mut max_pot) = initial_state_str
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
    );

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
                                return pattern_binary + i32::pow( 2, index as u32)
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

    let mut last_total = 0;

    let mut last_10_diffs = VecDeque::new();

    for i in 0..1000000 {
        (state_hash, min_pot, max_pot) = ((min_pot - 2)..=(max_pot + 2))
        .fold(
            (HashSet::new(), i32::max_value(), i32::min_value()),
            |(mut new_state_hash, min_pot, max_pot), index| {
                let this_pattern_binary = (-2..=2)
                .enumerate()
                .fold(
                    0,
                    |this_pattern_binary, (enumerate_index, index_diff) | {
                        if state_hash.contains(&(index + index_diff)) {
                            return this_pattern_binary + i32::pow( 2, enumerate_index as u32)
                        }
                            this_pattern_binary
                    }
                );
                if growth_patterns.contains(&this_pattern_binary) {
                    new_state_hash.insert(index);
                    if index < min_pot {
                        return (new_state_hash, index, index)
                    } else {
                        return (new_state_hash, min_pot, index)
                    }
                }
                (new_state_hash, min_pot, max_pot)
            }
        );

        let total = state_hash
        .iter()
        .fold(
            0,
            |total, value| {
                total + value
            }
        );

        last_10_diffs.push_back(total - last_total);

        last_total = total;

        if last_10_diffs.len() > 10 {
            last_10_diffs.pop_front();

            let mut stabilised_diff = true;
            let last_diff = last_10_diffs[0];
            for diff in last_10_diffs.iter() {
                if last_diff != *diff {
                    stabilised_diff = false;
                    break
                }
            }

            if stabilised_diff {
                println!("{}", last_diff);
                let remaining_iterations = 50000000000 - (i as i64 + 1);

                let big_total = last_diff as i64 * remaining_iterations + total as i64;

                return Some(big_total)
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(325));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
