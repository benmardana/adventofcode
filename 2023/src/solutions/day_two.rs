use super::Solution;

pub struct DayTwo {}

fn score(p1: usize, p2: usize) -> usize {
    if (p1 + 1) % 3 == p2 {
        // p2 wins
        return 0;
    }
    if p1 == p2 {
        // draw
        return 3;
    }
    // p1 wins
    return 6;
}

fn calc_move(p1: usize, outcome: usize) -> usize {
    if outcome == 1 {
        // draw
        return 3 + p1;
    }
    if outcome == 0 {
        // loss
        return 0 + (p1 + 2) % 3
    }
    // wins
    return 6 + (p1 + 1) % 3;
}

impl Solution for DayTwo {
    fn part_one(&self, input: &crate::input::Input) -> String {
        input
            .raw
            .lines()
            .fold(0usize, |acc, curr| {
                let (_opponent, _player) =
                    curr.split_once(' ').unwrap_or_else(|| panic!("{}", curr));
                let opponent = (_opponent.as_bytes()[0] - 65) as usize;
                let player = (_player.as_bytes()[0] - 88) as usize;

                let result = score(player, opponent);
                result + player + 1 + acc
            })
            .to_string()
    }

    fn part_two(&self, input: &crate::input::Input) -> String {
        input
            .raw
            .lines()
            .fold(0usize, |acc, curr| {
                let (_opponent, _outcome) =
                    curr.split_once(' ').unwrap_or_else(|| panic!("{}", curr));
                let opponent = (_opponent.as_bytes()[0] - 65) as usize;
                let outcome = (_outcome.as_bytes()[0] - 88) as usize;

                let result = calc_move(opponent, outcome);
                result + 1 + acc
            })
            .to_string()
    }
}
