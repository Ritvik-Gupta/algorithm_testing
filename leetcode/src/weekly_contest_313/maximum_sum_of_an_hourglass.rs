crate::solution!();

fn hourglass_sum(grid: &Vec<Vec<i32>>, (x, y): (usize, usize)) -> i32 {
    0 + 0
        + grid[x - 1][y - 1]
        + grid[x - 1][y]
        + grid[x - 1][y + 1]
        + grid[x][y]
        + grid[x + 1][y - 1]
        + grid[x + 1][y]
        + grid[x + 1][y + 1]
}

impl Solution {
    pub fn max_sum(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut max_sum = 0;

        for x in 1..m - 1 {
            let mut sum = hourglass_sum(&grid, (x, 1));
            for y in 1..n - 1 {
                max_sum = i32::max(max_sum, sum);

                if y != n - 2 {
                    sum -= grid[x][y] + grid[x - 1][y - 1] + grid[x + 1][y - 1];
                    sum += grid[x][y + 1] + grid[x - 1][y + 2] + grid[x + 1][y + 2];
                }
            }
        }

        max_sum
    }
}
