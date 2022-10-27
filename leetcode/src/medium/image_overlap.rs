crate::solution!();

use std::collections::HashMap;

type Matrix = Vec<Vec<i32>>;

fn locate_ones(matrix: &Matrix) -> Vec<(usize, usize)> {
    (0..matrix.len())
        .flat_map(|x| (0..matrix[0].len()).map(move |y| (x, y)))
        .filter(|&(x, y)| matrix[x][y] == 1)
        .collect()
}

impl Solution {
    pub fn largest_overlap(img1: Matrix, img2: Matrix) -> i32 {
        let (img1_ones, img2_ones) = (locate_ones(&img1), locate_ones(&img2));

        let mut transpose_table = HashMap::new();

        img1_ones.iter().for_each(|pos1| {
            img2_ones.iter().for_each(|pos2| {
                let transpose = (pos2.0 - pos1.0, pos2.1 - pos1.1);
                *transpose_table.entry(transpose).or_insert(0) += 1;
            });
        });

        *transpose_table.values().max().unwrap_or(&0)
    }
}
