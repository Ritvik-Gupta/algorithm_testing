crate::solution!();

use std::collections::HashSet;

impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        let mut res = -1;
        let mut seen_nums = HashSet::with_capacity(nums.len());

        for num in nums {
            if seen_nums.contains(&(-num)) {
                res = i32::max(res, num.abs());
            }
            seen_nums.insert(num);
        }

        res
    }
}
