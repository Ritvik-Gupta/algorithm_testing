crate::solution!();

use std::cmp::Ordering::*;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut result = Vec::new();

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let (mut l, mut r) = (i + 1, nums.len() - 1);

            while l < r {
                let sum = nums[i] + nums[l] + nums[r];

                match sum.cmp(&0) {
                    Equal => {
                        let triplet = vec![nums[i], nums[l], nums[r]];

                        while l < r && nums[l] == triplet[1] && nums[r] == triplet[2] {
                            l += 1;
                            r -= 1;
                        }

                        result.push(triplet);
                    }
                    Less => l += 1,
                    Greater => r -= 1,
                }
            }
        }

        result
    }
}
