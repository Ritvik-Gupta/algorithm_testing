pub struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let (num_rows, num_cols) = (matrix.len(), matrix[0].len());
        let mut positions_to_update: Vec<(usize, usize)> = Vec::new();

        matrix.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, &elm)| {
                if elm == 0 {
                    positions_to_update.push((i, j));
                }
            });
        });

        positions_to_update.into_iter().for_each(|(i, j)| {
            (0..num_rows).for_each(|i| matrix[i][j] = 0);
            (0..num_cols).for_each(|j| matrix[i][j] = 0);
        });
    }
}
