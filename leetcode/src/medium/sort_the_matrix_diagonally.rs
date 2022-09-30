crate::solution!();

impl Solution {
    pub fn diagonal_sort(mut matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::{cmp::Reverse, collections::BinaryHeap};

        let (m, n) = (matrix.len(), matrix[0].len());
        let mut diagonal_heaps = vec![BinaryHeap::new(); m + n + 1];
        let diagonal_offset = n;

        for i in 0..m {
            for j in 0..n {
                diagonal_heaps[i - j + diagonal_offset].push(Reverse(matrix[i][j]));
            }
        }

        for i in 0..m {
            for j in 0..n {
                matrix[i][j] = diagonal_heaps[i - j + diagonal_offset].pop().unwrap().0;
            }
        }

        matrix
    }
}
