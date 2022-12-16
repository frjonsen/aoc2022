use std::collections::HashSet;

use regex::Regex;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
struct Position(i32, i32);

fn parse_line(input: &str) -> (Position, Position) {
    let pattern =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();

    let captures = pattern.captures(input).unwrap();
    let coordinates: Vec<i32> = captures
        .iter()
        .skip(1)
        .map(|c| c.unwrap().as_str().parse::<i32>().unwrap())
        .collect();
    let sensor = Position(coordinates[0], coordinates[1]);
    let beacon = Position(coordinates[2], coordinates[3]);
    (sensor, beacon)
}

fn parse_input(input: &str) -> Vec<(Position, Position)> {
    input.lines().map(parse_line).collect()
}

fn calculate_distance(sensor: &Position, target: &Position) -> u32 {
    sensor.0.abs_diff(target.0) + sensor.1.abs_diff(target.1)
}

fn sensor_covers_position(sensor: &Position, reach: u32, target: &Position) -> bool {
    calculate_distance(sensor, target) <= reach
}

fn calculate_positions_out_of_reach(
    sensor: &Position,
    reach: u32,
    max_coordinate: i32,
) -> HashSet<Position> {
    let out_of_reach: i32 = reach as i32 + 1;
    let mut positions = HashSet::new();
    for y in 0..=out_of_reach {
        let dy = y;
        let dx = out_of_reach - y;

        positions.insert(Position(sensor.0 + dx, sensor.1 + dy));
        positions.insert(Position(sensor.0 + dx, sensor.1 - dy));
        positions.insert(Position(sensor.0 - dx, sensor.1 - dy));
        positions.insert(Position(sensor.0 - dx, sensor.1 + dy));
    }

    positions
        .into_iter()
        .filter(|p| p.0 <= max_coordinate && p.1 <= max_coordinate && p.0 >= 0 && p.1 >= 0)
        .collect()
}

fn run(input: &str, max_coordinate: i32) -> u128 {
    let sensors = parse_input(input);
    let sensor_reach: Vec<_> = sensors
        .into_iter()
        .map(|s| {
            let reach = calculate_distance(&s.0, &s.1);
            (s.0, reach)
        })
        .collect();

    for (sensor, reach) in sensor_reach.iter() {
        let positions_outside = calculate_positions_out_of_reach(sensor, *reach, max_coordinate);
        let uncovered = positions_outside.iter().find(|p| {
            !sensor_reach
                .iter()
                .any(|(s, r)| sensor_covers_position(s, *r, p))
        });

        if let Some(p) = uncovered {
            return (p.0 as u128) * 4_000_000 + (p.1 as u128);
        }
    }

    panic!("Not found");
}

fn main() {
    let input = include_str!("../../input.txt");
    let res = run(input, 4_000_000);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use crate::{
        calculate_positions_out_of_reach, parse_line, run, sensor_covers_position, Position,
    };

    #[test]
    fn test_parse_sample() {
        let input = include_str!("../sample.txt");
        let top = input.lines().next().unwrap();
        let parsed = parse_line(top);

        assert_eq!(parsed.0, Position(2, 18));
        assert_eq!(parsed.1, Position(-2, 15));
    }

    #[test]
    fn test_coverage() {
        let input = include_str!("../sample.txt");
        let res = run(input, 20);
        assert_eq!(56000011, res);
    }

    #[test]
    fn test_calculate_positions_out_of_reach() {
        let sensor = Position(5, 5);
        let reach = 1;
        let res = calculate_positions_out_of_reach(&sensor, reach, 5000);

        let out_of_reach = [
            Position(5, 3),
            Position(6, 4),
            Position(7, 5),
            Position(6, 6),
            Position(5, 7),
            Position(4, 6),
            Position(3, 5),
            Position(4, 4),
        ];

        for p in out_of_reach.into_iter() {
            assert!(res.contains(&p), "{p:?}");
        }
    }

    #[test]
    fn test_sensor_covers_position() {
        let sensor = Position(5, 5);
        let reach = 1;
        assert!(!sensor_covers_position(&sensor, reach, &Position(5, 3)));
        assert!(sensor_covers_position(&sensor, reach, &Position(5, 4)));
    }
}
