crate::leetcode::solution!();

impl Solution {
    pub fn find_length(fixed_nums: Vec<i32>, nums: Vec<i32>) -> i32 {
        let mut level_dp = vec![0; nums.len()];
        let mut max_subarr_length = 0;

        for &num in fixed_nums.iter().rev() {
            let mut next_val = 0;
            for r in (0..nums.len()).rev() {
                let level_val = if num == nums[r] { next_val + 1 } else { 0 };
                next_val = level_dp[r];
                level_dp[r] = level_val;
                max_subarr_length = i32::max(max_subarr_length, level_val);
            }
        }

        max_subarr_length
    }
}
