crate::leetcode::solution!();

impl Solution {
    pub fn rearrange_array(mut nums: Vec<i32>) -> Vec<i32> {
        let mut arr = vec![0; nums.len()];
        let (mut i, mut j) = (0, 0);

        for k in 0..nums.len() {
            if nums[k] > 0 {
                arr[i] = nums[k];
                i += 2;
            } else {
                arr[j] = nums[k];
                j += 2;
            }
        }
        arr
    }
}
