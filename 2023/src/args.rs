use std::{
    fmt::{self, Display},
    str::FromStr,
};

use anyhow::Result;
use argh::FromArgs;

#[derive(Debug, PartialEq)]
pub enum Part {
    One,
    Two,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParsePartError;

impl Display for ParsePartError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "possible values [1, 2]")
    }
}

impl FromStr for Part {
    type Err = ParsePartError;

    fn from_str(s: &str) -> Result<Part, ParsePartError> {
        match s {
            "1" => Ok(Part::One),
            "2" => Ok(Part::Two),
            _ => Err(ParsePartError),
        }
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Part::One => write!(f, "1"),
            Part::Two => write!(f, "2"),
        }
    }
}

#[derive(FromArgs, PartialEq, Debug)]
/// Advent of Code
pub struct Args {
    #[argh(positional)]
    pub day_to_run: u8,

    /// which part of a solution to run. possible values: [1, 2]
    #[argh(option, short = 'p')]
    pub part: Option<Part>,
}
