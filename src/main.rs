use std::{
    fs::{read_to_string, File},
    io::Write, fmt::Debug,
};

use anyhow::Result;
use args::parse_args;
use solutions::{day_one, day_two, day_three, day_four, day_five, day_six, day_seven, day_eight};


mod args;
mod solutions;

fn main() -> Result<()> {
    let day = parse_args()?.day;

    let solution: Box<dyn Debug> = match day {
        Some(1) => Box::new(day_one::solve(get_input(1)?.trim().to_string())),
        Some(2) => Box::new(day_two::solve(get_input(2)?.trim().to_string())),
        Some(3) => Box::new(day_three::solve(get_input(3)?.trim().to_string())),
        Some(4) => Box::new(day_four::solve(get_input(4)?.trim().to_string())),
        Some(5) => Box::new(day_five::solve(get_input(5)?.trim().to_string())),
        Some(6) => Box::new(day_six::solve(get_input(6)?.trim().to_string())),
        Some(7) => Box::new(day_seven::solve(get_input(7)?.trim().to_string())),
        Some(8) => Box::new(day_eight::solve(get_input(8)?.trim().to_string())),
        _ => {
            day_one::solve(get_input(1)?.trim().to_string());
            day_two::solve(get_input(2)?.trim().to_string());
            day_three::solve(get_input(3)?.trim().to_string());
            day_four::solve(get_input(4)?.trim().to_string());
            day_five::solve(get_input(5)?.trim().to_string());
            day_six::solve(get_input(6)?.trim().to_string());
            day_seven::solve(get_input(7)?.trim().to_string());
            Box::new(())
        },
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
