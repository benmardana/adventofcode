mod args;
mod input;
mod solutions;

use args::{Args, Part};
use input::get_input;
use solutions::{get_solution, get_solutions};

fn main() {
    let Args { day_to_run, part } = argh::from_env();
    if let Some(day_to_run) = day_to_run {
        let input = get_input(day_to_run).unwrap();

        let solution = get_solution(day_to_run)
            .unwrap_or_else(|| panic!("No solution for day {}", day_to_run));

        match part {
            Some(Part::One) => println!("part one: {}", solution.part_one(&input)),
            Some(Part::Two) => println!("part two: {}", solution.part_two(&input)),
            None => {
                println!("part one: {}", solution.part_one(&input));
                println!("part two: {}", solution.part_two(&input));
            }
        }
    } else {
        get_solutions()
            .iter()
            .enumerate()
            .for_each(|(i, solution)| {
                let input = get_input(i + 1).unwrap();
                println!("Day {}", i + 1);
                println!("  part one: {}", solution.part_one(&input));
                println!("  part two: {}\n", solution.part_two(&input));
            })
    }
}
