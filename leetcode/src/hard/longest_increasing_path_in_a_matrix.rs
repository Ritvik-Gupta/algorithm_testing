crate::solution!();

#[derive(Clone, Copy)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

use Direction::*;

#[derive(Clone, Copy)]
struct Position(usize, usize);

impl std::ops::Add<Direction> for Position {
    type Output = Position;

    fn add(self, direction: Direction) -> Self::Output {
        match direction {
            UP => Position(self.0 - 1, self.1),
            DOWN => Position(self.0 + 1, self.1),
            LEFT => Position(self.0, self.1 - 1),
            RIGHT => Position(self.0, self.1 + 1),
        }
    }
}

static NEIGHBOR_DIRECTIONS: [Direction; 4] = [UP, DOWN, LEFT, RIGHT];

struct GridMap {
    path_lengths: Vec<Vec<i32>>,
    matrix: Vec<Vec<i32>>,
}

impl GridMap {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        Self {
            path_lengths: vec![vec![-1; matrix[0].len()]; matrix.len()],
            matrix,
        }
    }

    fn get(&self, Position(i, j): Position) -> Option<i32> {
        Some(*self.matrix.get(i)?.get(j)?)
    }

    fn util(&mut self, pos: Position) -> i32 {
        if self.path_lengths[pos.0][pos.1] != -1 {
            return self.path_lengths[pos.0][pos.1];
        }

        let node_value = self.get(pos).unwrap();
        let mut neighbor_longest_path = 0;

        for neighbor_pos in NEIGHBOR_DIRECTIONS.iter().map(|&direction| pos + direction) {
            neighbor_longest_path = neighbor_longest_path.max(match self.get(neighbor_pos) {
                Some(neighbor_value) if neighbor_value > node_value => self.util(neighbor_pos),
                _ => 0,
            });
        }

        self.path_lengths[pos.0][pos.1] = neighbor_longest_path + 1;
        self.path_lengths[pos.0][pos.1]
    }
}

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let mut grid_map = GridMap::new(matrix);

        let mut longest_path = -1;
        for i in 0..grid_map.matrix.len() {
            for j in 0..grid_map.matrix[0].len() {
                longest_path = longest_path.max(grid_map.util(Position(i, j)));
            }
        }
        longest_path
    }
}
