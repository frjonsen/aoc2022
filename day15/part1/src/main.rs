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

fn calculate_sensor_coverage(sensor: &Position, reach: u32, row_of_interest: usize) -> i32 {
    let distance_to_row = (sensor.1).abs_diff(row_of_interest as i32) as i32;
    reach as i32 - distance_to_row
}

fn calculate_covered_positions_on_row(sensor_x: i32, reach_on_row: u32) -> HashSet<i32> {
    let mut positions = HashSet::new();
    (-(reach_on_row as i32)..=(reach_on_row as i32)).for_each(|i| {
        positions.insert(sensor_x + i);
    });

    positions
}

fn run(input: &str, row: usize) -> usize {
    let mut covered_positions: HashSet<i32> = HashSet::new();

    let items = parse_input(input);
    let reach_per_sensor = items.iter().map(|(sensor, beacon)| {
        (
            sensor.clone(),
            sensor.1.abs_diff(beacon.1) + sensor.0.abs_diff(beacon.0),
        )
    });
    for (sensor, reach) in reach_per_sensor {
        let reach_on_row: Result<u32, _> =
            calculate_sensor_coverage(&sensor, reach, row).try_into();
        if let Ok(reach_on_row) = reach_on_row {
            let new_covered_positions = calculate_covered_positions_on_row(sensor.0, reach_on_row);
            covered_positions.extend(new_covered_positions.into_iter());
        }
    }
    let mut beacons_on_row = Vec::<Position>::new();
    for (_, beacon) in items.iter().filter(|(_, b)| b.1 == row as i32) {
        if !beacons_on_row.contains(&beacon) {
            beacons_on_row.push((*beacon).clone());
        }
    }

    covered_positions.len() - beacons_on_row.len()
}

fn main() {
    let input = include_str!("../../input.txt");
    let res = run(input, 2000000);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use crate::{calculate_sensor_coverage, parse_line, run, Position};

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
        let res = run(input, 10);
        assert_eq!(26, res);
    }

    #[test]
    fn test_calculate_sensor_coverage() {
        let sensor = Position(8, 7);
        let reach = 9;
        let row_of_interest = 10;
        let res = calculate_sensor_coverage(&sensor, reach, row_of_interest);
        assert_eq!(6, res);
    }
}
