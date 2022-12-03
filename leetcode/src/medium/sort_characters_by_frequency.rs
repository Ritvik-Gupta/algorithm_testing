crate::solution!();

use std::collections::HashMap;

impl Solution {
    pub fn frequency_sort(word: String) -> String {
        let mut freq_table = HashMap::new();
        word.chars()
            .for_each(|ch| *freq_table.entry(ch).or_insert(0) += 1);

        let mut buckets = vec!["".to_string(); word.len() + 1];

        freq_table
            .iter()
            .for_each(|(&ch, &freq)| buckets[freq].push_str(&ch.to_string().repeat(freq)));

        buckets
            .iter()
            .rev()
            .flat_map(|bucket| bucket.chars())
            .collect()
    }
}
