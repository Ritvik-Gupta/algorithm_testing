crate::leetcode::solution!();

use std::collections::VecDeque;

impl Solution {
    pub fn unique_paths_with_obstacles(mut grid: Vec<Vec<i32>>) -> i32 {
        let (num_rows, num_cols) = (grid.len(), grid[0].len());

        if grid[0][0] == 1 || grid[num_rows - 1][num_cols - 1] == 1 {
            return 0;
        }

        grid[num_rows - 1][num_cols - 1] = -1;

        let mut node_queue = VecDeque::with_capacity(usize::max(num_rows, num_cols));
        node_queue.push_back((num_rows - 1, num_cols - 1));

        while let Some((pos_x, pos_y)) = node_queue.pop_front() {
            if pos_x > 0 && grid[pos_x - 1][pos_y] != 1 {
                if grid[pos_x - 1][pos_y] == 0 {
                    node_queue.push_back((pos_x - 1, pos_y));
                }
                grid[pos_x - 1][pos_y] += grid[pos_x][pos_y];
            }

            if pos_y > 0 && grid[pos_x][pos_y - 1] != 1 {
                if grid[pos_x][pos_y - 1] == 0 {
                    node_queue.push_back((pos_x, pos_y - 1));
                }
                grid[pos_x][pos_y - 1] += grid[pos_x][pos_y];
            }
        }

        -grid[0][0]
    }
}
