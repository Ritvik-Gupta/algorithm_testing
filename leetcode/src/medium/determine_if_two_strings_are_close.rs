crate::solution!();

use std::collections::HashSet;

fn ch_to_idx(ch: char) -> usize {
    (ch as u8 - b'a') as usize
}

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }
        if word1.chars().collect::<HashSet<_>>() != word2.chars().collect::<HashSet<_>>() {
            return false;
        }

        let (mut freq_table1, mut freq_table2) = ([0; 26], [0; 26]);

        word1.chars().zip(word2.chars()).for_each(|(ch1, ch2)| {
            freq_table1[ch_to_idx(ch1)] += 1;
            freq_table2[ch_to_idx(ch2)] += 1;
        });

        freq_table1.sort();
        freq_table2.sort();

        freq_table1 == freq_table2
    }
}
