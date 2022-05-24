crate::leetcode::solution!();

use std::str::Chars;

fn solve(str: &mut Chars) -> String {
    let mut result = String::new();

    let mut repetition = 0;
    while let Some(token) = str.next() {
        if token == ']' {
            break;
        }

        if let Some(digit) = token.to_digit(10) {
            repetition = repetition * 10 + digit;
        } else if token.is_alphabetic() {
            result.push(token);
        } else if token == '[' {
            result.push_str(&solve(str).repeat(repetition as usize));
            repetition = 0;
        }
    }
    result
}

impl Solution {
    pub fn decode_string(str: String) -> String {
        solve(&mut str.chars())
    }
}
