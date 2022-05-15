struct Interval(i32, i32);

impl Interval {
    fn contains(&self, val: i32) -> bool {
        self.0 <= val && val <= self.1
    }
}

impl PartialEq for Interval {
    fn eq(&self, other: &Self) -> bool {
        self.contains(other.0)
            || self.contains(other.1)
            || other.contains(self.0)
            || other.contains(self.1)
    }
}

impl Eq for Interval {}

use std::cmp::Ordering::{self, Equal, Greater, Less};

impl PartialOrd for Interval {
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

impl Ord for Interval {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

use std::collections::BTreeSet;

#[allow(dead_code)]
struct CountIntervals(BTreeSet<Interval>);

impl CountIntervals {
    #[allow(dead_code)]
    fn new() -> Self {
        Self(BTreeSet::new())
    }

    #[allow(dead_code)]
    fn add(&mut self, mut left: i32, mut right: i32) {
        while let Some(intersecting_interval) = self.0.take(&Interval(left, right)) {
            left = i32::min(left, intersecting_interval.0);
            right = i32::max(right, intersecting_interval.1);
        }

        self.0.insert(Interval(left, right));
    }

    #[allow(dead_code)]
    fn count(&self) -> i32 {
        self.0
            .iter()
            .map(|interval| interval.1 - interval.0 + 1)
            .sum()
    }
}
