crate::solution!();

use std::collections::HashMap;

fn fill_freq_table<'a>(
    iter: impl Iterator<Item = &'a str>,
    freq_table: &mut HashMap<String, usize>,
) {
    iter.for_each(|word| *freq_table.entry(word.to_string()).or_insert(0) += 1);
}

impl Solution {
    pub fn uncommon_from_sentences(sentence_1: String, sentence_2: String) -> Vec<String> {
        let mut freq_table = HashMap::new();
        fill_freq_table(sentence_1.split(' '), &mut freq_table);
        fill_freq_table(sentence_2.split(' '), &mut freq_table);

        freq_table
            .into_iter()
            .filter(|&(_, freq)| freq == 1)
            .map(|(word, _)| word)
            .collect()
    }
}
