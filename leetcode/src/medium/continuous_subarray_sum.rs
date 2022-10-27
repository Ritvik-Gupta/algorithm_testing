crate::solution!();

use std::collections::{hash_map::Entry::*, HashMap};

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut seen_prefix_sums = HashMap::new();
        let mut sum = 0;

        seen_prefix_sums.insert(sum, 0);

        nums.iter().enumerate().any(|(i, num)| {
            sum += num;

            match seen_prefix_sums.entry(sum % k) {
                Vacant(entry) => {
                    entry.insert(i + 1);
                }
                Occupied(entry) if *entry.get() < i => return true,
                _ => {}
            }
            false
        })
    }
}
