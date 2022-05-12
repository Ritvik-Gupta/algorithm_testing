crate::leetcode::solution!();

use std::collections::HashSet;

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let mut seen_nums = HashSet::with_capacity(nums.len() / 10);

        let mut subarray_sum = 0;
        let mut subarray_start = 0;
        let mut max_subarray_sum = 0;
        for &num in nums.iter() {
            while seen_nums.contains(&num) {
                seen_nums.remove(&nums[subarray_start]);
                subarray_sum -= nums[subarray_start];
                subarray_start += 1;
            }

            seen_nums.insert(num);
            subarray_sum += num;
            max_subarray_sum = max_subarray_sum.max(subarray_sum);
        }

        max_subarray_sum
    }
}
