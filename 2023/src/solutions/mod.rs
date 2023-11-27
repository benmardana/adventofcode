mod day_one;
mod day_two;

use crate::input::Input;

use self::day_one::DayOne;
use self::day_two::DayTwo;

pub trait Solution {
    fn part_one(&self, input: &Input) -> String;
    fn part_two(&self, input: &Input) -> String;
}

const SOLUTIONS: [&dyn Solution; 2] = [&DayOne {}, &DayTwo {}];


pub fn get_solution(day: usize) -> Option<&'static &'static dyn Solution> {
    SOLUTIONS.get(day - 1)
}
