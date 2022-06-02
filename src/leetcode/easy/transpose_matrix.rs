crate::leetcode::solution!();

impl Solution {
    pub fn transpose(mut matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (num_rows, num_cols) = (matrix.len(), matrix[0].len());

        if num_rows == num_cols {
            for i in 0..num_rows {
                for j in 0..i {
                    let temp = matrix[i][j];
                    matrix[i][j] = matrix[j][i];
                    matrix[j][i] = temp;
                }
            }
            matrix
        } else {
            let mut transposed = Vec::with_capacity(num_cols);
            for j in 0..num_cols {
                transposed.push(Vec::with_capacity(num_rows));
                for i in 0..num_rows {
                    transposed[j].push(matrix[i][j]);
                }
            }
            transposed
        }
    }
}
