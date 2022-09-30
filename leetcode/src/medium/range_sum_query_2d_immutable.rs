pub struct NumMatrix {
    num_rows: usize,
    num_cols: usize,
    matrix: Vec<Vec<i32>>,
}

type Vector = (usize, usize);

impl NumMatrix {
    pub fn new(matrix: Vec<Vec<i32>>) -> Self {
        let mut matrix = Self {
            num_rows: matrix.len(),
            num_cols: matrix[0].len(),
            matrix,
        };
        matrix.update_to_cumulative();
        matrix
    }

    fn get_val(&self, (x, y): Vector) -> i32 {
        *self.matrix.get(x).and_then(|row| row.get(y)).unwrap_or(&0)
    }

    fn update_to_cumulative(&mut self) {
        for i in 0..self.num_rows {
            for j in 0..self.num_cols {
                self.matrix[i][j] += self.get_val((i - 1, j));
            }
        }

        for i in 0..self.num_rows {
            for j in 0..self.num_cols {
                self.matrix[i][j] += self.get_val((i, j - 1));
            }
        }
    }

    pub fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let (start_row, start_col, end_row, end_col) =
            (row1 as usize, col1 as usize, row2 as usize, col2 as usize);

        self.get_val((end_row, end_col))
            - self.get_val((start_row - 1, end_col))
            - self.get_val((end_row, start_col - 1))
            + self.get_val((start_row - 1, start_col - 1))
    }
}
