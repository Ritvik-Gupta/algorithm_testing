crate::leetcode::solution!();

const ZERO: char = '0';

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let n = n as usize;
        let m = m as usize;

        let mut dp = vec![vec![0; n + 1]; m + 1];

        for (zeroes, ones) in strs.iter().map(|binary| {
            let num_zeroes = binary.chars().filter(|&ch| ch == ZERO).count();
            (num_zeroes, binary.len() - num_zeroes)
        }) {
            for i in (zeroes..=m).rev() {
                for j in (ones..=n).rev() {
                    dp[i][j] = i32::max(dp[i][j], 1 + dp[i - zeroes][j - ones]);
                }
            }
        }

        dp[m][n]
    }
}
