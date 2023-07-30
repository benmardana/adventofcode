use std::{
    fs::{read_to_string, File},
    io::Write, fmt::Debug,
};

use anyhow::Result;
use args::parse_args;
use solutions::{day_one, day_two, day_three, day_four, day_five, day_six, day_seven};


mod args;
mod solutions;

fn main() -> Result<()> {
    let day = parse_args()?.day;

    let input = get_input(day)?.trim().to_string();

    let solution: Box<dyn Debug> = match day {
        1 => Box::new(day_one::solve(input)),
        2 => Box::new(day_two::solve(input)),
        3 => Box::new(day_three::solve(input)),
        4 => Box::new(day_four::solve(input)),
        5 => Box::new(day_five::solve(input)),
        6 => Box::new(day_six::solve(input)),
        7 => Box::new(day_seven::solve(input)),
        _ => Box::new(()),
    };

    println!("{:?}", solution);

    Ok(())
}



fn get_input(day: u8) -> Result<String> {
    let file_destination = format!("cache/{}.txt", day);

    let cached_input = read_to_string(&file_destination);

    cached_input.or_else(|_| {
        let body: String =
            ureq::get(format!("https://adventofcode.com/2022/day/{}/input", day).as_str())
            .set("cookie", "session=53616c7465645f5fe4ea032659aba11a07bd14531283055324d43d95c938a4e588e41554869614e509d9129a962a5b255a21ef0ddec2bfa3d8a3f6d4449c3523")
            .call()?
            .into_string()?;
        
        File::create(&file_destination)?.write_all(body.as_bytes())?;

        Ok(body)
    })
}
