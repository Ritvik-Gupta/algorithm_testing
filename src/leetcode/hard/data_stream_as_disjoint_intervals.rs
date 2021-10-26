#[derive(PartialOrd, Ord)]
struct RangeSummary(i32, i32);

impl PartialEq for RangeSummary {
    fn eq(&self, other: &Self) -> bool {
        self.0 < other.0 && other.1 < self.1
    }
}

impl Eq for RangeSummary {}

use std::collections::BTreeSet;

#[allow(dead_code)]
struct SummaryRanges(BTreeSet<RangeSummary>);

impl SummaryRanges {
    #[allow(dead_code)]
    fn new() -> Self {
        let mut a = BTreeSet::new();
        a.insert(RangeSummary(1, 1));
        panic!();
    }

    #[allow(dead_code)]
    fn add_num(&self, _val: i32) {
        panic!();
    }

    #[allow(dead_code)]
    fn get_intervals(&self) -> Vec<Vec<i32>> {
        panic!();
    }
}
