use anyhow::Result;
use lexopt::Arg::{Long, Short, Value};
use lexopt::{Parser, ValueExt};

pub struct Args {
    pub day: u8,
}

pub fn parse_args() -> Result<Args> {
    let mut day: Option<u8> = None;

    let mut parser = Parser::from_env();
    while let Some(arg) = parser.next()? {
        match arg {
            Value(val) => {
                day = val.parse().ok();
            }
            Long("help") | Short('h') => {
                println!("Usage: adventofcode DAY");
                std::process::exit(0);
            }
            _ => return Err(arg.unexpected().into()),
        }
    }

    Ok(Args { day: day.unwrap() })
}
