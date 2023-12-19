use std::collections::HashSet;

use crate::input::Input;

use super::Solution;

pub struct DayFour {}

impl Solution for DayFour {
    fn part_one(&self, input: &Input) -> String {
        input
            .raw
            .lines()
            .fold(0, |acc, curr| {
                let (winners, numbers) = curr
                    .split_once(" | ")
                    .map(|(winners, numbers)| {
                        let winners: HashSet<&str> = HashSet::from_iter(
                            winners.split_once(": ").unwrap().1.split_whitespace(),
                        );
                        let numbers: HashSet<&str> = HashSet::from_iter(numbers.split_whitespace());
                        (winners, numbers)
                    })
                    .unwrap();
                let intersection: HashSet<_> = winners.intersection(&numbers).collect();
                let score = match intersection.len() {
                    0 => 0,
                    1 => 1,
                    n => 2_u32.pow((n - 1).try_into().unwrap()),
                };
                acc + score
            })
            .to_string()
    }

    fn part_two(&self, input: &Input) -> String {
        let mut winners = 0;

        let mut input: Vec<(usize, usize)> = input
            .raw
            .lines()
            .map(|line| {
                let (winners, numbers) = line
                    .split_once(" | ")
                    .map(|(winners, numbers)| {
                        let winners: HashSet<&str> = HashSet::from_iter(
                            winners.split_once(": ").unwrap().1.split_whitespace(),
                        );
                        let numbers: HashSet<&str> = HashSet::from_iter(numbers.split_whitespace());
                        (winners, numbers)
                    })
                    .unwrap();
                let intersection: HashSet<_> = winners.intersection(&numbers).collect();
                (1, intersection.len())
            })
            .collect();

        for i in 0..input.len() {
            winners += input[i].0;
            for j in 1..=input[i].1 {
                input[i + j].0 = input[i + j].0 + 1 * input[i].0
            }
        }

        winners.to_string()
    }
}
