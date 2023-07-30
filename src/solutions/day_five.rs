pub fn solve(input: String) -> Option<(String, String)> {
    let crates = vec![
        vec!['B', 'Z', 'T'],
        vec!['V', 'H', 'T', 'D', 'N'],
        vec!['B', 'F', 'M', 'D'],
        vec!['T', 'J', 'G', 'W', 'V', 'Q', 'L'],
        vec!['W', 'D', 'G', 'P', 'V', 'F', 'Q', 'M'],
        vec!['V', 'Z', 'Q', 'G', 'H', 'F', 'S'],
        vec!['Z', 'S', 'N', 'R', 'L', 'T', 'C', 'W'],
        vec!['Z', 'H', 'W', 'D', 'J', 'N', 'R', 'M'],
        vec!['M', 'Q', 'L', 'F', 'D', 'S'],
    ];

    let (_, procedure_input) = input.split_once("\n\n").unwrap();

    let part_one = {
        let mut local_crates = crates.clone();
        for proc in procedure_input.lines() {
            let parts = proc.splitn(6, ' ').collect::<Vec<_>>();
            let n: usize = parts[1].parse().unwrap();
            let from: usize = parts[3].parse().unwrap();
            let to: usize = parts[5].parse().unwrap();

            for _ in 0..n {
                let popped = local_crates[from - 1].pop().unwrap();
                local_crates[to - 1].push(popped);
            }
        }
        local_crates
            .iter()
            .map(|x| x.last().unwrap())
            .collect::<String>()
    };

    let part_two = {
        let mut local_crates = crates;
        for proc in procedure_input.lines() {
            let parts = proc.splitn(6, ' ').collect::<Vec<_>>();
            let n: usize = parts[1].parse().unwrap();
            let from: usize = parts[3].parse().unwrap();
            let to: usize = parts[5].parse().unwrap();

            let split = local_crates[from - 1].len() - n;
            let mut popped = local_crates[from - 1].split_off(split);
            local_crates[to - 1].append(&mut popped);
        }
        local_crates
            .iter()
            .map(|x| x.last().unwrap())
            .collect::<String>()
    };
    Some((part_one, part_two))
}
