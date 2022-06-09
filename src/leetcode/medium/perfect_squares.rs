crate::leetcode::solution!();

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let target_num = n as usize;
        let square_limit = (n as f64).sqrt().floor() as usize;

        let mut level_dp: Vec<_> = (0..=n).collect();

        for square_val in (1..=square_limit).map(|x| x.pow(2)) {
            for num in square_val..=target_num {
                level_dp[num] = i32::min(level_dp[num], 1 + level_dp[num - square_val]);
            }
        }

        level_dp[target_num]
    }
}
