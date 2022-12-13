crate::solution!();

impl Solution {
    pub fn min_falling_path_sum(mut matrix: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (matrix.len(), matrix[0].len());
        for x in (0..m - 1).rev() {
            for y in 0..n {
                matrix[x][y] += [y.wrapping_sub(1), y, y + 1]
                    .iter()
                    .filter(|&&y| y < n)
                    .map(|&y| matrix[x + 1][y])
                    .min()
                    .unwrap();
            }
        }
        *matrix[0].iter().min().unwrap()
    }
}
