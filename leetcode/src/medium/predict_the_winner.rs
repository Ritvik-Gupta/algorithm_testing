crate::solution!();

impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        let mut dp = nums.clone();

        (0..nums.len())
            .rev()
            .flat_map(|s| (s + 1..nums.len()).map(move |e| (s, e)))
            .for_each(|(s, e)| dp[e] = i32::max(nums[s] - dp[e], nums[e] - dp[e - 1]));

        dp[nums.len() - 1] >= 0
    }
}
