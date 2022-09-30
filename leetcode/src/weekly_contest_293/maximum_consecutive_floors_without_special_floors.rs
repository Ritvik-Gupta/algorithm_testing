crate::solution!();

struct FloorsRange(i32, i32);

impl FloorsRange {
    fn create(bottom: i32, top: i32) -> Option<FloorsRange> {
        if bottom > top {
            return None;
        }
        Some(Self(bottom, top))
    }

    fn contains(&self, val: i32) -> bool {
        self.0 <= val && val <= self.1
    }
}

impl PartialEq for FloorsRange {
    fn eq(&self, other: &Self) -> bool {
        other.contains(self.0) || other.contains(self.1)
    }
}

impl Eq for FloorsRange {}

use std::cmp::Ordering::{self, Equal, Greater, Less};

impl PartialOrd for FloorsRange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Equal)
        } else if self.0 > other.1 {
            Some(Greater)
        } else if other.0 > self.1 {
            Some(Less)
        } else {
            None
        }
    }
}

impl Ord for FloorsRange {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

use std::collections::BTreeSet;

impl Solution {
    pub fn max_consecutive(bottom: i32, top: i32, special: Vec<i32>) -> i32 {
        let mut floor_ranges = BTreeSet::new();
        floor_ranges.insert(FloorsRange(bottom, top));

        for special_floor in special {
            if let Some(range) = floor_ranges.take(&FloorsRange(special_floor, special_floor)) {
                FloorsRange::create(range.0, special_floor - 1)
                    .map(|range| floor_ranges.insert(range));
                FloorsRange::create(special_floor + 1, range.1)
                    .map(|range| floor_ranges.insert(range));
            }
        }

        floor_ranges
            .iter()
            .map(|range| 1 + range.1 - range.0)
            .max()
            .unwrap_or(0)
    }
}
