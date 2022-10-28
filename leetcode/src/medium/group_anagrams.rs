crate::solution!();

use std::collections::HashMap;

fn frequency_hash(word: &str) -> [u8; 26] {
    let mut freq_table = [0; 26];
    word.chars()
        .map(|ch| (ch as u8 - b'a') as usize)
        .for_each(|token| freq_table[token] += 1);
    freq_table
}

impl Solution {
    pub fn group_anagrams(words: Vec<String>) -> Vec<Vec<String>> {
        let mut anagrams_table = HashMap::new();

        words.into_iter().for_each(|word| {
            anagrams_table
                .entry(frequency_hash(&word))
                .or_insert(Vec::new())
                .push(word);
        });

        anagrams_table.into_values().collect()
    }
}
