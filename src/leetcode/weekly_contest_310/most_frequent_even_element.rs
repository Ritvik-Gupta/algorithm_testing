crate::leetcode::solution!();

use std::collections::HashMap;

impl Solution {
    pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
        let mut freq_store = HashMap::new();
        let mut most_freq_elm = -1;
        freq_store.insert(most_freq_elm, 0);

        nums.iter().filter(|&&num| num & 1 == 0).for_each(|&num| {
            *freq_store.entry(num).or_insert(0) += 1;

            let num_freq = freq_store[&num];
            if num_freq > freq_store[&most_freq_elm]
                || (num_freq == freq_store[&most_freq_elm] && num < most_freq_elm)
            {
                most_freq_elm = num;
            }
        });

        most_freq_elm
    }
}
