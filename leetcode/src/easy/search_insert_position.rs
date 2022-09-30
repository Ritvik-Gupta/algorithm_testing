crate::solution!();

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        use std::cmp::Ordering::*;

        let (mut lower_idx, mut upper_idx) = (0i32, nums.len() as i32 - 1);
        while lower_idx <= upper_idx {
            let mid_idx = lower_idx + (upper_idx - lower_idx) / 2;
            match target.cmp(&nums[mid_idx as usize]) {
                Less => upper_idx = mid_idx - 1,
                Equal => return mid_idx,
                Greater => lower_idx = mid_idx + 1,
            }
        }
        lower_idx
    }
}
