pub struct Solution;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        use std::collections::BTreeMap;

        let max_separation = k as usize;
        let mut prev_found_nums = BTreeMap::new();

        for pos in 0..nums.len() {
            match prev_found_nums.insert(nums[pos], pos) {
                Some(prev_pos) if pos <= prev_pos + max_separation => return true,
                _ => {}
            }
        }
        false
    }
}
