pub struct Solution;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Vector(usize, usize);

impl Vector {
    fn contains(&self, other: &Self) -> bool {
        other.0 < self.0 && other.1 < self.1
    }
}

impl std::ops::Add<&Direction> for Vector {
    type Output = Vector;

    fn add(self, direction: &Direction) -> Self::Output {
        match direction {
            UP => Vector(self.0 + 1, self.1),
            DOWN => Vector(self.0 - 1, self.1),
            LEFT => Vector(self.0, self.1 + 1),
            RIGHT => Vector(self.0, self.1 - 1),
        }
    }
}

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

use Direction::{DOWN, LEFT, RIGHT, UP};

static NEIGHBOUR_DIRECTIONS: [Direction; 4] = [UP, DOWN, LEFT, RIGHT];

struct DfsBoard<'a> {
    board: Vec<Vec<char>>,
    board_size: Vector,
    word: &'a [u8],
}

impl<'a> std::ops::Index<Vector> for DfsBoard<'a> {
    type Output = char;

    fn index(&self, location: Vector) -> &Self::Output {
        &self.board[location.0][location.1]
    }
}

impl<'a> std::ops::IndexMut<Vector> for DfsBoard<'a> {
    fn index_mut(&mut self, location: Vector) -> &mut Self::Output {
        &mut self.board[location.0][location.1]
    }
}

const SEEN_TILE: char = '#';

impl<'a> DfsBoard<'a> {
    fn new(board: Vec<Vec<char>>, word: &'a [u8]) -> Self {
        let board_size = Vector(board.len(), board[0].len());
        DfsBoard {
            board,
            board_size,
            word,
        }
    }

    fn from_word(&self, index: usize) -> char {
        self.word[index] as char
    }

    fn dfs(&mut self, location: Vector, word_ptr: usize) -> bool {
        if word_ptr == self.word.len() {
            return true;
        }
        if !self.board_size.contains(&location) || self[location] != self.from_word(word_ptr) {
            return false;
        }

        self[location] = SEEN_TILE;
        for neighbour in NEIGHBOUR_DIRECTIONS.iter().map(|dir| location + dir) {
            if self.dfs(neighbour, word_ptr + 1) {
                return true;
            }
        }
        self[location] = self.from_word(word_ptr);
        false
    }
}

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut dfs_board = DfsBoard::new(board, word.as_bytes());

        if word.len() > dfs_board.board_size.0 * dfs_board.board_size.1 {
            return false;
        }

        for i in 0..dfs_board.board_size.0 {
            for j in 0..dfs_board.board_size.1 {
                if dfs_board.dfs(Vector(i, j), 0) {
                    return true;
                }
            }
        }
        false
    }
}
