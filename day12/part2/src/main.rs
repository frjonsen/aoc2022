use std::rc::Rc;
#[derive(PartialEq, Eq)]
pub enum MapPoint {
    Altitude(u8),
    Start,
    End,
}

impl From<&MapPoint> for i32 {
    fn from(val: &MapPoint) -> Self {
        match val {
            MapPoint::Start => 0,
            MapPoint::End => 25,
            MapPoint::Altitude(a) => *a as i32,
        }
    }
}

impl MapPoint {
    fn allowed_to_walk_to(&self, other: &MapPoint) -> bool {
        let from_elevation: i32 = self.into();
        let to_elevation: i32 = other.into();
        to_elevation - from_elevation <= 1
    }
}

pub fn parse_input(input: &str) -> Vec<Vec<MapPoint>> {
    let mut topography: Vec<Vec<MapPoint>> = Vec::new();
    for line in input.trim().lines() {
        let mut p_line: Vec<MapPoint> = Vec::new();

        for c in line.trim().chars().map(|c| match c {
            'S' => MapPoint::Altitude(0),
            'E' => MapPoint::End,
            _ => MapPoint::Altitude(c as u8 - 97),
        }) {
            p_line.push(c);
        }
        topography.push(p_line);
    }
    topography
}

#[derive(PartialEq, Hash, Eq, Clone, Debug)]
pub struct Coordinate(pub usize, pub usize);

impl Coordinate {
    pub fn get_neighbors(&self, x_limit: usize, y_limit: usize) -> Vec<Self> {
        let mut neighbors = vec![];
        if self.0 > 0 {
            neighbors.push(Coordinate(
                ((self.0 as i32) - 1).try_into().unwrap(),
                self.1,
            ));
        }
        if self.0 < x_limit {
            neighbors.push(Coordinate(self.0 + 1, self.1));
        }
        if self.1 > 0 {
            neighbors.push(Coordinate(
                self.0,
                ((self.1 as i32) - 1).try_into().unwrap(),
            ));
        }
        if self.1 < y_limit {
            neighbors.push(Coordinate(self.0, self.1 + 1));
        }
        neighbors
    }
}

#[derive(Eq, Clone)]
struct Node {
    parent: Option<Rc<Node>>,
    position: Coordinate,
    g: usize,
    h: usize,
}

impl Node {
    fn new(parent: Option<Rc<Node>>, position: Coordinate) -> Self {
        Node {
            parent,
            position,
            g: 0,
            h: 0,
        }
    }

    fn f(&self) -> usize {
        self.g + self.h
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

fn find_end(map: &[Vec<MapPoint>]) -> Coordinate {
    for (y, line) in map.iter().enumerate() {
        for (x, _) in line.iter().enumerate() {
            if map[y][x] == MapPoint::End {
                return Coordinate(x, y);
            }
        }
    }

    panic!("Failed to find start and end");
}

pub fn find_path(
    map: &[Vec<MapPoint>],
    start_point: &Coordinate,
    explored_positions: &[Coordinate],
    shortest_known: usize,
    end: &Coordinate,
) -> Option<usize> {
    let x_limit = map[0].len() - 1;
    let y_limit = map.len() - 1;

    let mut open_list: Vec<Rc<Node>> = Vec::new();
    let mut closed_list: Vec<Rc<Node>> = Vec::new();

    let start_node = Node::new(None, start_point.clone());
    open_list.push(Rc::new(start_node));

    while !open_list.is_empty() {
        let (smallest_f_index, _) = open_list
            .iter()
            .enumerate()
            .min_by_key(|e| e.1.f())
            .unwrap();

        let smallest_node = {
            let smallest_node = open_list.swap_remove(smallest_f_index);
            closed_list.push(smallest_node);
            closed_list.last().unwrap()
        };

        if smallest_node.position == *end {
            let mut final_path = Vec::new();
            let mut current = smallest_node.clone();
            while current.parent.is_some() {
                final_path.push(current.position.clone());
                current = current.parent.clone().unwrap();
            }
            return Some(final_path.len());
        }

        let smallest_node_point = &map[smallest_node.position.1][smallest_node.position.0];
        let neighbors = smallest_node
            .position
            .get_neighbors(x_limit, y_limit)
            .into_iter()
            .filter(|p| !explored_positions.contains(p))
            .filter(|p| smallest_node_point.allowed_to_walk_to(&map[p.1][p.0]))
            .map(|p| {
                Rc::new(Node {
                    parent: Some(smallest_node.clone()),
                    h: end.0.abs_diff(p.0) + end.1.abs_diff(p.1),
                    position: p,
                    g: smallest_node.g + 1,
                })
            })
            .filter(|n| n.f() < shortest_known);

        for neighbor in neighbors {
            if closed_list.contains(&neighbor) {
                continue;
            }

            for point in open_list.iter() {
                if point.position == neighbor.position && neighbor.g > point.g {
                    continue;
                }
            }
            open_list.push(neighbor);
        }
    }

    None
}

fn find_all_low_points(map: &[Vec<MapPoint>]) -> Vec<Coordinate> {
    let mut low_points = Vec::new();
    for (y, row) in map.iter().enumerate() {
        for (x, altitude) in row.iter().enumerate() {
            if let MapPoint::Altitude(a) = altitude {
                if *a == 0 {
                    low_points.push(Coordinate(x, y));
                }
            };
        }
    }

    low_points
}

fn run(input: &str) -> usize {
    let map = parse_input(input);
    let end = find_end(&map);
    let mut start_points = find_all_low_points(&map);
    start_points
        .sort_by_cached_key(|f| std::cmp::Reverse(end.0.abs_diff(f.0) + end.1.abs_diff(f.1)));
    let mut smallest = usize::MAX;
    for i in 0..start_points.len() {
        let (explored_points, remaining_points) = start_points.split_at(i);
        let current_position = remaining_points.first().unwrap();
        if let Some(steps) = find_path(&map, &current_position, explored_points, smallest, &end) {
            if steps < smallest {
                smallest = steps
            }
        }
    }

    smallest
}

fn main() {
    let input = include_str!("../../input.txt");
    let minimum_steps = run(input);
    println!("Found path in {minimum_steps} steps");
}

#[cfg(test)]
mod tests {
    use crate::run;

    #[test]
    fn test_sample() {
        let sample = include_str!("../sample.txt");
        let steps = run(sample);
        assert_eq!(29, steps);
    }
}
