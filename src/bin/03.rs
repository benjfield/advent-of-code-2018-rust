use regex::Regex;
use std::collections::HashMap;
use std::cmp;
advent_of_code::solution!(3);

struct Claim {
    claim_number: u32,
    start_x: u32,
    x_length: u32,
    start_y: u32,
    y_length: u32
}

pub struct Rectangle {
    start_x: u32,
    finish_x: u32,
    start_y: u32,
    finish_y: u32
}

impl Rectangle {
    fn size(self) -> u32 {
        (self.finish_x - self.start_x - 1) * (self.finish_x - self.start_x - 1)
    }

    // Assume rectangle to remove is entirely inside.
    fn remainders_after_rectangle_removal(self, rectangle_to_remove: Rectangle) -> Vec<Rectangle>{
        let mut remainder_rectangles = Vec::new();

        let remainder_finish_y = cmp::min(rectangle_to_remove.start_y - 1, self.finish_y);
        if remainder_finish_y >= self.start_y {
            remainder_rectangles.push(
                Rectangle{
                    start_x: self.start_x,
                    finish_x: self.finish_x,
                    start_y: self.start_x,
                    finish_y: remainder_finish_y
                }
            );
        }

        let remainder_finish_x = cmp::min(rectangle_to_remove.start_x - 1, self.finish_x);
        if remainder_finish_x >= self.start_x {
            remainder_rectangles.push(
                Rectangle{
                    start_x: self.start_x,
                    finish_x: remainder_finish_x,
                    start_y: rectangle_to_remove.start_y,
                    finish_y: rectangle_to_remove.finish_y
                }
            );
        }

        let remainder_start_x = cmp::max(rectangle_to_remove.finish_x + 1, self.start_x);
        if remainder_start_x <= self.finish_x {
            remainder_rectangles.push(
                Rectangle{
                    start_x: remainder_start_x,
                    finish_x: self.finish_x,
                    start_y: rectangle_to_remove.start_y,
                    finish_y: rectangle_to_remove.finish_y
                }
            );
        }

        let remainder_start_y = cmp::max(rectangle_to_remove.finish_y + 1, self.start_y);
        if remainder_start_y <= self.finish_y {
            remainder_rectangles.push(
                Rectangle{
                    start_x: self.start_x,
                    finish_x: self.finish_x,
                    start_y: remainder_start_y,
                    finish_y: self.finish_y
                }
            );
        }

        remainder_rectangles
    }
}


pub fn get_insersection_rectangle(rectangle_1: Rectangle, rectangle_2: Rectangle) -> Option<Rectangle> {
    let intersect_start_x = cmp::max(rectangle_1.start_x, rectangle_2.start_x);
    let intersect_finish_x = cmp::min(rectangle_1.finish_x, rectangle_2.finish_x);
    if intersect_finish_x < intersect_start_x {
        return None
    }
    let intersect_start_y = cmp::max(rectangle_1.start_y, rectangle_2.start_y);
    let intersect_finish_y = cmp::min(rectangle_1.finish_y, rectangle_2.finish_y);
    if intersect_finish_y < intersect_start_y {
        return None
    }
    Some(
        Rectangle{
            start_x: intersect_start_x,
            finish_x: intersect_finish_x,
            start_y: intersect_start_y,
            finish_y: intersect_finish_y,
        }
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    let coord_counts = input
    .lines()
    .map(
        |line_str| {
            let (_full, [
                claim_number,
                start_x,
                start_y,
                x_length,
                y_length
                ]
            ) = re.captures(line_str).unwrap().extract();

            Claim{
                claim_number: claim_number.parse::<u32>().unwrap(),
                start_x: start_x.parse::<u32>().unwrap(),
                x_length: x_length.parse::<u32>().unwrap(),
                start_y: start_y.parse::<u32>().unwrap(),
                y_length: y_length.parse::<u32>().unwrap(),
            }
        }
    ).fold(
        HashMap::new(),
        |mut count_hash: HashMap<(u32, u32), u32>, claim| {
            for x in claim.start_x..claim.start_x + claim.x_length {
                for y in claim.start_y..claim.start_y + claim.y_length {
                    let coord = (x, y);
                    count_hash
                    .entry(coord)
                    .and_modify(|count| { *count += 1 })
                    .or_insert( 1);
                }
            }
            count_hash
        }
    );

    let more_than_2_count = coord_counts
    .values()
    .fold(
        0,
        |total: u32, claim_count| {
            if *claim_count > 1 {
                total + 1
            } else {
                total
            }
        }
    );

    Some(more_than_2_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    let claims = input
    .lines()
    .map(
        |line_str| {
            let (_full, [
                claim_number,
                start_x,
                start_y,
                x_length,
                y_length
                ]
            ) = re.captures(line_str).unwrap().extract();

            Claim{
                claim_number: claim_number.parse::<u32>().unwrap(),
                start_x: start_x.parse::<u32>().unwrap(),
                x_length: x_length.parse::<u32>().unwrap(),
                start_y: start_y.parse::<u32>().unwrap(),
                y_length: y_length.parse::<u32>().unwrap(),
            }
        }
    );

    let claim_count = claims
        .clone()
        .fold(
        HashMap::new(),
        |mut count_hash: HashMap<(u32, u32), u32>, claim| {
            for x in claim.start_x..claim.start_x + claim.x_length {
                for y in claim.start_y..claim.start_y + claim.y_length {
                    let coord = (x, y);
                    count_hash
                    .entry(coord)
                    .and_modify(|count| { *count += 1 })
                    .or_insert( 1);
                }
            }
            count_hash
        }
    );

    let mut answer = 0;
    for claim in claims {
        let mut overlap = false;
        for x in claim.start_x..claim.start_x + claim.x_length {
            for y in claim.start_y..claim.start_y + claim.y_length {
                let coord = (x, y);
                if claim_count[&coord] > 1 {
                    overlap = true;

                }
            }
        }

        if !overlap {
            answer = claim.claim_number;
            break
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
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }
}
