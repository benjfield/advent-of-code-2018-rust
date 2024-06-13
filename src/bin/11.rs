use itertools::Itertools;
use std::collections::HashMap;
advent_of_code::solution!(11);

fn get_power_level((x, y): &(i32, i32), serial_number: &i32) -> i32 {
    let rack_id = x + 10;
    let mut power_level = (rack_id * y + serial_number) * &rack_id;

    power_level = (power_level % 1000) / 100;

    power_level -= 5;

    power_level

}

fn build_summed_area_table(serial_number: &i32) -> HashMap<(i32, i32), i32> {
    (1..=300).cartesian_product(1..=300)
    .fold(
        HashMap::new(),
        |mut summed_area_table, (x, y)| {
            let mut value = get_power_level(&(x, y), serial_number);

            if y > 1 {
                value += summed_area_table.get(&(x, y - 1)).unwrap()
            }

            if x > 1 {
                value += summed_area_table.get(&(x - 1, y)).unwrap()
            }

            if x > 1 && y > 1 {
                value -= summed_area_table.get(&(x - 1, y - 1)).unwrap()
            }

            summed_area_table.insert((x, y), value);

            summed_area_table
        }
    )
}

fn get_power_level_by_size(summed_area_table: &HashMap<(i32, i32), i32>, size: &i32) -> (i32, i32, i32) {
    let (power_level, (x, y)) = (1..=(301-size)).cartesian_product(1..=(301-size))
    .fold(
        (0, (0, 0)),
        |(max_power_level, coord), (x, y) | {
            let mut this_power_level = 0;

            let a = summed_area_table.get(&(x - 1, y - 1));
            if !a.is_none() {
                this_power_level += a.unwrap()
            }

            let d = summed_area_table.get(&(x + size - 1, y + size - 1));
            if !d.is_none() {
                this_power_level += d.unwrap()
            }

            let b = summed_area_table.get(&(x + size - 1, y - 1));
            if !b.is_none() {
                this_power_level -= b.unwrap()
            }

            let c = summed_area_table.get(&(x - 1, y + size - 1));
            if !c.is_none() {
                this_power_level -= c.unwrap()
            }

            if this_power_level > max_power_level {
                (this_power_level, (x, y))
            } else {
                (max_power_level, coord)
            }
        }
    );

    (power_level, x, y)

}

pub fn part_one(input: &str) -> Option<String> {
    let serial_number = input.trim().parse::<i32>().unwrap();

    let summed_area_table = build_summed_area_table(&serial_number);

    let (_, x,y) = get_power_level_by_size(
        &summed_area_table,
        &3
    );

    let answer = format!("{},{}", x.to_string(), y.to_string());
    Some(answer)
}

pub fn part_two(input: &str) -> Option<String> {
    let serial_number = input.trim().parse::<i32>().unwrap();

    let summed_area_table = build_summed_area_table(&serial_number);

    let mut max_power_level = 0;

    let mut x = 0;
    let mut y = 0;
    let mut size = 0;


    for this_size in (1..=300).rev() {
        if size * size * 4 < max_power_level {
            break;
        }

        let (power_level, this_x, this_y) = get_power_level_by_size(
            &summed_area_table,
            &this_size
        );

        if power_level > max_power_level {
            max_power_level = power_level;
            x = this_x;
            y = this_y;
            size = this_size;
        }
    }

    let answer = format!("{},{},{}", x.to_string(), y.to_string(), size.to_string());
    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("33,45".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("90,269,16".to_string()));
    }
}
