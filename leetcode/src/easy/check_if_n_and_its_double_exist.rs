crate::solution!();

use std::collections::HashSet;

impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut seen_nums = HashSet::new();

        arr.into_iter().any(|num| {
            if seen_nums.contains(&(num * 2)) || (num & 1 == 0 && seen_nums.contains(&(num / 2))) {
                return true;
            }
            seen_nums.insert(num);
            false
        })
    }
}
