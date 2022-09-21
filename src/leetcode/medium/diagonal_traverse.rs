crate::leetcode::solution!();

enum Diagonal {
    Up,
    Dw,
}

use Diagonal::*;

impl Diagonal {
    fn flip(&mut self) {
        *self = match self {
            Up => Dw,
            Dw => Up,
        }
    }
}

impl Solution {
    pub fn find_diagonal_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut result = Vec::with_capacity(m * n);

        let (mut x, mut y) = (0, 0);
        let mut diagonal = Up;

        while x < m && y < n {
            result.push(matrix[x][y]);

            let (new_x, new_y) = match diagonal {
                Up => (x.wrapping_sub(1), y + 1),
                Dw => (x + 1, y.wrapping_sub(1)),
            };

            if new_x < m && new_y < n {
                x = new_x;
                y = new_y;
            } else {
                let (delta_x, delta_y) = match diagonal {
                    Up if y + 1 == n => (1, 0),
                    Up => (0, 1),
                    Dw if x + 1 == m => (0, 1),
                    Dw => (1, 0),
                };

                x += delta_x;
                y += delta_y;
                diagonal.flip();
            }
        }

        result
    }
}
