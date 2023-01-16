crate::solution!();

macro_rules! i {
    (st $interval: expr) => {
        $interval[0]
    };
    (en $interval: expr) => {
        $interval[1]
    };
}

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let n = intervals.len();
        let (mut i, mut res) = (0, Vec::new());

        while i < n && i!(en intervals[i]) < i!(st new_interval) {
            res.push(intervals[i].clone());
            i += 1;
        }

        while i < n && i!(en new_interval) >= i!(st intervals[i]) {
            i!(st new_interval) = i32::min(i!(st new_interval), i!(st intervals[i]));
            i!(en new_interval) = i32::max(i!(en new_interval), i!(en intervals[i]));
            i += 1;
        }
        res.push(new_interval);

        while i < n {
            res.push(intervals[i].clone());
            i += 1;
        }

        res
    }
}
