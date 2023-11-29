mod input;
mod solutions;

use input::get_input;
use solutions::get_solution;

mod args;
use args::{Args, Part};

fn main() {
    let Args { day_to_run, part } = argh::from_env();
    let input = get_input(day_to_run).unwrap();

    let solution = get_solution(day_to_run).unwrap_or_else(|| panic!("No solution for day {}", day_to_run));

    match part {
        Some(Part::One) => println!("part one: {}", solution.part_one(&input)),
        Some(Part::Two) => println!("part two: {}", solution.part_two(&input)),
        None => {
            println!("part one: {}", solution.part_one(&input));
            println!("part two: {}", solution.part_two(&input));
        },
    } 
}
