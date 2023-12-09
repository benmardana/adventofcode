mod day_one;
mod day_two;
mod day_three;

use crate::input::Input;

use self::day_one::DayOne;
use self::day_two::DayTwo;
use self::day_three::DayThree;

pub trait Solution {
    fn part_one(&self, _input: &Input) -> String {
        String::from("todo")
    }
    fn part_two(&self, _input: &Input) -> String {
        String::from("todo")
    }
}

const SOLUTIONS: [&dyn Solution; 3] = [&DayOne {}, &DayTwo {}, &DayThree {}];


pub fn get_solution(day: usize) -> Option<&'static &'static dyn Solution> {
    SOLUTIONS.get(day - 1)
}
