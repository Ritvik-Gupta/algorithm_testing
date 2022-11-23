crate::solution!();

use std::collections::HashSet;

impl Solution {
    pub fn appeal_sum(word: String) -> i64 {
        let word = word.as_bytes();
        let mut seen_tokens = HashSet::with_capacity(26);
        let mut total_sum = 0;

        for i in 0..word.len() {
            seen_tokens.clear();
            for j in i..word.len() {
                seen_tokens.insert(word[j]);
                total_sum += seen_tokens.len() as i64;
            }
        }
        total_sum
    }
}
