use std::arch::x86_64::_MM_ROUND_DOWN;

use itertools::Itertools;

pub fn solve(input: String) -> Option<(i32, i32)> {
    let map = input.as_bytes().split(|&ch| ch == b'\n').collect_vec();
    let start_coord = 'start_coords: {
        for row in 0..map.len() - 1 {
            if let Ok(col) = map[row].binary_search(&b'S') {
                break 'start_coords (col, row);
            }
        }
        (0usize, 0usize)
    };
    dbg!(map);

    None
}
