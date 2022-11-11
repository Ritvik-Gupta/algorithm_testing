crate::solution!();

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut left_ptr = 0;

        for ptr in 0..nums.len() {
            if nums[ptr] != val {
                nums[left_ptr] = nums[ptr];
                left_ptr += 1;
            }
        }

        left_ptr as i32
    }
}
