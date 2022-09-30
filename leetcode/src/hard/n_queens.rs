crate::solution!();

type Position = (usize, usize);

struct Board(Vec<Vec<u8>>);

impl std::ops::Index<Position> for Board {
    type Output = u8;

    fn index(&self, (row, col): Position) -> &Self::Output {
        self.0
            .get(row)
            .and_then(|row| row.get(col))
            .unwrap_or(&b'.')
    }
}

impl Into<Vec<String>> for &mut Board {
    fn into(self) -> Vec<String> {
        self.0
            .iter()
            .map(|row| String::from_utf8_lossy(row).into_owned())
            .collect()
    }
}

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;

        let mut board = Board(vec![vec![b'.'; n]; n]);
        let mut result = Vec::new();

        recursive_compute_board_config(&mut board, 0, &mut result);
        return result;
    }
}

fn recursive_compute_board_config(board: &mut Board, row: usize, result: &mut Vec<Vec<String>>) {
    if row == board.0.len() {
        result.push(board.into());
        return;
    }

    for col in 0..board.0.len() {
        if is_safe(board, row, col) {
            board.0[row][col] = b'Q';
            recursive_compute_board_config(board, row + 1, result);
            board.0[row][col] = b'.';
        }
    }
}

fn is_safe(board: &Board, row: usize, col: usize) -> bool {
    for i in 0..board.0.len() {
        if board[(i, col)] == b'Q'
            || board[(row - i, col - i)] == b'Q'
            || board[(row - i, col + i)] == b'Q'
        {
            return false;
        }
    }
    true
}
