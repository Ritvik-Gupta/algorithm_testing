pub struct Solution;

struct Grid<'a> {
    matrix: &'a mut Vec<Vec<i32>>,
    num_rows: usize,
    num_cols: usize,
}

use std::{
    cmp::{max, min},
    ops::Range,
};

impl<'a> Grid<'a> {
    fn new(matrix: &'a mut Vec<Vec<i32>>) -> Self {
        let (num_rows, num_cols) = (matrix.len(), matrix[0].len());
        Grid {
            matrix,
            num_rows,
            num_cols,
        }
    }

    fn range_x_neighbours(&self, x: usize) -> Range<usize> {
        (max(0, x as isize - 1) as usize)..min(x + 2, self.num_rows)
    }

    fn range_y_neighbours(&self, y: usize) -> Range<usize> {
        (max(0, y as isize - 1) as usize)..min(y + 2, self.num_cols)
    }

    fn is_alive_at(&self, x: usize, y: usize) -> bool {
        self.matrix[x][y] % 10 == 1
    }

    fn live_neighbours(&self, x: usize, y: usize) -> u8 {
        self.range_x_neighbours(x).fold(0, |acc, i| {
            acc + self
                .range_y_neighbours(y)
                .fold(0, |acc, j| acc + self.is_alive_at(i, j) as u8)
        })
    }

    fn next_iteration(&mut self) {
        for i in 0..self.num_rows {
            for j in 0..self.num_cols {
                self.matrix[i][j] += match self.live_neighbours(i, j) {
                    3 => 10,
                    4 if self.is_alive_at(i, j) => 10,
                    _ => 0,
                }
            }
        }

        self.matrix
            .iter_mut()
            .for_each(|row| row.iter_mut().for_each(|elm| *elm /= 10));
    }
}

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let mut grid = Grid::new(board);
        grid.next_iteration();
    }
}
