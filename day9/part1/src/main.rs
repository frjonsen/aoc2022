use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Clone)]
struct Point {
    x: i64,
    y: i64,
}

fn run(input: &str) -> usize {
    let mut visited_points: HashSet<Point> = HashSet::new();
    let mut head = Point { x: 0, y: 0 };
    let mut tail = head.clone();
    visited_points.insert(tail.clone());

    for line in input.trim().lines().map(str::trim) {
        let (direction, steps) = line.split_once(' ').expect("Line had weird format");
        let steps: i64 = steps.parse().unwrap();
        for _ in 0..steps {
            match direction {
                "R" => head.x += 1,
                "L" => head.x -= 1,
                "U" => head.y += 1,
                "D" => head.y -= 1,
                _ => panic!("Invalid direction"),
            };

            if (head.x - tail.x).abs() < 2 && (head.y - tail.y).abs() < 2 {
                continue;
            }

            let dy = head.y - tail.y;
            let dx = head.x - tail.x;

            let y_move = dy.clamp(-1, 1);
            let x_move = dx.clamp(-1, 1);

            if dy.abs() >= 2 {
                tail.y += y_move;
                tail.x = head.x;
            } else if dx.abs() >= 2 {
                tail.x += x_move;
                tail.y = head.y;
            }

            visited_points.insert(tail.clone());
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
        assert_eq!(13, res);
    }
}
