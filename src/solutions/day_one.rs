pub fn solve(input: String) -> Option<(i32, i32)> {
    let mut tmp: Vec<i32> = input
        .split("\n\n")
        .map(|group| {
            group
                .split('\n')
                .fold(0, |acc, curr| acc + curr.parse::<i32>().unwrap())
        })
        .collect();
    tmp.sort_by(|a, b| b.cmp(a));

    Some((tmp[0], tmp.iter().take(3).sum::<i32>()))
}
