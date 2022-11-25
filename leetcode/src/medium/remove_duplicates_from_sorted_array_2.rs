crate::solution!();

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut num_context = 0;
        let mut j = 1;

        for i in 1..nums.len() {
            if (nums[i] == nums[i - 1] && num_context == 0) || nums[i] != nums[i - 1] {
                num_context = if nums[i] != nums[i - 1] { 0 } else { 1 };

                nums[j] = nums[i];
                j += 1;
            }
        }
        j as i32
    }
}
