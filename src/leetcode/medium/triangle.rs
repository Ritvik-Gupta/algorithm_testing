crate::leetcode::solution!();

impl Solution {
    pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
        for i in (0..triangle.len() - 1).rev() {
            for j in 0..=i {
                triangle[i][j] += std::cmp::min(triangle[i + 1][j], triangle[i + 1][j + 1]);
            }
        }

        triangle[0][0]
    }
}
