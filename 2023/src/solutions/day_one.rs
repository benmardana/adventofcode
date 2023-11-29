use crate::input::Input;

use super::Solution;

pub struct DayOne {}

impl Solution for DayOne {
    fn part_one(&self, input: &Input) -> String {
        input
            .raw
            .split("\n\n")
            .map(|group| {
                group
                    .split_whitespace()
                    .fold(0u64, |acc, curr| acc + curr.parse::<u64>().unwrap())
            })
            .max()
            .unwrap()
            .to_string()
    }

    fn part_two(&self, input: &Input) -> String {
        let mut calories = input
            .raw
            .split("\n\n")
            .map(|group| {
                group
                    .split_whitespace()
                    .fold(0u64, |acc, curr| acc + curr.parse::<u64>().unwrap())
            })
            .collect::<Vec<_>>();

        calories.sort();
        calories.reverse();

        calories
            .iter()
            .take(3)
            .fold(0, |acc, curr| acc + curr)
            .to_string()
    }
}
