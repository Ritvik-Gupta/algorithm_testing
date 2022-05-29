crate::leetcode::solution!();

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        nums.iter()
            .map(|&num| num as usize)
            .enumerate()
            .fold(nums.len(), |acc, (idx, num)| acc ^ idx ^ num) as i32
    }
}
