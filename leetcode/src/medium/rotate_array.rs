crate::solution!();

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let nums_size = nums.len();
        let k = (k as usize) % nums_size;

        nums[0..nums_size].reverse();
        nums[0..k].reverse();
        nums[k..nums_size].reverse();
    }
}
