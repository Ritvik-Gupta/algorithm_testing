pub struct Solution;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Location(usize, usize);

impl Location {
    fn contains(&self, other: &Self) -> bool {
        other.0 < self.0 && other.1 < self.1
    }
}

impl std::ops::Add<&Direction> for Location {
    type Output = Location;

    fn add(self, direction: &Direction) -> Self::Output {
        match direction {
            UP => Location(self.0 + 1, self.1),
            DOWN => Location(self.0 - 1, self.1),
            LEFT => Location(self.0, self.1 + 1),
            RIGHT => Location(self.0, self.1 - 1),
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
    board_size: Location,
    word: &'a [u8],
}

impl<'a> std::ops::Index<Location> for DfsBoard<'a> {
    type Output = char;

    fn index(&self, location: Location) -> &Self::Output {
        &self.board[location.0][location.1]
    }
}

impl<'a> std::ops::IndexMut<Location> for DfsBoard<'a> {
    fn index_mut(&mut self, location: Location) -> &mut Self::Output {
        &mut self.board[location.0][location.1]
    }
}

const TILE_MARK_OFFSET: u8 = 60;

impl<'a> DfsBoard<'a> {
    fn new(board: Vec<Vec<char>>, word: &'a [u8]) -> Self {
        let board_size = Location(board.len(), board[0].len());
        DfsBoard {
            board,
            board_size,
            word,
        }
    }

    fn from_word(&self, index: usize) -> char {
        self.word[index] as char
    }

    fn mark_tile(&mut self, location: Location) {
        self[location] = (self[location] as u8 + TILE_MARK_OFFSET) as char;
    }

    fn unmark_tile(&mut self, location: Location) {
        self[location] = (self[location] as u8 - TILE_MARK_OFFSET) as char;
    }

    fn dfs(&mut self, location: Location, word_ptr: usize) -> bool {
        if word_ptr == self.word.len() {
            return true;
        }

        self.mark_tile(location);
        for neighbour in NEIGHBOUR_DIRECTIONS.iter().map(|dir| location + dir) {
            if self.board_size.contains(&neighbour)
                && self[neighbour] == self.from_word(word_ptr)
                && self.dfs(neighbour, word_ptr + 1)
            {
                return true;
            }
        }
        self.unmark_tile(location);
        false
    }
}

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut dfs_board = DfsBoard::new(board, word.as_bytes());

        for i in 0..dfs_board.board_size.0 {
            for j in 0..dfs_board.board_size.1 {
                if dfs_board[Location(i, j)] == dfs_board.from_word(0)
                    && dfs_board.dfs(Location(i, j), 1)
                {
                    return true;
                }
            }
        }
        false
    }
}
