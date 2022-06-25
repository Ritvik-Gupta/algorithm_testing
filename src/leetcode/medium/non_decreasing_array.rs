crate::leetcode::solution!();

impl Solution {
    pub fn check_possibility(mut nums: Vec<i32>) -> bool {
        let mut cnt = 0;

        for i in 1..nums.len() {
            if nums[i] < nums[i - 1] {
                cnt += 1;
                if cnt > 1 {
                    return false;
                }

                if i == 1 || nums[i - 2] <= nums[i] {
                    nums[i - 1] = nums[i];
                } else {
                    nums[i] = nums[i - 1];
                }
            }
        }
        true
    }
}
