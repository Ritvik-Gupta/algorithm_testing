struct RangeSummary(i32, i32);

impl RangeSummary {
    fn contains(&self, val: i32) -> bool {
        self.0 <= val && val <= self.1
    }
}

impl Into<Vec<i32>> for &RangeSummary {
    fn into(self) -> Vec<i32> {
        vec![self.0, self.1]
    }
}

impl PartialEq for RangeSummary {
    fn eq(&self, other: &Self) -> bool {
        other.contains(self.0) || other.contains(self.1)
    }
}

impl Eq for RangeSummary {}

use std::cmp::Ordering::{self, Equal, Greater, Less};

impl PartialOrd for RangeSummary {
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

impl Ord for RangeSummary {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

use std::collections::BTreeSet;

struct SummaryRanges(BTreeSet<RangeSummary>);

impl SummaryRanges {
    #[allow(dead_code)]
    fn new() -> Self {
        Self(BTreeSet::new())
    }

    #[allow(dead_code)]
    fn add_num(&mut self, val: i32) {
        let (mut lower_bound, mut upper_bound) = (val, val);

        if !self.0.contains(&RangeSummary(lower_bound, upper_bound)) {
            if let Some(lower_range) = self.0.take(&RangeSummary(lower_bound - 1, upper_bound)) {
                lower_bound = lower_range.0;
            }
            if let Some(upper_range) = self.0.take(&RangeSummary(lower_bound, upper_bound + 1)) {
                upper_bound = upper_range.1;
            }
            self.0.insert(RangeSummary(lower_bound, upper_bound));
        }
    }

    #[allow(dead_code)]
    fn get_intervals(&self) -> Vec<Vec<i32>> {
        self.0.iter().map(Into::into).collect()
    }
}
