crate::leetcode::solution!();

impl Solution {
    pub fn combination_sum4(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();

        let target = target as usize;
        let mut dp = vec![0; target + 1];
        dp[0] = 1;

        for i in 1..=target {
            nums.iter()
                .map(|&x| x as usize)
                .take_while(|&num| num <= i)
                .for_each(|num| dp[i] += dp[i - num]);
        }

        dp[target]
    }
}
