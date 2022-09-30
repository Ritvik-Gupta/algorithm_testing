crate::solution!();

struct Vec2D(i32, i32);

impl Vec2D {
    fn distance(&self) -> u32 {
        let (x, y) = (self.0 as u32, self.1 as u32);
        x.pow(2) + y.pow(2)
    }
}

impl PartialEq for Vec2D {
    fn eq(&self, other: &Self) -> bool {
        other.distance().eq(&self.distance())
    }
}

impl Eq for Vec2D {}

use std::cmp::Ordering;

impl PartialOrd for Vec2D {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.distance().partial_cmp(&self.distance())
    }
}

impl Ord for Vec2D {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance().cmp(&self.distance())
    }
}

impl From<&Vec<i32>> for Vec2D {
    fn from(vector: &Vec<i32>) -> Self {
        Self(vector[0], vector[1])
    }
}

impl From<Vec2D> for Vec<i32> {
    fn from(position: Vec2D) -> Self {
        vec![position.0, position.1]
    }
}

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        use std::collections::BinaryHeap;

        let mut closest_to_origin = BinaryHeap::with_capacity(points.len());

        points
            .iter()
            .for_each(|point| closest_to_origin.push(Vec2D::from(point)));

        (0..k)
            .map(|_| closest_to_origin.pop().unwrap())
            .map(From::from)
            .collect()
    }
}
