use itertools::Itertools;
use std::{collections::HashMap, path::PathBuf, str::FromStr};

fn run<'a>(input: &'a str) -> HashMap<PathBuf, u64> {
    let mut current_path = PathBuf::new();
    let mut directories: HashMap<PathBuf, u64> = HashMap::new();
    let commands = input.split("$").skip(1).map(str::trim);

    for cmd in commands {
        if cmd.starts_with("cd") {
            let (_, path) = cmd
                .split_whitespace()
                .collect_tuple::<(&str, &str)>()
                .expect("Failed to parse cd command");

            match path {
                ".." => current_path = current_path.parent().unwrap().to_path_buf(),
                p => current_path.push(p),
            };
        } else if cmd.starts_with("ls") {
            let dir_size: u64 = cmd
                .lines()
                .skip(1)
                .filter_map(|l| {
                    let (size, _) = l.split_once(' ').expect("Failed to split entry");
                    size.parse::<u64>().ok()
                })
                .sum();

            directories.insert(current_path.clone(), dir_size);

            let mut current = current_path.parent();
            while let Some(parent) = current {
                current = parent.parent();
                *directories.get_mut(parent).unwrap() += dir_size;
            }
        }
    }

    directories
}

fn main() {
    let input = include_str!("../../input.txt");
    let res = run(input);

    let current_free = 70_000_000 - res.get(&PathBuf::from_str("/").unwrap()).unwrap();
    let needed_to_free = 30_000_000 - current_free;

    let smallest_option = res.values().filter(|e| **e > needed_to_free).min().unwrap();

    println!("{smallest_option}")
}
