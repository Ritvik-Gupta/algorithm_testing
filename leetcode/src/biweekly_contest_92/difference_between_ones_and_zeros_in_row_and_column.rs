crate::solution!();

impl Solution {
    pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (grid.len(), grid[0].len());
        let (mut row_records, mut col_records) = (vec![0; m], vec![0; n]);

        for x in 0..m {
            for y in 0..n {
                row_records[x] += 2 * grid[x][y] - 1;
                col_records[y] += 2 * grid[x][y] - 1;
            }
        }

        (0..m)
            .map(|x| (0..n).map(|y| row_records[x] + col_records[y]).collect())
            .collect()
    }
}
