use regex::Regex;
use std::{borrow::BorrowMut, collections::HashSet};
advent_of_code::solution!(10);

fn generate_lights(input: &str) -> Vec<(i32, i32, i32, i32)> {
    let re = Regex::new(r"position=<\s*([\-0-9]+),\s*([\-0-9]+)> velocity=<\s*([\-0-9]+),\s*([\-0-9]+)>").unwrap();

    input
    .lines()
    .fold(
        Vec::new(),
        |mut lights, line| {
            let (_, [x_str, y_str, x_vel_str, y_vel_str]) = re.captures(line).unwrap().extract();
            let x = x_str.parse::<i32>().unwrap();
            let y = y_str.parse::<i32>().unwrap();
            let x_vel = x_vel_str.parse::<i32>().unwrap();
            let y_vel = y_vel_str.parse::<i32>().unwrap();
            lights.push((x, y, x_vel, y_vel));
            lights
        }
    )
}

fn tick(lights: &mut Vec<(i32, i32, i32, i32)>) {
    for light in lights.iter_mut() {
        light.0 += light.2;
        light.1 += light.3;
    }
}

fn generate_bounds(lights: &Vec<(i32, i32, i32, i32)>) -> (i32, i32, i32, i32) {
    lights
    .iter()
    .fold(
        (i32::MAX, i32::MIN, i32::MAX, i32::MIN),
        |(mut min_x, mut max_x, mut min_y, mut max_y), (x, y, _, _)| {
            if x < &min_x {
                min_x = *x;
            }

            if x > &max_x {
                max_x = *x;
            }

            if y < &min_y {
                min_y = *y;
            }

            if y > &max_y {
                max_y = *y;
            }

            (min_x, max_x, min_y, max_y)
        }
    )
}

fn generate_size_bounds_and_set(lights: &Vec<(i32, i32, i32, i32)>) -> (i64, (i32, i32, i32, i32), HashSet<(i32, i32)>) {
    let bounds = generate_bounds(lights);

    let lights_set: HashSet<(i32, i32)> = lights
    .iter()
    .map(
        |light| { (light.0, light.1)}
    ).collect();

    let x_size = bounds.1 - bounds.0 + 1;
    let y_size = bounds.3 - bounds.2 + 1;

    let size = (x_size as i64) * (y_size as i64);
    (size, bounds, lights_set)
}

fn print_strings(bounds: &(i32, i32, i32, i32), lights_set: &HashSet<(i32, i32)>) {
    for y in bounds.2..=bounds.3 {
        let mut row_string = "".to_string();
        for x in bounds.0..=bounds.1 {
            if lights_set.contains(&(x, y)) {
                row_string.push_str("#");
            } else {
                row_string.push_str(".");
            }
        }
        println!("{}", row_string);
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lights = generate_lights(input);

    let (mut last_size, mut last_bounds, mut last_light_set) = generate_size_bounds_and_set(&lights);

    let mut i = 0;
    loop {
        tick(lights.borrow_mut());

        let (size, bounds, light_set) = generate_size_bounds_and_set(&lights);

        if size > last_size {
            print_strings(&last_bounds, &last_light_set);
            break
        } else {
            last_size = size;
            last_bounds = bounds;
            last_light_set = light_set;
            i += 1
        }
    }
    Some(i)
}

pub fn part_two(input: &str) -> Option<u32> {
    part_one(input)
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
        assert_eq!(result, Some(3));
    }
}
