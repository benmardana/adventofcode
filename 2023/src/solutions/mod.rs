mod day_four;
mod day_one;
mod day_three;
mod day_two;

use crate::input::Input;

use self::day_four::DayFour;
use self::day_one::DayOne;
use self::day_three::DayThree;
use self::day_two::DayTwo;

pub trait Solution {
    fn part_one(&self, _input: &Input) -> String {
        String::from("todo")
    }
    fn part_two(&self, _input: &Input) -> String {
        String::from("todo")
    }
}

const SOLUTIONS: [&dyn Solution; 4] = [&DayOne {}, &DayTwo {}, &DayThree {}, &DayFour {}];

pub fn get_solution(day: usize) -> Option<&'static &'static dyn Solution> {
    SOLUTIONS.get(day - 1)
}
