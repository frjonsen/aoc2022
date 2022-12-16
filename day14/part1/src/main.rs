use std::thread::current;
use std::thread::sleep;
use std::time::Duration;

#[derive(Eq, PartialEq, Debug)]
struct Position(usize, usize);

impl Position {
    fn move_candidates(&self) -> Vec<Self> {
        let new_y = self.1 + 1;
        vec![
            Position(self.0, new_y),
            Position(self.0 - 1, new_y),
            Position(self.0 + 1, new_y),
        ]
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
enum Material {
    Sand,
    Air,
    Rock,
}

fn parse_line(line: &str) -> Vec<Position> {
    line.split(" -> ")
        .map(|l| {
            l.split_once(',')
                .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                .map(|(x, y)| Position(x, y))
                .unwrap()
        })
        .collect()
}

fn parse_input(input: &str) -> Vec<Vec<Position>> {
    input.trim().lines().map(parse_line).collect()
}

fn get_boundaries(input: &Vec<Vec<Position>>) -> (usize, usize) {
    let mut largest_x = usize::MIN;
    let mut largest_y = usize::MIN;

    for p in input.iter().flatten() {
        if p.0 > largest_x {
            largest_x = p.0;
        }
        if p.1 > largest_y {
            largest_y = p.1;
        }
    }

    (largest_x, largest_y)
}

fn draw_map(map: &Vec<Vec<Material>>, current_position: &Position) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if current_position == &Position(x, y) {
                print!("+");
            } else {
                print!(
                    "{}",
                    match map[y][x] {
                        Material::Air => ".",
                        Material::Rock => "#",
                        Material::Sand => "o",
                    }
                )
            }
        }
        println!()
    }
}

fn positions_to_map(input: Vec<Vec<Position>>) -> Vec<Vec<Material>> {
    let boundaries = get_boundaries(&input);

    let mut map: Vec<Vec<Material>> = (0..boundaries.1 + 2)
        .map(|_| vec![Material::Air; boundaries.0 + 1])
        .collect();

    for line in input.iter() {
        for pair in line.windows(2) {
            if pair[0].0 == pair[1].0 {
                let from = pair[0].1.min(pair[1].1);
                let to = pair[0].1.max(pair[1].1);
                for y in from..=to {
                    map[y][pair[0].0] = Material::Rock;
                }
            } else {
                let from = pair[0].0.min(pair[1].0);
                let to = pair[0].0.max(pair[1].0);
                for x in from..=to {
                    map[pair[0].1][x] = Material::Rock
                }
            }
        }
    }

    map
}

fn simulate_move(current_position: &Position, map: &Vec<Vec<Material>>) -> Option<Position> {
    let potential_positions = current_position.move_candidates();

    for candidate in potential_positions {
        if map[candidate.1][candidate.0] == Material::Air {
            return Some(candidate);
        }
    }

    None
}

fn main() {
    let input = include_str!("../../input.txt");
    let map = parse_input(input);
    let map = positions_to_map(map);
    println!("{} came to rest", run(map));
}

fn run(mut map: Vec<Vec<Material>>) -> usize {
    let mut sand_count = 0;
    loop {
        sand_count += 1;
        let mut current_position = Position(500, 0);

        while let Some(new_position) = simulate_move(&current_position, &map) {
            if new_position.1 >= (map.len() - 1) {
                draw_map(&map, &current_position);
                return sand_count - 1;
            }
            current_position = new_position;
        }

        map[current_position.1][current_position.0] = Material::Sand;
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, parse_line, positions_to_map, run, Material, Position};

    #[test]
    fn test_parse_sample_line() {
        let input = "498,4 -> 498,6 -> 496,6";
        let res = parse_line(input);
        assert_eq!(
            res,
            vec![Position(498, 4), Position(498, 6), Position(496, 6)]
        )
    }

    #[test]
    fn test_convert_to_map() {
        let input = "498,4 -> 498,6 -> 496,6";
        let res = parse_input(input);
        let map = positions_to_map(res);
        assert_eq!(map[4][498], Material::Rock);
    }

    #[test]
    fn test_sample() {
        let input = include_str!("../sample.txt");
        let map = positions_to_map(parse_input(input));

        assert_eq!(24, run(map));
    }
}
