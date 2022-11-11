crate::solution!();

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut unq_ptr = 1;

        for ptr in 1..nums.len() {
            if nums[ptr] != nums[ptr - 1] {
                nums[unq_ptr] = nums[ptr];
                unq_ptr += 1;
            }
        }

        unq_ptr as i32
    }
}
