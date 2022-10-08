crate::solution!();

use std::cmp::Ordering::*;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();

        let mut closest_sum = nums[0] + nums[1] + nums[2];
        for pivot_idx in 0..nums.len() {
            let (mut l, mut r) = (pivot_idx + 1, nums.len() - 1);
            while l < r {
                let sum = nums[pivot_idx] + nums[l] + nums[r];

                if i32::abs(sum - target) < i32::abs(closest_sum - target) {
                    closest_sum = sum;
                }
                match sum.cmp(&target) {
                    Less => l += 1,
                    Greater => r -= 1,
                    Equal => return closest_sum,
                }
            }
        }
        closest_sum
    }
}
