use std::collections::HashMap;

const BOARD_SIZE: usize = 5;

#[derive(Debug)]
struct BingoBoard {
    row_strengths: [usize; BOARD_SIZE],
    col_strengths: [usize; BOARD_SIZE],
    location_map: HashMap<usize, (usize, usize)>,
    is_finished: bool,
}

impl BingoBoard {
    fn new() -> Self {
        Self {
            row_strengths: [0; BOARD_SIZE],
            col_strengths: [0; BOARD_SIZE],
            location_map: HashMap::with_capacity(BOARD_SIZE * BOARD_SIZE),
            is_finished: false,
        }
    }
}

#[derive(Debug)]
pub struct Bingo {
    numbers_drawn: Vec<usize>,
    boards: Vec<BingoBoard>,
}

impl Bingo {
    pub fn parse(mut lines: impl Iterator<Item = String>) -> Self {
        let numbers_drawn = lines.next().expect("has all numbers drawn list");
        let mut boards = Vec::new();
        let mut lines = lines.peekable();

        while lines.peek().is_some() {
            lines.next().expect("is an empty line");

            let mut board = BingoBoard::new();
            (0..BOARD_SIZE).for_each(|row_idx| {
                lines
                    .next()
                    .expect("has a row for board")
                    .split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|num| num.parse().expect("is a number"))
                    .enumerate()
                    .for_each(|(col_idx, num)| {
                        board.row_strengths[row_idx] += 1;
                        board.col_strengths[col_idx] += 1;
                        board.location_map.insert(num, (row_idx, col_idx));
                    });
            });

            boards.push(board);
        }

        Self {
            numbers_drawn: numbers_drawn
                .split(',')
                .map(|num| num.parse().expect("is a number"))
                .collect(),
            boards,
        }
    }

    pub fn first_winning_board(&mut self) -> u64 {
        let (mut unmarked_sum, mut board_winning_number) = (0, 0);

        'drawing_numbers: for &number_drawn in self.numbers_drawn.iter() {
            for board in self.boards.iter_mut() {
                if let Some((row_idx, col_idx)) = board.location_map.remove(&number_drawn) {
                    board.row_strengths[row_idx] -= 1;
                    board.col_strengths[col_idx] -= 1;

                    if board.row_strengths[row_idx] == 0 || board.col_strengths[col_idx] == 0 {
                        unmarked_sum = board.location_map.keys().map(|&x| x).sum::<usize>() as u64;
                        board_winning_number = number_drawn as u64;
                        break 'drawing_numbers;
                    }
                }
            }
        }

        unmarked_sum * board_winning_number
    }

    pub fn last_winning_board(&mut self) -> u64 {
        let (mut unmarked_sum, mut board_winning_number) = (0, 0);
        let mut left_boards = self.boards.len();

        for &number_drawn in self.numbers_drawn.iter() {
            for board in self.boards.iter_mut().filter(|board| !board.is_finished) {
                if let Some((row_idx, col_idx)) = board.location_map.remove(&number_drawn) {
                    board.row_strengths[row_idx] -= 1;
                    board.col_strengths[col_idx] -= 1;

                    if board.row_strengths[row_idx] == 0 || board.col_strengths[col_idx] == 0 {
                        board.is_finished = true;
                        left_boards -= 1;
                        if left_boards == 0 {
                            unmarked_sum =
                                board.location_map.keys().map(|&x| x).sum::<usize>() as u64;
                            board_winning_number = number_drawn as u64;
                        }
                    }
                }
            }
        }

        unmarked_sum * board_winning_number
    }
}
