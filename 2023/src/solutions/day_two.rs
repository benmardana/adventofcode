use super::Solution;

pub struct DayTwo {}

impl Solution for DayTwo {
    fn part_one(&self, input: &crate::input::Input) -> String {
        input.raw.lines().fold(0, |acc, curr| {
            let (left, right) = curr.split_once(": ").unwrap();
            let (_, _id) = left.split_once(' ').unwrap();
            let id: usize = _id.parse().unwrap();

            let balls = right.split("; ");

            acc
        })
    }
}
