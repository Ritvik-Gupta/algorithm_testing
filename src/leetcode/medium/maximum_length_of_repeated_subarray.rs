crate::leetcode::solution!();

impl Solution {
    pub fn find_length(fixed_nums: Vec<i32>, range_nums: Vec<i32>) -> i32 {
        let mut prev_dp = vec![0; range_nums.len() + 1];
        let mut levl_dp = vec![0; range_nums.len() + 1];
        let mut max_subarr_length = 0;

        for &num in fixed_nums.iter().rev() {
            for r in (0..range_nums.len()).rev() {
                levl_dp[r] = if num == range_nums[r] {
                    prev_dp[r + 1] + 1
                } else {
                    0
                };
                max_subarr_length = i32::max(max_subarr_length, levl_dp[r]);
            }
            std::mem::swap(&mut prev_dp, &mut levl_dp);
        }

        max_subarr_length
    }
}
