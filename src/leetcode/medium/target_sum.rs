crate::leetcode::solution!();

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let total = nums.iter().sum::<i32>() as usize;
        if target.abs() as usize > total {
            return 0;
        }

        let sum_space = 2 * total + 1;

        let mut prev_level_dp = vec![0; sum_space];
        let mut level_dp = vec![0; sum_space];

        prev_level_dp[total + nums[0] as usize] += 1;
        prev_level_dp[total - nums[0] as usize] += 1;

        for i in 1..nums.len() {
            prev_level_dp
                .iter_mut()
                .enumerate()
                .filter(|(_, value)| **value > 0)
                .for_each(|(sum, value)| {
                    level_dp[sum + nums[i] as usize] += *value;
                    level_dp[sum - nums[i] as usize] += *value;
                    *value = 0;
                });
            std::mem::swap(&mut prev_level_dp, &mut level_dp);
        }

        prev_level_dp[total + target as usize]
    }
}
