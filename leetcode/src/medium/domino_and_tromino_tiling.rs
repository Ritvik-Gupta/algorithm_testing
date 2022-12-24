crate::solution!();

const TOP: usize = 0;
const BOTTOM: usize = 1;
const BOTH: usize = 2;

const BIG_MOD: i64 = 1000000007;

impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![[0; 3]; n + 2];
        dp[0][BOTH] += 1;

        let tiling_dependencies = [
            vec![(1, BOTTOM), (2, BOTH)],
            vec![(1, TOP), (2, BOTH)],
            vec![(1, BOTH), (2, BOTH), (1, TOP), (1, BOTTOM)],
        ];

        for i in 0..n {
            [TOP, BOTTOM, BOTH].iter().for_each(|&tile| {
                tiling_dependencies[tile]
                    .iter()
                    .for_each(|&(offset, next_tile)| {
                        dp[i + offset][next_tile] += dp[i][tile];
                        dp[i + offset][next_tile] %= BIG_MOD;
                    });
            });
        }

        dp[n][BOTH] as i32
    }
}
