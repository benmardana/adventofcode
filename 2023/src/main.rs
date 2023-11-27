mod input;
mod solutions;

use input::get_input;
use solutions::get_solution;

mod args;
use args::{Args, Part};

fn main() {
    let Args { day_to_run, part } = argh::from_env();
    let input = get_input(day_to_run).unwrap();

    let solution = get_solution(day_to_run).unwrap();

    if let Some(part) = part {
        if part == Part::One {
            dbg!(solution.part_one(&input));
            return;
        }
        if part == Part::One {
            dbg!(solution.part_two(&input));
            return;
        }
    } else {
        dbg!(solution.part_one(&input));
        dbg!(solution.part_two(&input));
        return;
    }
}
