crate::solution!();

use std::collections::HashMap;

impl Solution {
    pub fn num_splits(word: String) -> i32 {
        let mut right_freq_table = {
            let mut freq_table = HashMap::with_capacity(26);
            word.chars()
                .for_each(|ch| *freq_table.entry(ch).or_insert(0) += 1);
            freq_table
        };

        let mut left_freq_table = HashMap::with_capacity(26);

        word.chars()
            .filter(|&ch| {
                let freq = right_freq_table.get_mut(&ch).unwrap();
                *freq -= 1;
                if *freq == 0 {
                    right_freq_table.remove(&ch);
                }
                *left_freq_table.entry(ch).or_insert(0) += 1;

                left_freq_table.len() == right_freq_table.len()
            })
            .count() as i32
    }
}
