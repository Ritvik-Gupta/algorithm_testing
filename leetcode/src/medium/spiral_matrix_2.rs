crate::solution!();

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
use Direction::*;

impl Direction {
    fn next(&mut self) {
        *self = match self {
            RIGHT => DOWN,
            DOWN => LEFT,
            LEFT => UP,
            UP => RIGHT,
        };
    }
}

struct Vector(usize, usize);

impl std::ops::Add<&Direction> for &Vector {
    type Output = Vector;

    fn add(self, direction: &Direction) -> Self::Output {
        match direction {
            UP => Vector(self.0 - 1, self.1),
            DOWN => Vector(self.0 + 1, self.1),
            LEFT => Vector(self.0, self.1 - 1),
            RIGHT => Vector(self.0, self.1 + 1),
        }
    }
}

struct SquareMatrix(Vec<Vec<i32>>);

const DEFAULT_ELM: i32 = 0;

impl SquareMatrix {
    fn new(matrix_size: usize) -> Self {
        Self(
            (0..matrix_size)
                .map(|_| Vec::with_capacity(matrix_size))
                .collect(),
        )
    }

    fn get_mut(&mut self, position: &Vector) -> Option<&mut i32> {
        let row = self.0.get_mut(position.0)?;
        if position.1 >= row.capacity() {
            return None;
        }

        while position.1 >= row.len() {
            row.push(DEFAULT_ELM);
        }
        Some(&mut row[position.1])
    }
}

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut matrix = SquareMatrix::new(n as usize);
        let (mut position, mut direction, mut store) = (Vector(0, 0), RIGHT, 1);

        while store <= n * n {
            *matrix.get_mut(&position).unwrap() = store;
            store += 1;

            let mut next_position = &position + &direction;
            if matrix
                .get_mut(&next_position)
                .map(|&mut elm| elm != DEFAULT_ELM)
                .unwrap_or(true)
            {
                direction.next();
                next_position = &position + &direction;
            }
            position = next_position;
        }
        matrix.0
    }
}
