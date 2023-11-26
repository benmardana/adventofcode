use std::collections::{HashMap, HashSet};
use std::iter::zip;

pub fn solve(input: String) -> Option<(i32, i32)> {
    let lines = input.split('\n');

    let part_one = {
        let lowercase = ('a'..='z').collect::<Vec<char>>();
        let uppercase = ('A'..='Z').collect::<Vec<char>>();
        let score_map = zip([lowercase, uppercase].concat(), 1..=52).collect::<HashMap<_, _>>();

        lines.clone().fold(0, |acc, curr| {
            let (first, last) = curr.split_at(curr.len() / 2);
            let first_map = first.chars().collect::<HashSet<_>>();
            let last_map = last.chars().collect::<HashSet<_>>();
            if let Some(diff) = (&first_map & &last_map).iter().next() {
                return acc + score_map.get(diff).unwrap();
            }
            acc
        })
    };

    let part_two = {
        let lowercase = ('a'..='z').collect::<Vec<char>>();
        let uppercase = ('A'..='Z').collect::<Vec<char>>();
        let score_map = zip([lowercase, uppercase].concat(), 1..=52).collect::<HashMap<_, _>>();

        lines.collect::<Vec<_>>().chunks(3).fold(0, |acc, curr| {
            let (first, middle, last) = (curr[0], curr[1], curr[2]);
            let first_map = first.chars().collect::<HashSet<_>>();
            let middle_map = middle.chars().collect::<HashSet<_>>();
            let last_map = last.chars().collect::<HashSet<_>>();
            if let Some(diff) = (&(&first_map & &middle_map) & &last_map).iter().next() {
                return acc + score_map.get(diff).unwrap();
            }
            acc
        })
    };

    Some((part_one, part_two))
}
