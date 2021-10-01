pub struct Solution;

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn from(vector: &Vec<i32>) -> Self {
        Point {
            x: vector[0],
            y: vector[1],
        }
    }

    fn lies_on_line(&self, line: &Line) -> bool {
        (self.y - line.0.y) * (line.1.x - line.0.x) == (self.x - line.0.x) * (line.1.y - line.0.y)
    }
}

#[derive(Debug)]
struct Line(Point, Point);

impl Line {
    fn slope(&self) -> f64 {
        f64::from(self.1.x - self.0.x) / f64::from(self.1.y - self.0.y)
    }

    fn y_intercept(&self) -> f64 {
        f64::from(self.0.y) - self.slope() * f64::from(self.0.x)
    }
}

impl std::cmp::PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        other.0.lies_on_line(self) && self.0.lies_on_line(other)
    }
}

impl std::cmp::Eq for Line {}

use std::cmp::Ordering;

impl std::cmp::PartialOrd for Line {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.slope().partial_cmp(&other.slope()) {
            Some(Ordering::Equal) => (self.y_intercept()).partial_cmp(&other.y_intercept()),
            otherwise => otherwise,
        }
    }
}

impl std::cmp::Ord for Line {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        use std::collections::{BTreeMap, HashSet};
        let mut recorded_lines: BTreeMap<Line, HashSet<Point>> = BTreeMap::new();

        for i in 0..points.len() {
            let point_a = Point::from(&points[i]);
            for j in i + 1..points.len() {
                let point_b = Point::from(&points[j]);

                recorded_lines
                    .iter()
                    .for_each(|(line, num_points)| println!("{:?} -> {:?}", line, num_points));
                println!();

                let record = recorded_lines
                    .entry(Line(point_a, point_b))
                    .or_insert(HashSet::new());
                record.insert(point_a);
                record.insert(point_b);
            }
        }

        0
    }
}
