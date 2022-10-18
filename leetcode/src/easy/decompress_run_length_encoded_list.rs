crate::solution!();

impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        (0..nums.len())
            .step_by(2)
            .flat_map(|i| std::iter::repeat(nums[i + 1]).take(nums[i] as usize))
            .collect()
    }
}
