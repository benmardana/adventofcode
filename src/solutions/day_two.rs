use std::str::FromStr;

use strum::EnumString;

const ROCK: isize = 1;
const PAPER: isize = 2;
const SCISSORS: isize = 3;
const WIN: isize = 6;
const LOSS: isize = 0;
const DRAW: isize = 3;

#[derive(EnumString)]
enum Score {
    AX = ROCK + DRAW,
    AY = PAPER + WIN,
    AZ = SCISSORS + LOSS,

    BX = ROCK + LOSS,
    BY = PAPER + DRAW,
    BZ = SCISSORS + WIN,

    CX = ROCK + WIN,
    CY = PAPER + LOSS,
    CZ = SCISSORS + DRAW,
}

#[derive(EnumString)]
enum DecryptedScore {
    AX = SCISSORS + LOSS,
    AY = ROCK + DRAW,
    AZ = PAPER + WIN,

    BX = ROCK + LOSS,
    BY = PAPER + DRAW,
    BZ = SCISSORS + WIN,

    CX = PAPER + LOSS,
    CY = SCISSORS + DRAW,
    CZ = ROCK + WIN,
}

pub fn solve(input: String) -> Option<(i32, i32)> {
    let lines = input.split('\n');
    let part_one = lines
        .clone()
        .map(|line| Score::from_str(&line.replace(' ', "")).unwrap())
        .fold(0, |acc, curr| acc + (curr as i32));

    let part_two = lines
        .map(|line| DecryptedScore::from_str(&line.replace(' ', "")).unwrap())
        .fold(0, |acc, curr| acc + (curr as i32));

    Some((part_one, part_two))
}
