use std::collections::HashSet;

fn find_marker(bytes: &[u8], size: usize) -> Option<usize> {
    let mut set = HashSet::new();
    for (index, window) in bytes.windows(size).enumerate() {
        for char in window.iter() {
            set.insert(char);
        }
        if set.len() == size {
            return Some(index + size);
        }
        set.drain();
    }
    None
}

pub fn solve(input: String) -> Option<(usize, usize)> {
    let part_one = find_marker(input.as_bytes(), 4).unwrap();
    let part_two = find_marker(input.as_bytes(), 14).unwrap();
    Some((part_one, part_two))
}
