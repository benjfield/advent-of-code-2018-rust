
use std::{borrow::Borrow, collections::{HashMap, HashSet}};
use regex::Regex;
advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<String> {
    let re: Regex = Regex::new(r"Step (\w+) must be finished before step (\w+) can begin.").unwrap();
    let (preceding_steps, anteceding_steps) = input
    .lines()
    .fold(
        (HashMap::new(), HashMap::new()),
        |(mut preceding_steps, mut anteceding_steps), line| {
            let (_, [first_step_name, second_step_name]) = re.captures(line).unwrap().extract();

            if !preceding_steps.contains_key(first_step_name) {
                preceding_steps.insert(first_step_name, Vec::new());
            }

            let second_step_precedings = preceding_steps
            .entry(second_step_name)
            .or_insert(Vec::new());

            second_step_precedings.push(first_step_name.to_string());

            if !anteceding_steps.contains_key(second_step_name) {
                anteceding_steps.insert(second_step_name, Vec::new());
            }

            let first_step_antecedings = anteceding_steps
            .entry(first_step_name)
            .or_insert(Vec::new());

            first_step_antecedings.push(second_step_name.to_string());
            (preceding_steps, anteceding_steps)
        }
    );

    let mut letters: Vec<String> = Vec::new();
    let mut next_steps: Vec<String> = Vec::new();
    let mut processed_steps = HashSet::new();

    for (step, this_preceding_steps) in preceding_steps.iter() {
        if this_preceding_steps.len() == 0 {
            next_steps.push(step.to_string())
        }
    }

    while next_steps.len() > 0 {
        next_steps.sort_by(|a, b| b.cmp(a));

        let next_step = next_steps.pop().unwrap();
        if !processed_steps.contains(&next_step) {
            letters.push(next_step.clone());
            processed_steps.insert(next_step.clone());

            for potential_next_step in anteceding_steps.get(&next_step as &str).unwrap() {
                if !processed_steps.contains(potential_next_step) {
                    let mut all_preceding_processed = true;
                    for potential_step_preceding_step in preceding_steps.get(potential_next_step as &str).unwrap() {
                        if !processed_steps.contains(potential_step_preceding_step) {
                            all_preceding_processed = false;
                            break
                        }
                    }

                    if all_preceding_processed {
                        next_steps.push(potential_next_step.clone());
                    }
                }
            }
        }
    }


    Some(letters.join(""))
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
        assert_eq!(result, Some("CABDFE".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
