pub fn solve(input: String) -> Option<(i32, i32)> {
    let lines = input.split('\n');
    let ranges = lines.clone().map(|line| {
        let (range_a, range_b): ((u8, u8), (u8, u8)) = line
            .split_once(',')
            .map(|(string_a, string_b)| {
                (
                    string_a
                        .split_once('-')
                        .map(|(lower, upper)| (lower.parse().unwrap(), upper.parse().unwrap()))
                        .unwrap(),
                    string_b
                        .split_once('-')
                        .map(|(lower, upper)| (lower.parse().unwrap(), upper.parse().unwrap()))
                        .unwrap(),
                )
            })
            .unwrap();
        (range_a, range_b)
    });

    let part_one: i32 = ranges
        .clone()
        .filter_map(|((lower_a, upper_a), (lower_b, upper_b))| {
            if lower_a <= lower_b && upper_a >= upper_b {
                return Some(());
            }
            if lower_b <= lower_a && upper_b >= upper_a {
                return Some(());
            }
            None
        })
        .collect::<Vec<_>>()
        .len()
        .try_into()
        .unwrap();

    let part_two: i32 = ranges
        .filter_map(|((lower_a, upper_a), (lower_b, upper_b))| {
            if lower_a <= upper_b && lower_b <= upper_a {
                return Some(());
            }
            None
        })
        .collect::<Vec<_>>()
        .len()
        .try_into()
        .unwrap();

    Some((part_one, part_two))
}
