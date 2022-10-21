crate::solution!();

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        use std::collections::HashMap;

        let max_separation = k as usize;
        let mut prev_found_nums = HashMap::with_capacity(max_separation);

        nums.iter()
            .enumerate()
            .any(|(idx, &num)| match prev_found_nums.insert(num, idx) {
                Some(prev_idx) if idx <= prev_idx + max_separation => true,
                _ => false,
            })
    }
}
