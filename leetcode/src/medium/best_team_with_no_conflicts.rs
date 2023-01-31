crate::solution!();

use std::cmp::Reverse;

impl Solution {
    pub fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
        let n = scores.len();
        let mut players = ages.into_iter().zip(scores.into_iter()).collect::<Vec<_>>();

        players.sort_by_key(|&key| Reverse(key));

        let mut res = 0;
        let mut dp = vec![0; n];
        for i in 0..n {
            let score = players[i].1;
            dp[i] = score;

            for j in 0..i {
                if players[j].1 >= players[i].1 {
                    dp[i] = i32::max(dp[i], dp[j] + score);
                }
            }
            res = i32::max(res, dp[i]);
        }
        res
    }
}
