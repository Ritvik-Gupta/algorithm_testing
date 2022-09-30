crate::solution!();

struct Matrix {
    matrix: Vec<Vec<i32>>,
    num_rows: usize,
    num_cols: usize,
}

impl Matrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let (num_rows, num_cols) = (matrix.len(), matrix[0].len());
        Self {
            matrix,
            num_rows,
            num_cols,
        }
    }

    fn size(&self) -> usize {
        self.num_rows * self.num_cols
    }

    fn into_coordinate(&self, pos: usize) -> (usize, usize) {
        (pos / self.num_cols, pos % self.num_cols)
    }
}

impl std::ops::Index<usize> for Matrix {
    type Output = i32;

    fn index(&self, pos: usize) -> &Self::Output {
        let coordinate = self.into_coordinate(pos);
        &self.matrix[coordinate.0][coordinate.1]
    }
}

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        use std::cmp::Ordering::{Equal, Greater, Less};

        let matrix = Matrix::new(matrix);

        let (mut lower, mut upper) = (0, matrix.size() - 1);
        while lower <= upper {
            let mid = (lower + upper) / 2;
            match target.cmp(&matrix[mid]) {
                Less if mid == 0 => return false,
                Less => upper = mid - 1,
                Greater => lower = mid + 1,
                Equal => return true,
            }
        }
        false
    }
}
