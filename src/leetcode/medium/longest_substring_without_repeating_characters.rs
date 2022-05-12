crate::leetcode::solution!();

use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn length_of_longest_substring(word: String) -> i32 {
        let mut substr = VecDeque::new();
        let mut seen_chars = HashSet::new();

        let mut max_found_substr_len = 0;
        for ch in word.chars() {
            if seen_chars.contains(&ch) {
                while let Some(substr_front) = substr.pop_front() {
                    seen_chars.remove(&substr_front);
                    if substr_front == ch {
                        break;
                    }
                }
            }

            substr.push_back(ch);
            seen_chars.insert(ch);
            max_found_substr_len = max_found_substr_len.max(substr.len() as i32);
        }

        max_found_substr_len
    }
}
