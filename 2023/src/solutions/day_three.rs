use std::cmp::min;

use super::Solution;

pub struct DayThree {}

impl Solution for DayThree {
    fn part_one(&self, input: &crate::input::Input) -> String {
        let num_re = regex::Regex::new(r"\d+").unwrap();
        let symbol_re = regex::Regex::new(r"[^0-9\.]").unwrap();
        let mut sum: u64 = 0;
        input.raw.lines().enumerate().for_each(|(i, line)| {
            let nums = num_re.find_iter(line);
            for num in nums {
                let above = 'above: {
                    if i == 0 {
                        break 'above "".to_owned();
                    }
                    dbg!(input.raw.lines().nth(i - 1).unwrap_or("").get(num.start().saturating_sub(1)..(min(num.end() + 1, line.len() + 1))));
                    input.raw.lines().nth(i - 1).unwrap_or("").get(num.start().saturating_sub(1)..(min(num.end() + 1, line.len()))).unwrap_or("").to_string()
                };
                let middle = format!("{}{}", line.get(num.start().saturating_sub(1)..num.start()).unwrap_or(""), line.get(num.end()..(min(num.end() + 1, line.len()))).unwrap_or(""));
                let below = 'below: {
                    if i == input.raw.len() + 1 {
                        break 'below "".to_owned();
                    }
                    input.raw.lines().nth(i + 1).unwrap_or("").get(num.start().saturating_sub(1)..(min(num.end() + 1, line.len()))).unwrap_or("").to_string()
                };
                let context = above + &middle + &below;
                dbg!(num, &context);
                if symbol_re.is_match(&context) {
                    sum = sum + num.as_str().parse::<u64>().unwrap()
                }
            }
        });
        sum.to_string()
    }

    fn part_two(&self, input: &crate::input::Input) -> String {
        let gear_re = regex::Regex::new(r"\*").unwrap();
        let symbol_re = regex::Regex::new(r"[^0-9\.]").unwrap();
        let mut sum: u64 = 0;
        input.raw.lines().enumerate().for_each(|(i, line)| {
            let gears = gear_re.find_iter(line);
            for gear in gears {
                let above = 'above: {
                    if i == 0 {
                        break 'above "".to_owned();
                    }
                    dbg!(input.raw.lines().nth(i - 1).unwrap_or("").get(num.start().saturating_sub(1)..(min(num.end() + 1, line.len() + 1))));
                    input.raw.lines().nth(i - 1).unwrap_or("").get(num.start().saturating_sub(1)..(min(num.end() + 1, line.len()))).unwrap_or("").to_string()
                };
                let middle = format!("{}{}", line.get(num.start().saturating_sub(1)..num.start()).unwrap_or(""), line.get(num.end()..(min(num.end() + 1, line.len()))).unwrap_or(""));
                let below = 'below: {
                    if i == input.raw.len() + 1 {
                        break 'below "".to_owned();
                    }
                    input.raw.lines().nth(i + 1).unwrap_or("").get(num.start().saturating_sub(1)..(min(num.end() + 1, line.len()))).unwrap_or("").to_string()
                };
                let context = above + &middle + &below;
                dbg!(num, &context);
                if symbol_re.is_match(&context) {
                    sum = sum + num.as_str().parse::<u64>().unwrap()
                }
            }
        });
        sum.to_string()
    }
}

