pub fn solve(input: String) -> Option<(i32, i32)> {
    let trees = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut visible_trees = ((trees.len() - 2) * 2) + (trees[0].len() * 2);
    for y in 1..trees.len() - 1 {
        for x in 1..trees[y].len() - 1 {
            if is_visible(&trees, x, y) {
                visible_trees += 1;
            }
        }
    }
    Some((visible_trees.try_into().unwrap(), 0))
}

fn _score(input: &[Vec<u32>], x: usize, y: usize) -> u8 {
    let target_tree: u32 = input[y][x];
    let mut view_length = 0;
    for tree in input[0..y].iter().rev().map(|row| row[x]) {
        view_length += 1;
        if tree >= target_tree {
            break;
        }
    }
    view_length
}

fn is_visible(input: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    let target_tree: u32 = input[y][x];

    // up
    if input[0..y].iter().map(|row| row[x]).max().unwrap() < target_tree {
        return true;
    }

    // down
    if input[y + 1..input.len()]
        .iter()
        .map(|row| row[x])
        .max()
        .unwrap()
        < target_tree
    {
        return true;
    }

    // left
    if input[y][0..x].iter().max().unwrap() < &target_tree {
        return true;
    }

    // right
    if input[y][x + 1..input[y].len()].iter().max().unwrap() < &target_tree {
        return true;
    }

    false
}
