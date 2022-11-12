crate::solution!();

use std::collections::HashSet;

impl Solution {
    pub fn distinct_averages(mut nums: Vec<i32>) -> i32 {
        nums.sort();

        let mut distinct_sums = HashSet::new();
        for i in 0..nums.len() / 2 {
            distinct_sums.insert(nums[i] + nums[nums.len() - 1 - i]);
        }
        distinct_sums.len() as i32
    }
}
