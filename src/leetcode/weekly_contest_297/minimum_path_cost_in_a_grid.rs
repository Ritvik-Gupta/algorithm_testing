crate::leetcode::solution!();

impl Solution {
    pub fn min_path_cost(grid: Vec<Vec<i32>>, move_costs: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());

        let mut next_level_dp = vec![0; n];
        let mut level_dp = vec![0; n];

        for j in 0..n {
            level_dp[j] = grid[m - 1][j];
        }

        for i in (0..m - 1).rev() {
            std::mem::swap(&mut next_level_dp, &mut level_dp);

            for j in 0..n {
                level_dp[j] = grid[i][j]
                    + move_costs[grid[i][j] as usize]
                        .iter()
                        .enumerate()
                        .map(|(k, &cost)| cost + next_level_dp[k])
                        .min()
                        .unwrap();
            }
        }

        *level_dp.iter().min().unwrap()
    }
}
