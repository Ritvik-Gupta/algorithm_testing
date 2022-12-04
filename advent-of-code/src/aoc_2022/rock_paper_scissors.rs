use std::cmp::Ordering::{self, *};
use Move::*;

#[derive(PartialEq, Clone, Copy)]
enum Move {
    Rock = 0,
    Paper = 1,
    Scissor = 2,
}

impl From<u8> for Move {
    fn from(played_move: u8) -> Self {
        match played_move {
            b'A' | b'X' => Rock,
            b'B' | b'Y' => Paper,
            b'C' | b'Z' => Scissor,
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match (self, other) {
            (Rock, Scissor) => Greater,
            (Scissor, Rock) => Less,
            _ => (*self as u8).cmp(&(*other as u8)),
        })
    }
}

pub struct RockPaperScissors;

impl crate::AdventDayProblem for RockPaperScissors {
    type Arg = Vec<(u8, u8)>;

    fn get_problem_name() -> &'static str {
        crate::problem_name!()
    }

    fn construct_arg(dataset: impl Iterator<Item = String>) -> Self::Arg {
        dataset
            .map(|line| {
                let moves = line.as_bytes();
                (moves[0], moves[2])
            })
            .collect()
    }

    fn part_1(games: Self::Arg) -> i128 {
        games
            .iter()
            .map(|moves| (Move::from(moves.0), Move::from(moves.1)))
            .map(|(opponent_move, player_move)| {
                (1 + player_move as i128)
                    + match player_move.partial_cmp(&opponent_move).unwrap() {
                        Greater => 6,
                        Equal => 3,
                        Less => 0,
                    }
            })
            .sum()
    }

    fn part_2(games: Self::Arg) -> i128 {
        games
            .iter()
            .map(|&(opponent_move, result)| {
                let opponent_move = Move::from(opponent_move);
                let player_move = Move::from(
                    match result {
                        b'X' => (opponent_move as u8 + 3 - 1) % 3,
                        b'Y' => opponent_move as u8,
                        b'Z' => (opponent_move as u8 + 1) % 3,
                        _ => unreachable!(),
                    } + b'A',
                );

                (1 + player_move as i128)
                    + match player_move.partial_cmp(&opponent_move).unwrap() {
                        Greater => 6,
                        Equal => 3,
                        Less => 0,
                    }
            })
            .sum()
    }
}
