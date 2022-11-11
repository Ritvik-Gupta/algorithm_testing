crate::solution!();

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut left_ptr = 0;

        for ptr in 0..nums.len() {
            if nums[ptr] != 0 {
                nums[left_ptr] = nums[ptr];
                left_ptr += 1;
            }
        }

        (left_ptr..nums.len()).for_each(|ptr| nums[ptr] = 0)
    }
}
