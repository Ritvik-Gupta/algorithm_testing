crate::solution!();

use std::collections::HashSet;

fn parenthesis_value(paren: char) -> i32 {
    if paren == '(' {
        1
    } else {
        -1
    }
}

struct ParenGrid {
    grid: Vec<Vec<char>>,
    num_rows: usize,
    num_cols: usize,
    visited_for_balance: Vec<Vec<HashSet<i32>>>,
}

impl ParenGrid {
    fn new(grid: Vec<Vec<char>>) -> Self {
        let (num_rows, num_cols) = (grid.len(), grid[0].len());
        Self {
            grid,
            num_rows,
            num_cols,
            visited_for_balance: vec![vec![Default::default(); num_cols]; num_rows],
        }
    }

    fn check_paren_validity(&mut self) -> bool {
        self.dfs_for_paren_validity(0, 0, 0)
    }

    fn dfs_for_paren_validity(&mut self, row: usize, col: usize, mut balance: i32) -> bool {
        if row >= self.num_rows || col >= self.num_cols {
            return false;
        }

        balance += parenthesis_value(self.grid[row][col]);

        if balance < 0
            || balance > (self.num_rows + self.num_cols) as i32 / 2
            || self.visited_for_balance[row][col].contains(&balance)
        {
            return false;
        }
        self.visited_for_balance[row][col].insert(balance);

        if row == self.num_rows - 1 && col == self.num_cols - 1 {
            return balance == 0;
        }

        self.dfs_for_paren_validity(row + 1, col, balance)
            || self.dfs_for_paren_validity(row, col + 1, balance)
    }
}

impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<char>>) -> bool {
        if grid.len() & 1 == grid[0].len() & 1 {
            return false;
        }
        ParenGrid::new(grid).check_paren_validity()
    }
}
