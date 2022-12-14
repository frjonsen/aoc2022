use std::rc::Rc;
#[derive(PartialEq, Eq)]
pub enum MapPoint {
    Altitude(u8),
    Start,
    End,
}

pub fn parse_input() -> Vec<Vec<MapPoint>> {
    let input = include_str!("../../input.txt");
    let mut topography: Vec<Vec<MapPoint>> = Vec::new();
    for line in input.trim().lines() {
        let mut p_line: Vec<MapPoint> = Vec::new();

        for c in line.trim().chars().map(|c| match c {
            'S' => MapPoint::Start,
            'E' => MapPoint::End,
            _ => MapPoint::Altitude(c as u8 - 97),
        }) {
            p_line.push(c);
        }
        topography.push(p_line);
    }
    topography
}

#[derive(PartialEq, Hash, Eq, Clone)]
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

fn find_start_and_end(map: &[Vec<MapPoint>]) -> (Coordinate, Coordinate) {
    let mut start: Option<Coordinate> = None;
    let mut end: Option<Coordinate> = None;
    'outer: for (y, line) in map.iter().enumerate() {
        for (x, _) in line.iter().enumerate() {
            if map[y][x] == MapPoint::Start {
                start = Some(Coordinate(x, y));
            } else if map[y][x] == MapPoint::End {
                end = Some(Coordinate(x, y))
            }

            if start.is_some() && end.is_some() {
                break 'outer;
            }
        }
    }

    if start.is_none() || end.is_none() {
        panic!("Failed to find start and end");
    }

    (start.unwrap(), end.unwrap())
}

pub fn find_path(map: &[Vec<MapPoint>]) -> Vec<Coordinate> {
    let (start, end) = find_start_and_end(map);
    let x_limit = map[0].len() - 1;
    let y_limit = map.len() - 1;

    let mut open_list: Vec<Rc<Node>> = Vec::new();
    let mut closed_list: Vec<Rc<Node>> = Vec::new();

    let start_node = Node::new(None, Coordinate(start.0, start.1));
    open_list.push(Rc::new(start_node));

    while !open_list.is_empty() {
        let mut smallest_f = usize::MAX;
        let mut smallest_f_index = usize::MAX;

        for (i, e) in open_list.iter().enumerate() {
            if e.f() < smallest_f {
                smallest_f = e.f();
                smallest_f_index = i;
            }
        }

        let smallest_node = {
            let smallest_node = open_list.swap_remove(smallest_f_index);
            closed_list.push(smallest_node);
            closed_list.last().unwrap()
        };
        if smallest_node.position == end {
            let mut final_path = Vec::new();
            let mut current = smallest_node.clone();
            while current.parent.is_some() {
                final_path.push(current.position.clone());
                current = current.parent.clone().unwrap();
            }
            return final_path;
        }

        let neighbors = smallest_node.position.get_neighbors(x_limit, y_limit);
        let neighbors = neighbors
            .into_iter()
            .filter(|p| {
                let point = &map[p.1][p.0];
                let current_node = &map[smallest_node.position.1][smallest_node.position.0];
                match point {
                    MapPoint::Start => true,
                    MapPoint::End => true,
                    MapPoint::Altitude(a) => match current_node {
                        MapPoint::Start => true,
                        MapPoint::End => true,
                        MapPoint::Altitude(ca) => {
                            let a = *a as i32;
                            let ca = *ca as i32;
                            a - ca <= 1
                        }
                    },
                }
            })
            .map(|p| {
                let mut n = Node::new(Some(smallest_node.clone()), p);
                n.g = smallest_node.g + 1;
                n.h = end.0.abs_diff(n.position.0) + end.1.abs_diff(n.position.1);
                Rc::new(n)
            });

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

    panic!("Found no path");
}

fn main() {
    let map = parse_input();
    let path = find_path(&map);
    println!("Found path in {} steps", path.len());
}
