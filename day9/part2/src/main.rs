use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Point {
    x: i64,
    y: i64,
}

fn visualize(knots: &Vec<Point>) {
    let smallest_x = knots.iter().map(|c| c.x).min().unwrap().min(0);
    let largest_x = knots.iter().map(|c| c.x).max().unwrap().max(0);
    let smallest_y = knots.iter().map(|c| c.y).min().unwrap().min(0);
    let largest_y = knots.iter().map(|c| c.y).max().unwrap().max(0);

    for y in smallest_y..=largest_y {
        for x in smallest_x..=largest_x {
            if let Some(point) = knots.iter().enumerate().find(|c| c.1 == &Point { y, x }) {
                let c = if point.0 == 0 {
                    "H".to_owned()
                } else {
                    (point.0).to_string()
                };
                print!("{}", c);
            } else if x == 0 && y == 0 {
                print!("s");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    println!("")
}

fn run(input: &str) -> usize {
    let mut visited_points: HashSet<Point> = HashSet::new();
    let mut knots = vec![Point { x: 0, y: 0 }; 10];

    for line in input.trim().lines().map(str::trim) {
        let (direction, steps) = line.split_once(' ').expect("Line had weird format");
        let steps: i64 = steps.parse().unwrap();
        for _ in 0..steps {
            match direction {
                "R" => knots[0].x += 1,
                "L" => knots[0].x -= 1,
                "U" => knots[0].y -= 1,
                "D" => knots[0].y += 1,
                _ => panic!("Invalid direction"),
            };

            for i in 0..(knots.len() - 1) {
                let dy = knots[i].y - knots[i + 1].y;
                let dx = knots[i].x - knots[i + 1].x;

                if dy.abs() >= 2 && dx.abs() >= 2 {
                    knots[i + 1].y += dy.clamp(-1, 1);
                    knots[i + 1].x += dx.clamp(-1, 1);
                } else if dy.abs() >= 2 {
                    knots[i + 1].y += dy.clamp(-1, 1);
                    knots[i + 1].x = knots[i].x;
                } else if dx.abs() >= 2 {
                    knots[i + 1].x += dx.clamp(-1, 1);
                    knots[i + 1].y = knots[i].y;
                }
            }

            visited_points.insert(knots.last().unwrap().clone());
        }
    }

    visited_points.len()
}

fn main() {
    let input = include_str!("../../input.txt");
    let res = run(input);
    println!("{res}")
}

#[cfg(test)]
mod tests {
    use crate::run;

    #[test]
    fn test_sample() {
        let input = "R 4
                     U 4
                     L 3
                     D 1
                     R 4
                     D 1
                     L 5
                     R 2";

        let res = run(input);
        assert_eq!(1, res);
    }

    #[test]
    fn test_second_sample() {
        let input = "R 5
        U 8
        L 8
        D 3
        R 17
        D 10
        L 25
        U 20";
        let res = run(input);
        assert_eq!(36, res);
    }
}
