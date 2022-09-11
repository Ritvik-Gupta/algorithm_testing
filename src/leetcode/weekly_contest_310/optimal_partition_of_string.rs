crate::leetcode::solution!();

use std::collections::HashSet;

impl Solution {
    pub fn partition_string(word: String) -> i32 {
        let mut seen_chars = HashSet::new();
        let mut total_partitions = 1;

        word.chars().for_each(|ch| {
            if seen_chars.contains(&ch) {
                seen_chars.clear();
                total_partitions += 1;
            }

            seen_chars.insert(ch);
        });

        total_partitions
    }
}
