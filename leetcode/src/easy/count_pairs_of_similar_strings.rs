crate::solution!();

use std::collections::HashMap;

fn construct_freq_table(word: &str) -> u32 {
    let mut freq_table = 0;
    word.chars()
        .for_each(|ch| freq_table |= 1 << (ch as u8 - b'a'));
    freq_table
}

impl Solution {
    pub fn similar_pairs(words: Vec<String>) -> i32 {
        let mut seen_anagrams = HashMap::new();

        words
            .iter()
            .for_each(|word| *seen_anagrams.entry(construct_freq_table(word)).or_insert(0) += 1);

        seen_anagrams
            .values()
            .map(|&freq| freq * (freq - 1) / 2)
            .sum()
    }
}
