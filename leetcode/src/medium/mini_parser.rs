crate::solution!();

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

use std::iter::Peekable;

fn is_integer_component(token: char) -> bool {
    token.is_ascii_digit() || token == '-'
}

fn parse_int(sentence: &mut Peekable<impl Iterator<Item = char>>) -> NestedInteger {
    let mut res = 0;
    let mut sign_multiplier = 1;

    while let Some(&token) = sentence.peek() {
        if !is_integer_component(token) {
            break;
        }
        match token {
            '-' => sign_multiplier = -1,
            _ => res = res * 10 + (token as u8 - b'0') as i32,
        }
        sentence.next();
    }

    NestedInteger::Int(res * sign_multiplier)
}

fn parse_nested_level(sentence: &mut Peekable<impl Iterator<Item = char>>) -> NestedInteger {
    sentence.next();
    let mut list = Vec::new();

    while let Some(token) = sentence.peek() {
        match token {
            '[' => list.push(parse_nested_level(sentence)),
            ',' => {
                sentence.next();
            }
            ']' => {
                sentence.next();
                break;
            }
            _ => list.push(parse_int(sentence)),
        }
    }
    NestedInteger::List(list)
}

impl Solution {
    pub fn deserialize(sentence: String) -> NestedInteger {
        let mut sentence = sentence.chars().peekable();

        let x = String::new();

        match is_integer_component(*sentence.peek().unwrap()) {
            true => parse_int(&mut sentence),
            false => parse_nested_level(&mut sentence),
        }
    }
}
