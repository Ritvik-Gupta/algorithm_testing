crate::solution!();

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn unique_occurrences(nums: Vec<i32>) -> bool {
        let mut freq_table = HashMap::new();
        nums.iter()
            .for_each(|&num| *freq_table.entry(num).or_insert(0) += 1);

        let mut unique_freqs = HashSet::new();
        freq_table.values().all(|freq| unique_freqs.insert(freq))
    }
}
