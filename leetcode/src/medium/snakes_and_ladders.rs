crate::solution!();

use std::collections::VecDeque;

#[derive(Default, Clone)]
struct Cell {
    jump_to: Option<usize>,
    is_visited: bool,
}

impl Solution {
    pub fn snakes_and_ladders(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let grid_size = n * n;
        let mut board = vec![Cell::default(); grid_size];

        let (mut i, mut j, mut index, mut is_right_trav) = (n - 1, 0, 0, true);

        while index < grid_size {
            board[index].jump_to = match grid[i][j] {
                -1 => None,
                jump_cell => Some(jump_cell as usize - 1),
            };
            index += 1;

            if (is_right_trav && j == n - 1) || (!is_right_trav && j == 0) {
                is_right_trav = !is_right_trav;
                i -= 1;
            } else {
                j = if is_right_trav { j + 1 } else { j - 1 };
            }
        }

        let last_cell = grid_size - 1;
        let mut queue = VecDeque::new();

        let start = board[0].jump_to.unwrap_or(0);
        queue.push_back((start, 0));
        board[start].is_visited = true;

        while let Some((curr, step)) = queue.pop_front() {
            if curr == last_cell {
                return step;
            }

            (curr + 1..=usize::min(curr + 6, last_cell)).for_each(|next_cell| {
                let next_cell = board[next_cell].jump_to.unwrap_or(next_cell);
                if !board[next_cell].is_visited {
                    board[next_cell].is_visited = true;
                    queue.push_back((next_cell, step + 1));
                }
            });
        }
        -1
    }
}
