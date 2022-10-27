crate::solution!();

use std::collections::HashSet;

impl Solution {
    pub fn odd_cells(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
        let mut odd_cells = HashSet::new();

        indices.iter().for_each(|index| {
            (0..n)
                .map(|col| (index[0], col))
                .chain((0..m).map(|row| (row, index[1])))
                .for_each(|pos| {
                    if !odd_cells.insert(pos) {
                        odd_cells.remove(&pos);
                    }
                });
        });

        odd_cells.len() as i32
    }
}
