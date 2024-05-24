
use std::{cell::RefCell, collections::HashMap};

use regex::Regex;
advent_of_code::solution!(7);

#[derive(Clone)]
struct Step {
    name: String,
    preceding_steps: Vec<RefCell<Step>>,
    anteceding_steps: Vec<RefCell<Step>>,
    on: bool,
}

impl Step {
    fn add_preceding_step(&mut self, other_step: RefCell<Step>) {
        self.preceding_steps.push(other_step)
    }

    fn add_anteceding_step(&mut self, other_step: RefCell<Step>) {
        self.anteceding_steps.push(other_step)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"Step (\w+) must be finished before step (\w+) can begin.").unwrap();
    let steps = input
    .lines()
    .fold(
        HashMap::new(),
        |mut all_steps: HashMap<&str, RefCell<Step>>, line| {
            let (_, [first_step_name, second_step_name]) = re.captures(line).unwrap().extract();

            let first_step_opt = all_steps.get(first_step_name);
            let first_step;
            if first_step_opt.is_none() {
                first_step = RefCell::new(Step{
                    name: first_step_name.to_string(),
                    preceding_steps: Vec::new(),
                    anteceding_steps: Vec::new(),
                    on: false,
                });
                all_steps.insert(first_step_name, first_step.clone());
            } else {
                first_step = first_step_opt.unwrap().to_owned();
            }

            let second_step_opt = all_steps.get(second_step_name);
            let second_step;
            if second_step_opt.is_none() {
                second_step = RefCell::new(Step{
                    name: second_step_name.to_string(),
                    preceding_steps: Vec::new(),
                    anteceding_steps: Vec::new(),
                    on: false,
                });
                all_steps.insert(second_step_name, second_step.clone());
            } else {
                second_step = second_step_opt.unwrap().to_owned();
            }

            first_step.borrow_mut().add_anteceding_step(second_step.clone());
            second_step.borrow_mut().add_preceding_step(first_step.clone());

            all_steps
        }
    );

    let mut letters: Vec<String> = Vec::new();
    let mut next_steps: Vec<RefCell<Step>> = Vec::new();

    for step in steps.values() {
        if step.borrow().preceding_steps.len() == 0 {
            next_steps.push(step.clone())
        }
    }

    while next_steps.len() > 0 {
        let mut unsorted_letters:  Vec<String> = Vec::new();

        let mut potential_next_steps: Vec<RefCell<Step>> = Vec::new();
        for step in next_steps {
            unsorted_letters.push(step.borrow().name.clone());
            step.borrow_mut().on = true;

            for anteceding_step in step.borrow().anteceding_steps {
                potential_next_steps.push(anteceding_step.clone())

            }
        }

        unsorted_letters.sort();
        letters.extend(unsorted_letters);

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
