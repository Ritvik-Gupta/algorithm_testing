crate::solution!();

impl Solution {
    pub fn percentage_letter(word: String, letter: char) -> i32 {
        ((word.chars().filter(|&ch| ch == letter).count() * 100) / word.len()) as i32
    }
}
