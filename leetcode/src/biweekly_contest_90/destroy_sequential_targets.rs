crate::solution!();

use std::{cmp::Reverse, collections::HashMap};

impl Solution {
    pub fn destroy_targets(nums: Vec<i32>, space: i32) -> i32 {
        let mut target_groups = HashMap::new();
        nums.iter().for_each(|&num| {
            target_groups
                .entry(num % space)
                .or_insert(Vec::new())
                .push(num);
        });

        target_groups
            .into_iter()
            .map(|(_, elms)| (*elms.iter().min().unwrap(), elms))
            .max_by_key(|(target_id, elms)| (elms.len(), Reverse(*target_id)))
            .unwrap()
            .0
    }
}
