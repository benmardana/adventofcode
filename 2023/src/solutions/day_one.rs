use std::collections::HashMap;

use regex::Regex;

use crate::input::Input;

use super::Solution;

pub struct DayOne {}

impl Solution for DayOne {
    fn part_one(&self, input: &Input) -> String {
        input
            .raw
            .lines()
            .fold(0, |acc, curr| {
                let chars = curr.as_bytes();
                let mut left: Option<u16> = None;
                let mut right: Option<u16> = None;
                for i in 0..curr.len() {
                    if left.is_none() && chars[i].is_ascii_digit() {
                        left = Some((chars[i] - 48).into());
                    }

                    if right.is_none() && chars[curr.len() - i - 1].is_ascii_digit() {
                        right = Some((chars[curr.len() - i - 1] - 48).into());
                    }

                    if left.is_some() && right.is_some() {
                        break;
                    }
                }

                let value = left.unwrap() * 10 + right.unwrap();
                acc + value
            })
            .to_string()
    }

    fn part_two(&self, input: &Input) -> String {
        let re_left = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();
        let re_right = Regex::new(r"\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();
        input
            .raw
            .lines()
            .fold(0, |acc, curr| {
                let left = re_left.find(curr).unwrap().as_str();
                let rev = curr.chars().rev().collect::<String>();
                let right = re_right.find(&rev).unwrap().as_str();
                let value = map(left) * 10 + map(right);
                acc + value
            })
            .to_string()
    }
}

fn map(word: &str) -> u16 {
    let arr = HashMap::from([
        ("1", 1),
        ("one", 1),
        ("eno", 1),
        ("2", 2),
        ("two", 2),
        ("owt", 2),
        ("3", 3),
        ("three", 3),
        ("eerht", 3),
        ("4", 4),
        ("four", 4),
        ("ruof", 4),
        ("5", 5),
        ("five", 5),
        ("evif", 5),
        ("6", 6),
        ("six", 6),
        ("xis", 6),
        ("7", 7),
        ("seven", 7),
        ("neves", 7),
        ("8", 8),
        ("eight", 8),
        ("thgie", 8),
        ("9", 9),
        ("nine", 9),
        ("enin", 9),
    ]);
    *arr.get(word).unwrap()
}
