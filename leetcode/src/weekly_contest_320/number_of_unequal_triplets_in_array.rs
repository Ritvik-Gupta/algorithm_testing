crate::solution!();

use std::collections::HashMap;

impl Solution {
    pub fn unequal_triplets(nums: Vec<i32>) -> i32 {
        let mut freq_table = HashMap::new();
        nums.iter()
            .for_each(|&num| *freq_table.entry(num).or_insert(0) += 1);

        let (mut res, mut left, mut right) = (0, 0, nums.len() as i32);
        freq_table.values().for_each(|&cnt| {
            right -= cnt;
            res += left * cnt * right;
            left += cnt;
        });

        res
    }
}
