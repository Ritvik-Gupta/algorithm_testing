crate::solution!();

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut last = nums.len() - 1;

        for i in (0..nums.len() - 1).rev() {
            if i + nums[i] as usize >= last {
                last = i;
            }
        }
        last <= 0
    }
}
