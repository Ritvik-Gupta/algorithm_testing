crate::solution!();

use std::collections::HashMap;

impl Solution {
    pub fn count_quadruplets(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let len = nums.len();

        let mut count = HashMap::new();
        count.insert(nums[len - 1] - nums[len - 2], 1);

        for b in (1..=len - 3).rev() {
            for a in (0..=b - 1).rev() {
                res += count.get(&(nums[a] + nums[b])).unwrap_or(&0);
            }
            for x in (b + 1..=len - 1).rev() {
                *count.entry(nums[x] - nums[b]).or_insert(0) += 1;
            }
        }

        res
    }
}
