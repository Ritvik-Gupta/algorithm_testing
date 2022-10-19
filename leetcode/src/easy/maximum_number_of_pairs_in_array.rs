crate::solution!();

use std::collections::HashMap;

impl Solution {
    pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
        let mut freq_table = HashMap::new();
        nums.into_iter()
            .for_each(|num| *freq_table.entry(num).or_insert(0) += 1);

        let (num_pairs, left_nums) = freq_table
            .values()
            .fold((0, 0), |(np, ln), freq| (np + (freq >> 1), ln + (freq & 1)));

        vec![num_pairs, left_nums]
    }
}
