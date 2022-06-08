crate::leetcode::solution!();

impl Solution {
    pub fn calculate_minimum_hp(mut dungeon: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (dungeon.len(), dungeon[0].len());

        (0..m)
            .rev()
            .flat_map(|i| (0..n).rev().map(move |j| (i, j)))
            .for_each(|(i, j)| {
                dungeon[i][j] += match (i == m - 1, j == n - 1) {
                    (true, true) => 0,
                    (is_last_row, is_last_col) => i32::max(
                        if is_last_row {
                            i32::MIN
                        } else {
                            dungeon[i + 1][j]
                        },
                        if is_last_col {
                            i32::MIN
                        } else {
                            dungeon[i][j + 1]
                        },
                    ),
                };
                dungeon[i][j] = i32::min(0, dungeon[i][j]);
            });

        1 + dungeon[0][0].abs()
    }
}
