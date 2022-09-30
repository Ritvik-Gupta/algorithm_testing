crate::solution!();

enum Direction {
    U,
    D,
    L,
    R,
}

use Direction::{D, L, R, U};

static NEIGHBOUR_DIRS: [Direction; 4] = [U, D, L, R];

struct Vector(usize, usize);

impl std::ops::Add<&Direction> for &Vector {
    type Output = Vector;

    fn add(self, direction: &Direction) -> Self::Output {
        match direction {
            U => Vector(self.0 - 1, self.1),
            D => Vector(self.0 + 1, self.1),
            L => Vector(self.0, self.1 - 1),
            R => Vector(self.0, self.1 + 1),
        }
    }
}

impl Vector {
    fn neighbours(self) -> impl Iterator<Item = Vector> {
        NEIGHBOUR_DIRS
            .iter()
            .map(move |direction| &self + direction)
    }
}

const X_TOKEN: char = 'X';
const O_TOKEN: char = 'O';
const MID_FLIPPED_TOKEN: char = 'M';

struct ClusterBoard<'a>(&'a mut Vec<Vec<char>>);

impl<'a> ClusterBoard<'a> {
    fn get(&self, pos: &Vector) -> Option<char> {
        Some(*self.0.get(pos.0)?.get(pos.1)?)
    }

    fn explore_o_cluster(&mut self, pos: Vector) {
        if let Some(O_TOKEN) = self.get(&pos) {
            self.0[pos.0][pos.1] = MID_FLIPPED_TOKEN;
            pos.neighbours()
                .for_each(|neighbour_pos| self.explore_o_cluster(neighbour_pos));
        }
    }
}

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let (num_rows, num_cols) = (board.len(), board[0].len());
        let mut cluster_board = ClusterBoard(board);

        for j in 0..num_cols {
            cluster_board.explore_o_cluster(Vector(0, j));
            cluster_board.explore_o_cluster(Vector(num_rows - 1, j));
        }
        for i in 0..num_rows {
            cluster_board.explore_o_cluster(Vector(i, 0));
            cluster_board.explore_o_cluster(Vector(i, num_cols - 1));
        }

        board.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|elm| {
                *elm = match *elm {
                    O_TOKEN => X_TOKEN,
                    MID_FLIPPED_TOKEN => O_TOKEN,
                    token => token,
                };
            });
        });
    }
}
