use regex::Regex;

use super::Solution;

pub struct DayTwo {}

impl Solution for DayTwo {
    fn part_one(&self, input: &crate::input::Input) -> String {
        let red_regex = Regex::new(r"(\d+) red").unwrap();
        let green_regex = Regex::new(r"(\d+) green").unwrap();
        let blue_regex = Regex::new(r"(\d+) blue").unwrap();
        input.raw.lines().fold(0, |acc, curr| {
            let (game_id, sets) = curr.split_once(": ").unwrap();
            let (_, _id) = game_id.split_once(' ').unwrap();
            let id: usize = _id.parse().unwrap();

            let impossible = sets.split("; ").any(|set| {
                let red = red_regex.captures(set);
                let green = green_regex.captures(set);
                let blue = blue_regex.captures(set);

                if let Some(red_capture) = red {
                    if red_capture[1].parse::<usize>().unwrap() > 12 {
                        return true
                    }
                }
                if let Some(green_capture) = green {
                    if green_capture[1].parse::<usize>().unwrap() > 13 {
                        return true
                    }
                }
                if let Some(blue_capture) = blue {
                    if blue_capture[1].parse::<usize>().unwrap() > 14 {
                        return true
                    }
                }
                false
            });

            if impossible {
                return acc
            }

            acc + id
        }).to_string()
    }

    fn part_two(&self, input: &crate::input::Input) -> String {
        let red_regex = Regex::new(r"(\d+) red").unwrap();
        let green_regex = Regex::new(r"(\d+) green").unwrap();
        let blue_regex = Regex::new(r"(\d+) blue").unwrap();
        input.raw.lines().fold(0, |acc, curr| {
            let (_, sets) = curr.split_once(": ").unwrap();

            let (max_red, max_green, max_blue) = sets.split("; ").fold((0,0,0), |(max_red, max_green, max_blue), curr| {
                let red = red_regex.captures(curr).map_or(max_red, |cap| {
                    let val = cap[1].parse::<usize>().unwrap();
                    if val > max_red {
                        return val;
                    }
                    max_red
                });

                let green = green_regex.captures(curr).map_or(max_green, |cap| {
                    let val = cap[1].parse::<usize>().unwrap();
                    if val > max_green {
                        return val;
                    }
                    max_green
                });

                let blue = blue_regex.captures(curr).map_or(max_blue, |cap| {
                    let val = cap[1].parse::<usize>().unwrap();
                    if val > max_blue {
                        return val;
                    }
                    max_blue
                });
                (red, green, blue)
            });

            acc + (max_red * max_green * max_blue)
        }).to_string()
    }
}
