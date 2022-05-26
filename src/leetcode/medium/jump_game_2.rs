crate::leetcode::solution!();

const OUTSIDE_JUMP: i32 = 1e9 as i32;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len()];

        for i in (0..nums.len() - 1).rev() {
            dp[i] = dp[i + 1..usize::min(i + 1 + nums[i] as usize, nums.len())]
                .iter()
                .map(|&next_step| next_step + 1)
                .min()
                .unwrap_or(OUTSIDE_JUMP);
        }

        dp[0]
    }
}
