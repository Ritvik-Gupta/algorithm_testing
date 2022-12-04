use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(PartialEq, Eq, Clone, Copy)]
enum Delimeter {
    ROUND,
    SQAURE,
    CURLY,
    POINTY,
}

use Delimeter::*;

impl Delimeter {
    fn from_char(token: char) -> Self {
        match token {
            '(' | ')' => ROUND,
            '[' | ']' => SQAURE,
            '{' | '}' => CURLY,
            '<' | '>' => POINTY,
            _ => unreachable!(),
        }
    }

    fn is_starting(token: char) -> bool {
        ['(', '[', '{', '<'].contains(&token)
    }

    fn illegal_points(&self) -> u64 {
        match self {
            ROUND => 3,
            SQAURE => 57,
            CURLY => 1197,
            POINTY => 25137,
        }
    }

    fn incomplete_points(&self) -> u64 {
        match self {
            ROUND => 1,
            SQAURE => 2,
            CURLY => 3,
            POINTY => 4,
        }
    }
}

fn score_syntax_for_illegal_chunks(chunks: impl Iterator<Item = String>) -> u64 {
    let mut delimeter_stack = Vec::new();
    chunks
        .map(|chunk| {
            delimeter_stack.clear();

            for token in chunk.chars() {
                let delimeter = Delimeter::from_char(token);

                if Delimeter::is_starting(token) {
                    match delimeter_stack.pop() {
                        Some(last_token) => {
                            if last_token != delimeter {
                                return delimeter.illegal_points();
                            }
                        }
                        None => return delimeter.illegal_points(),
                    }
                } else {
                    delimeter_stack.push(delimeter);
                }
            }

            0
        })
        .sum()
}

fn middle_score_of_incomplete_chunks(chunks: impl Iterator<Item = String>) -> u64 {
    let mut scores: Vec<u64> = chunks
        .filter_map(|chunk| {
            let mut delimeter_stack = Vec::new();

            for token in chunk.chars() {
                let delimeter = Delimeter::from_char(token);

                if !Delimeter::is_starting(token) {
                    match delimeter_stack.pop() {
                        Some(last_token) => {
                            if last_token != delimeter {
                                return None;
                            }
                        }
                        None => return None,
                    }
                } else {
                    delimeter_stack.push(delimeter);
                }
            }

            Some(
                delimeter_stack
                    .iter()
                    .rev()
                    .fold(0, |acc, delimeter| acc * 5 + delimeter.incomplete_points()),
            )
        })
        .collect();

    scores.sort();
    scores[scores.len() / 2]
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./files/syntax_scoring.txt")?;
    let result = score_syntax_for_illegal_chunks(
        BufReader::new(&file)
            .lines()
            .map(|line| line.expect("is a valid line")),
    );
    println!("{}", result);

    let file = File::open("./files/syntax_scoring.txt")?;
    let result = middle_score_of_incomplete_chunks(
        BufReader::new(&file)
            .lines()
            .map(|line| line.expect("is a valid line")),
    );
    println!("{}", result);

    Ok(())
}
