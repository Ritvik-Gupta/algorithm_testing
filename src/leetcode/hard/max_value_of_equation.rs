pub struct Solution;

struct Vector(i32, i32);

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.diff() == other.diff()
    }
}

impl Eq for Vector {}

impl PartialOrd for Vector {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.diff().cmp(&other.diff()))
    }
}

impl std::cmp::Ord for Vector {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Vector {
    fn diff(&self) -> i32 {
        self.1 - self.0
    }

    fn x_diff(&self, other: &Self) -> i32 {
        self.0 - other.0
    }

    fn eqn_value(&self, other: &Self) -> i32 {
        self.1 + other.1 + self.x_diff(other)
    }
}

impl Into<Vector> for &Vec<i32> {
    fn into(self) -> Vector {
        Vector(self[0], self[1])
    }
}

use std::collections::BinaryHeap;

struct EqnPoints {
    max_separation: i32,
    visited_points: BinaryHeap<Vector>,
}

impl EqnPoints {
    fn new(max_separation: i32, capacity: usize) -> Self {
        EqnPoints {
            max_separation,
            visited_points: BinaryHeap::with_capacity(capacity),
        }
    }

    fn add_relative_to_first(&mut self, point: Vector) -> i32 {
        let mut result = i32::MIN;
        while let Some(best_eqn_point) = self.visited_points.peek() {
            if point.x_diff(best_eqn_point) > self.max_separation {
                self.visited_points.pop();
            } else {
                result = point.eqn_value(best_eqn_point);
                break;
            }
        }
        self.visited_points.push(point);
        result
    }
}

impl Solution {
    pub fn find_max_value_of_equation(points: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut eqn_points = EqnPoints::new(k, points.len());
        points
            .iter()
            .map(|point| eqn_points.add_relative_to_first(point.into()))
            .max()
            .unwrap()
    }
}
