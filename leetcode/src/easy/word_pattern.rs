crate::solution!();

use std::collections::hash_map::Entry::*;
use std::collections::HashMap;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        if pattern.len() != s.split(' ').count() {
            return false;
        }

        let mut letter_bijection_table = [None; 26];
        let mut word_bijection_table = HashMap::<&str, char>::new();

        for (letter, word) in pattern.chars().zip(s.split(' ')) {
            let letter_bucket = &mut letter_bijection_table[(letter as u8 - b'a') as usize];
            let word_bucket = word_bijection_table.entry(word);

            match (&letter_bucket, word_bucket) {
                (None, Vacant(entry)) => {
                    *letter_bucket = Some(word);
                    entry.insert(letter);
                }
                (Some(_), Vacant(_)) => return false,
                (None, Occupied(_)) => return false,
                (Some(stored_word), Occupied(stored_letter))
                    if *stored_word != word || *stored_letter.get() != letter =>
                {
                    return false;
                }
                _ => {}
            }
        }
        true
    }
}
