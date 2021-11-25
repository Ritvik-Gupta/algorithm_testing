crate::leetcode::solution!();

impl Solution {
    pub fn diagonal_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let (mut sum, matrix_size) = (0, matrix.len());
        for (j, matrix_row) in matrix.iter().enumerate() {
            sum += matrix_row[j] + matrix_row[matrix_size - 1 - j];
        }
        if matrix_size % 2 == 1 {
            sum -= matrix[matrix_size / 2][matrix_size / 2];
        }
        sum
    }
}
