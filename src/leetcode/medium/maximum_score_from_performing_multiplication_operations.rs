crate::leetcode::solution!();

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let m = multipliers.len();
        let mut nxt_dp = vec![0; multipliers.len() + 1];
        let mut lvl_dp = vec![0; multipliers.len() + 1];

        for opr in (0..m).rev() {
            for left in (0..=opr).rev() {
                let right = nums.len() - 1 - (opr - left);
                nxt_dp[left] = i32::max(
                    multipliers[opr] * nums[left] + lvl_dp[left + 1],
                    multipliers[opr] * nums[right] + lvl_dp[left],
                );
            }

            std::mem::swap(&mut lvl_dp, &mut nxt_dp);
        }

        lvl_dp[0]
    }
}
