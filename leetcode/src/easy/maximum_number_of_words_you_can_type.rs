crate::solution!();

use std::collections::HashSet;

impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let broken_letters = broken_letters.chars().collect::<HashSet<_>>();
        text.split(' ')
            .filter(|word| word.chars().all(|ch| !broken_letters.contains(&ch)))
            .count() as i32
    }
}
