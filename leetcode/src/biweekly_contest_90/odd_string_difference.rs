crate::solution!();

use std::collections::HashMap;

fn word_to_diff_array(word: &str) -> Vec<i32> {
    word.chars()
        .scan('-', |prev_ch, ch| {
            let diff = i32::from(ch as u8) - i32::from(*prev_ch as u8);
            *prev_ch = ch;
            Some(diff)
        })
        .skip(1)
        .collect()
}

impl Solution {
    pub fn odd_string(words: Vec<String>) -> String {
        let mut seen_diff_arrs = HashMap::with_capacity(2);

        words.iter().for_each(|word| {
            let diff_array = word_to_diff_array(word);
            seen_diff_arrs
                .entry(diff_array)
                .or_insert(Vec::new())
                .push(word);
        });

        seen_diff_arrs
            .into_iter()
            .find(|(_, freq)| freq.len() == 1)
            .map(|(_, word)| word[0].clone())
            .unwrap()
    }
}
