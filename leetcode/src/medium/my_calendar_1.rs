struct Interval(i32, i32);

impl Interval {
    fn contains(&self, val: i32) -> bool {
        self.0 <= val && val <= self.1
    }
}

impl PartialEq for Interval {
    fn eq(&self, other: &Self) -> bool {
        other.contains(self.0)
            || other.contains(self.1)
            || self.contains(other.0)
            || self.contains(other.1)
    }
}

impl Eq for Interval {}

use std::cmp::Ordering::{self, *};

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(if self == other {
            Equal
        } else if self.0 > other.1 {
            Greater
        } else if other.0 > self.1 {
            Less
        } else {
            unreachable!()
        })
    }
}

use std::collections::BTreeSet;

#[allow(dead_code)]
struct MyCalendar {
    bookings: BTreeSet<Interval>,
}

impl Ord for Interval {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl MyCalendar {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            bookings: BTreeSet::new(),
        }
    }

    #[allow(dead_code)]
    fn book(&mut self, start: i32, end: i32) -> bool {
        let booking = Interval(start, end - 1);

        if self.bookings.contains(&booking) {
            return false;
        }

        self.bookings.insert(booking);
        true
    }
}
