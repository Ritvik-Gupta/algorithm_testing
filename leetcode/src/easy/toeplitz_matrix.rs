crate::solution!();

impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        (1..matrix.len())
            .flat_map(|i| (1..matrix[0].len()).map(move |j| (i, j)))
            .all(|(i, j)| matrix[i][j] == matrix[i - 1][j - 1])
    }
}
