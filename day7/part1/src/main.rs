use itertools::Itertools;
use std::{collections::HashMap, path::PathBuf};

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

            let mut current_walk = current_path.parent();
            while let Some(parent) = current_walk {
                current_walk = parent.parent();
                *directories.get_mut(parent).unwrap() += dir_size;
            }
        }
    }

    directories
}

fn main() {
    let input = include_str!("../../input.txt");
    let res = run(input);
    let sum: u64 = res
        .values()
        .filter_map(|d| if *d > 100_000 { None } else { Some(d) })
        .sum();

    println!("{sum}")
}
