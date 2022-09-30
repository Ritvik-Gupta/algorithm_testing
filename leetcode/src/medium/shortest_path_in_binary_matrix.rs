crate::solution!();

use std::collections::VecDeque;

struct GridMap {
    grid: Vec<Vec<i32>>,
    num_rows: usize,
    num_cols: usize,
}

impl GridMap {
    fn new(grid: Vec<Vec<i32>>) -> Self {
        let (num_rows, num_cols) = (grid.len(), grid[0].len());
        Self {
            grid,
            num_rows,
            num_cols,
        }
    }

    fn size(&self) -> usize {
        self.num_rows * self.num_cols
    }

    fn mark(&mut self, row: usize, col: usize) {
        self.grid[row][col] = 1;
    }

    fn is_valid(&self, row: usize, col: usize) -> bool {
        self.grid
            .get(row)
            .and_then(|row| row.get(col))
            .map(|&x| x == 0)
            .unwrap_or(false)
    }
}

impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = GridMap::new(grid);
        let mut node_queue = VecDeque::with_capacity(grid.size() / 5);

        if grid.is_valid(0, 0) {
            node_queue.push_back((0, 0, 1));
            grid.mark(0, 0);
        }

        while let Some((row, col, distance)) = node_queue.pop_front() {
            if row == grid.num_rows - 1 && col == grid.num_cols - 1 {
                return distance;
            }

            for next_row in [row - 1, row, row + 1] {
                for next_col in [col - 1, col, col + 1] {
                    if grid.is_valid(next_row, next_col) {
                        node_queue.push_back((next_row, next_col, distance + 1));
                        grid.mark(next_row, next_col);
                    }
                }
            }
        }

        -1
    }
}
