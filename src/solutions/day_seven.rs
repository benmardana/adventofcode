use std::collections::HashMap;

const TOTAL_DISK: i32 = 70000000;
const UPDATE_SPACE_REQ: i32 = 30000000;

pub fn solve(input: String) -> Option<(i32, i32)> {
    let lines = input.lines();
    let mut directories: HashMap<String, i32> = HashMap::new();
    let mut path_vec: Vec<&str> = Vec::new();
    let mut ls_mode = false;

    for line in lines {
        match line.trim_start_matches("$ ").split_once(' ') {
            Some(("cd", path)) => {
                if ls_mode {
                    ls_mode = false;
                }
                match path {
                    ".." => {
                        path_vec.pop();
                    }
                    "/" => {
                        path_vec.clear();
                        path_vec.push("~");
                    }
                    _ => {
                        path_vec.push(path);
                    }
                }
            }
            Some(("dir", _)) => {}
            Some((size, _)) => {
                path_vec.iter().fold(String::new(), |acc, curr| {
                    let path = if acc.is_empty() {
                        curr.to_string()
                    } else {
                        format!("{}/{}", acc, curr)
                    };
                    let sum = directories.get(&path).unwrap_or(&0);
                    directories.insert(path.clone(), size.parse::<i32>().unwrap() + sum);
                    path
                });
            }
            None => {
                ls_mode = true;
            }
        }
    }
    let part_one = directories
        .clone()
        .into_values()
        .reduce(|acc, curr| if curr < 100000 { acc + curr } else { acc })
        .unwrap();

    let space_required = UPDATE_SPACE_REQ - (TOTAL_DISK - directories.get("~").unwrap());
    let mut directory_sizes = directories.into_values().collect::<Vec<_>>();
    directory_sizes.sort();
    let part_two = directory_sizes
        .iter()
        .find(|&v| v >= &space_required)
        .unwrap();

    Some((part_one, *part_two))
}
