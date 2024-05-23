
use std::collections::HashMap;

use regex::Regex;
advent_of_code::solution!(7);

struct Step {
    name: String,
    preceding_steps: Vec<Box<Step>>,
    anteceding_steps: Vec<Box<Step>>,
    on: bool,
}

impl Step {
    fn add_preceding_step(&mut self, other_step: Box<Step>) {
        self.preceding_steps.push(other_step)
    }

    fn add_anteceding_step(&mut self, other_step: Box<Step>) {
        self.anteceding_steps.push(other_step)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"Step (\w+) must be finished before step (\w+) can begin.").unwrap();
    let steps = input
    .lines()
    .fold(
        HashMap::new(),
        |mut all_steps: HashMap<&str, Step>, line| {
            let (_, [first_step_name, second_step_name]) = re.captures(line).unwrap().extract();
            let first_step = *all_steps
                        .entry(first_step_name)
                        .or_insert(Step{name: first_step_name.to_string(), preceding_steps: Vec::new(), anteceding_steps: Vec::new(), on: false});

            let mut second_step: Step = *all_steps
            .entry(second_step_name)
            .or_insert(Step{name: second_step_name.to_string(), preceding_steps: Vec::new(), anteceding_steps: Vec::new(), on: false});

            first_step.add_anteceding_step(Box::new(second_step));
            second_step.add_preceding_step(Box::new(first_step));

            all_steps
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
