use std::{
    cmp::{max, min},
    collections::HashMap,
};

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
                    input
                        .raw
                        .lines()
                        .nth(i - 1)
                        .unwrap_or("")
                        .get(num.start().saturating_sub(1)..(min(num.end() + 1, line.len())))
                        .unwrap_or("")
                        .to_string()
                };
                let middle = format!(
                    "{}{}",
                    line.get(num.start().saturating_sub(1)..num.start())
                        .unwrap_or(""),
                    line.get(num.end()..(min(num.end() + 1, line.len())))
                        .unwrap_or("")
                );
                let below = 'below: {
                    if i == input.raw.len() + 1 {
                        break 'below "".to_owned();
                    }
                    input
                        .raw
                        .lines()
                        .nth(i + 1)
                        .unwrap_or("")
                        .get(num.start().saturating_sub(1)..(min(num.end() + 1, line.len())))
                        .unwrap_or("")
                        .to_string()
                };
                let context = above + &middle + &below;
                if symbol_re.is_match(&context) {
                    sum += num.as_str().parse::<u64>().unwrap()
                }
            }
        });
        sum.to_string()
    }

    fn part_two(&self, input: &crate::input::Input) -> String {
        let num_re = regex::Regex::new(r"\d+").unwrap();
        let symbol_re = regex::Regex::new(r"\*").unwrap();
        let mut map: HashMap<String, Vec<u64>> = HashMap::new();
        input.raw.lines().enumerate().for_each(|(i, line)| {
            let nums = num_re.find_iter(line);
            for num in nums {
                let above = 'above: {
                    if i == 0 {
                        break 'above "".to_owned();
                    }
                    input
                        .raw
                        .lines()
                        .nth(i - 1)
                        .unwrap_or("")
                        .get(num.start().saturating_sub(1)..(min(num.end() + 1, line.len())))
                        .unwrap_or("")
                        .to_string()
                };

                if let Some(m) = symbol_re.find(&above) {
                    let x = max(1, num.start()) + m.start() - 1;
                    let y = i - 1;
                    let key = format!("{x},{y}");
                    map.entry(key)
                        .or_default()
                        .push(num.as_str().parse::<u64>().unwrap());
                }

                let front = line
                    .get(num.start().saturating_sub(1)..num.start())
                    .unwrap_or("");
                if symbol_re.find(front).is_some() {
                    let x = num.start() - 1;
                    let y = i;
                    let key = format!("{x},{y}");
                    map.entry(key)
                        .or_default()
                        .push(num.as_str().parse::<u64>().unwrap());
                }

                let back = line
                    .get(num.end()..(min(num.end() + 1, line.len())))
                    .unwrap_or("");
                if symbol_re.find(back).is_some() {
                    let x = num.end();
                    let y = i;
                    let key = format!("{x},{y}");
                    map.entry(key)
                        .or_default()
                        .push(num.as_str().parse::<u64>().unwrap());
                }

                let below = 'below: {
                    if i == input.raw.len() + 1 {
                        break 'below "".to_owned();
                    }
                    input
                        .raw
                        .lines()
                        .nth(i + 1)
                        .unwrap_or("")
                        .get(num.start().saturating_sub(1)..(min(num.end() + 1, line.len())))
                        .unwrap_or("")
                        .to_string()
                };
                if let Some(m) = symbol_re.find(&below) {
                    let x = max(1, num.start()) + m.start() - 1;
                    let y = i + 1;
                    let key = format!("{x},{y}");
                    map.entry(key)
                        .or_default()
                        .push(num.as_str().parse::<u64>().unwrap());
                }
            }
        });
        map.iter()
            .fold(0, |acc, (_, value)| {
                if value.len() != 2 {
                    return acc;
                }
                acc + (value[0] * value[1])
            })
            .to_string()
    }
}
