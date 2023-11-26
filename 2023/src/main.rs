mod input;

use std::collections::HashMap;

use input::{get_input, Input};

mod args;
use args::{Args, Part};

fn day_one(input: Input, part: Option<Part>) {
    dbg!(input, part);
}

fn get_day(day_to_run: u8) -> Option<fn(Input, Option<Part>)> {
    let map = HashMap::from([
        (1, Some(day_one)),
        (2, None),
        (3, None),
        (4, None),
        (5, None),
        (6, None),
        (7, None),
        (8, None),
        (9, None),
        (10, None),
        (11, None),
        (12, None),
        (13, None),
        (14, None),
        (15, None),
        (16, None),
        (17, None),
        (18, None),
        (19, None),
        (20, None),
        (21, None),
        (22, None),
        (23, None),
        (24, None),
    ]);

    map.get(day_to_run)
}

fn main() {
    let Args { day_to_run, part } = argh::from_env();
    let input = get_input(day_to_run).unwrap();

    let func = get_day(day_to_run);

    func(input, part);
}
