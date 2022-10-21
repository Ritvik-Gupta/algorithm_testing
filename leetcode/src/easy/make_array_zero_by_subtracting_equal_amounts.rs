crate::solution!();

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .filter(|&num| num != 0)
            .collect::<std::collections::HashSet<_>>()
            .len() as i32
    }
}
