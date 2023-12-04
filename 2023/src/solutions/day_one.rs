use regex::RegexSet;

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
        let re = RegexSet::new(&[
            r"\d",
            r"zero",
            r"one",
            r"two",
            r"three",
            r"four",
            r"five",
            r"six",
            r"seven",
            r"eight",
            r"nine"
        ]).unwrap();
        input.raw.lines().fold(0, |acc, curr| 
            {   
                let mut it = re.matches(curr).into_iter();
                let left = it.next();
                let right = it.last();
                dbg!(left, right);
                acc
            }
        ).to_string()
    }
}
