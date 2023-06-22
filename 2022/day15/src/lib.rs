use std::collections::BTreeSet;

#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct Point(pub i32, pub i32);
impl Point {
    fn distance_to(&self, other: &Point) -> i32 {
        ((self.0.abs_diff(other.0)) + (self.1.abs_diff(other.1))) as i32
    }
}
fn range(point: &Point, distance: i32, line: i32) -> Option<(i32, i32)> {
    let vertical = (point.1 - line).abs();
    if vertical > distance {
        None
    } else {
        let horizontal = distance - vertical;
        Some((point.0 - horizontal, point.0 + horizontal))
    }
}
pub fn solve(input: &Vec<(Point, Point)>, line: i32) -> i32 {
    let mut blocked = BTreeSet::new();
    let mut beacons = BTreeSet::new();
    let mut iter = input.iter();
    while let Some((sensor, beacon)) = iter.next() {
        if beacon.1 == line {
            beacons.insert(beacon.0);
        }
        let distance = sensor.distance_to(&beacon);
        if let Some((start, end)) = range(&sensor, distance, line) {
            for col in start..=end {
                blocked.insert(col);
            }
        }
    }
    (blocked.len() - beacons.len()) as i32
}
fn in_range(sensors: &Vec<(Point, Point)>, row: usize, col: usize) -> bool {
    for (sensor, beacon) in sensors {
        let radius = sensor.distance_to(&beacon);
        if sensor.distance_to(&Point(col as i32, row as i32)) <= radius {
            return true;
        }
    }
    false
}
pub fn solve_b(input: &Vec<(Point, Point)>, search_range: i32) -> u64 {
    for row in 0..=search_range {
        let mut current = -1;
        while current < search_range {
            if let Some((_, end)) = input
                .iter()
                .filter_map(|(sensor, beacon)| {
                    range(sensor, sensor.distance_to(beacon), row as i32)
                })
                .filter(|(start, end)| (*start <= current + 1) && (*end > current))
                .next()
            {
                current = end;
            } else {
                println!("row:{}, col :{}", row, current + 1);
                return (4000000 * (current as u64 + 1) + row as u64);
            }
        }
    }
    0
}
pub fn line_to_points(line: &str) -> (Point, Point) {
    let mut iter = line.split('=');
    iter.next();
    let x1: i32 = iter
        .next()
        .unwrap()
        .split(',')
        .next()
        .unwrap()
        .parse()
        .unwrap();
    let y1: i32 = iter
        .next()
        .unwrap()
        .split(':')
        .next()
        .unwrap()
        .parse()
        .unwrap();
    let x2: i32 = iter
        .next()
        .unwrap()
        .split(',')
        .next()
        .unwrap()
        .parse()
        .unwrap();
    let y2: i32 = iter.next().unwrap().parse().unwrap();
    (Point(x1, y1), Point(x2, y2))
}
