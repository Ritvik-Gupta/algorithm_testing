crate::solution!();

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let (num_rows, num_cols) = (matrix.len(), matrix[0].len());
        let (mut spiral, mut row_start, mut row_end, mut col_start, mut col_end) = (
            Vec::with_capacity(num_rows * num_cols),
            0,
            num_rows - 1,
            0,
            num_cols - 1,
        );

        macro_rules! has_spiral_finished {
            () => {
                if spiral.len() == num_rows * num_cols {
                    return spiral;
                }
            };
        }
        loop {
            has_spiral_finished!();
            spiral.extend((col_start..=col_end).map(|j| matrix[row_start][j]));
            row_start += 1;

            has_spiral_finished!();
            spiral.extend((row_start..=row_end).map(|i| matrix[i][col_end]));
            col_end -= 1;

            has_spiral_finished!();
            spiral.extend((col_start..=col_end).rev().map(|j| matrix[row_end][j]));
            row_end -= 1;

            has_spiral_finished!();
            spiral.extend((row_start..=row_end).rev().map(|i| matrix[i][col_start]));
            col_start += 1;
        }
    }
}
