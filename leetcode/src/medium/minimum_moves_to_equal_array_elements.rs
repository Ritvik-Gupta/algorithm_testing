crate::solution!();

impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        Some(nums.iter().fold((0, i32::MAX), |(sum, min), &num| {
            (sum + num, i32::min(min, num))
        }))
        .map_or(0, |(sum, min)| sum - min * nums.len() as i32)
    }
}
