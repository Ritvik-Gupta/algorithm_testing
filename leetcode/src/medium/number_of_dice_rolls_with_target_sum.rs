crate::solution!();

const BIG_MOD: i32 = 1000000007;

impl Solution {
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        let (n, k, target) = (n as usize, k as usize, target as usize);

        let mut prv_dp = vec![0; target + 1];
        let mut lvl_dp = vec![0; target + 1];
        prv_dp[0] = 1;

        for _ in 1..=n {
            for j in 0..=target {
                lvl_dp[j] = (1..=usize::min(k, j))
                    .map(|dice_roll| prv_dp[j - dice_roll])
                    .fold(0, |acc, possibilities| (acc + possibilities) % BIG_MOD);
            }

            std::mem::swap(&mut prv_dp, &mut lvl_dp);
        }

        prv_dp[target]
    }
}
