crate::leetcode::solution!();

use std::collections::HashMap;

fn construct_freq_table(words: Vec<String>) -> HashMap<String, usize> {
    let mut freq_table = HashMap::new();
    words
        .into_iter()
        .for_each(|word| *freq_table.entry(word).or_insert(0) += 1);
    freq_table
}

impl Solution {
    pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
        let freq_table1 = construct_freq_table(words1);
        let freq_table2 = construct_freq_table(words2);

        freq_table1
            .into_iter()
            .filter(|(key, freq)| *freq == 1 && *freq_table2.get(key).unwrap_or(&0) == 1)
            .count() as i32
    }
}
