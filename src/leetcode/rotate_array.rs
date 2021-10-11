pub struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        use std::collections::VecDeque;

        let nums_size = nums.len();
        let k = (k as usize) % nums_size;

        let mut loop_back_queue = VecDeque::with_capacity(k);
        for i in (0..=nums_size - 1).rev() {
            if i >= nums_size - k {
                loop_back_queue.push_back(nums[i]);
            }
            nums[i] = if i < k {
                loop_back_queue.pop_front().unwrap()
            } else {
                nums[i - k]
            };
        }
    }
}
