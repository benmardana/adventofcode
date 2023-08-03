use std::cmp;

pub fn solve(input: String) -> Option<(i32, i32)> {
    let trees = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut visible_trees = 0;
    for y in 0..trees.len() {
        for x in 0..trees[y].len() {
            dbg!(trees[y][x]);
            if is_visible(&trees, x, y) {
                visible_trees += 1;
                dbg!(true);
            } else {
                dbg!(false);
            }
        }
    }
    Some((visible_trees, 0))
}

fn is_visible(input: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    let target_tree: u32 = input[y][x];

    // up
    if ((y.saturating_sub(1))..=0).all(|iy| input[iy][x] < target_tree) {
        return true;
    }

    // down
    if (cmp::min(y + 1, input.len() - 1)..input.len()).all(|iy| input[iy][x] < target_tree) {
        return true;
    }

    // left
    if ((x.saturating_sub(1))..=0).all(|ix| input[y][ix] < target_tree) {
        return true;
    }

    // right
    if (cmp::min(x + 1, input.len() - 1)..input[0].len()).all(|ix| input[y][ix] < target_tree) {
        return true;
    }

    false
}
