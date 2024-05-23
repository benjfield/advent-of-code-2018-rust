use std::collections::HashMap;
use regex::Regex;

advent_of_code::solution!(4);

struct Guard {
    total_minutes: u32,
    sleep_minutes: HashMap<u32, u32>
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut log: Vec<&str> = input
    .lines()
    .collect();

    log.sort();

    let re = Regex::new(r"\[.*:(\d+)\] (.*)").unwrap();
    let guard_re = Regex::new(r"Guard #(\d+) begins shift").unwrap();

    let mut guard_times: HashMap<u32, Guard> = HashMap::new();

    let mut prior_minute = 0;
    let mut guard_number = 0;
    for line in log.into_iter() {
        let (_, [minute_str, log_text]) = re.captures(line).unwrap().extract();
        let minute = minute_str.parse::<u32>().unwrap();

        if log_text == "wakes up" {
            let guard =
            guard_times
            .entry(guard_number)
            .or_insert(Guard { total_minutes: 0, sleep_minutes: HashMap::new() });

            for i in prior_minute..minute {
                guard
                .sleep_minutes
                .entry(i)
                .and_modify(|count| { *count += 1 })
                .or_insert( 1);
            }

            guard.total_minutes += minute - prior_minute
        } else if log_text == "falls asleep" {
            prior_minute = minute;
        } else {
            let (_, [guard_number_str]) = guard_re.captures(log_text).unwrap().extract();
            guard_number = guard_number_str.parse::<u32>().unwrap();
        }
    }

    let mut worst_guard_number = 0;
    let mut most_sleep = 0;

    for (guard_number, guard ) in guard_times.iter() {
        if guard.total_minutes > most_sleep {
            most_sleep = guard.total_minutes;
            worst_guard_number = *guard_number;
        }
    }

    let mut worst_minute = 0;
    let mut worst_minute_count = 0;
    let worst_guard = &guard_times[&worst_guard_number];

    for (minute, minute_count) in worst_guard.sleep_minutes.iter() {
        if *minute_count > worst_minute_count {
            worst_minute_count = *minute_count;
            worst_minute = *minute;
        }
    }

    Some(worst_minute * worst_guard_number)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut log: Vec<&str> = input
    .lines()
    .collect();

    log.sort();

    let re = Regex::new(r"\[.*:(\d+)\] (.*)").unwrap();
    let guard_re = Regex::new(r"Guard #(\d+) begins shift").unwrap();

    let mut guard_times: HashMap<u32, Guard> = HashMap::new();

    let mut prior_minute = 0;
    let mut guard_number = 0;
    for line in log.into_iter() {
        let (_, [minute_str, log_text]) = re.captures(line).unwrap().extract();
        let minute = minute_str.parse::<u32>().unwrap();

        if log_text == "wakes up" {
            let guard =
            guard_times
            .entry(guard_number)
            .or_insert(Guard { total_minutes: 0, sleep_minutes: HashMap::new() });

            for i in prior_minute..minute {
                guard
                .sleep_minutes
                .entry(i)
                .and_modify(|count| { *count += 1 })
                .or_insert( 1);
            }

            guard.total_minutes += minute - prior_minute
        } else if log_text == "falls asleep" {
            prior_minute = minute;
        } else {
            let (_, [guard_number_str]) = guard_re.captures(log_text).unwrap().extract();
            guard_number = guard_number_str.parse::<u32>().unwrap();
        }
    }

    let mut worst_guard_number = 0;
    let mut worst_sleep_minute = 0;
    let mut most_sleep = 0;

    for (guard_number, guard ) in guard_times.iter() {
        for (minute, minute_count) in guard.sleep_minutes.iter() {
            if *minute_count > most_sleep {
                most_sleep = *minute_count;
                worst_guard_number = *guard_number;
                worst_sleep_minute = *minute;
            }
        }
    }

    Some(worst_sleep_minute * worst_guard_number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(240));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4455));
    }
}
