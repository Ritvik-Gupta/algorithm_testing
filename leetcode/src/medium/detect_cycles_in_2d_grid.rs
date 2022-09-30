crate::solution!();

#[derive(Clone, Copy)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

use Direction::*;

#[derive(Clone, Copy, PartialEq, Eq)]
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
    m: usize,
    n: usize,
    grid: Vec<Vec<char>>,
    is_visited: Vec<Vec<bool>>,
}

impl GridMap {
    fn new(grid: Vec<Vec<char>>) -> Self {
        let (m, n) = (grid.len(), grid[0].len());
        Self {
            m,
            n,
            grid,
            is_visited: vec![vec![false; n]; m],
        }
    }

    fn get(&self, Position(i, j): Position) -> Option<char> {
        Some(*self.grid.get(i)?.get(j)?)
    }

    fn dfs_to_detect_cycle(&mut self, pos: Position, prev_pos: Position) -> bool {
        if self.is_visited[pos.0][pos.1] {
            return true;
        }
        self.is_visited[pos.0][pos.1] = true;

        NEIGHBOR_DIRECTIONS
            .iter()
            .map(|&direction| pos + direction)
            .any(|next_pos| match self.get(next_pos) {
                Some(next_token)
                    if next_pos != prev_pos && self.grid[pos.0][pos.1] == next_token =>
                {
                    self.dfs_to_detect_cycle(next_pos, pos)
                }
                _ => false,
            })
    }
}

impl Solution {
    pub fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
        let mut grid = GridMap::new(grid);

        for x in 0..grid.m {
            for y in 0..grid.n {
                if grid.is_visited[x][y] {
                    continue;
                }

                if grid.dfs_to_detect_cycle(Position(x, y), Position(x, y)) {
                    return true;
                }
            }
        }
        false
    }
}
