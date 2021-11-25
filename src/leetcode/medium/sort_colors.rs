crate::leetcode::solution!();

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut left, mut mid, mut right) = (0, 0, nums.len() - 1);
        while mid <= right {
            if nums[mid] == 0 {
                Solution::swap(nums, left, mid);
                left += 1;
                mid += 1;
            } else if nums[mid] == 1 {
                mid += 1;
            } else {
                Solution::swap(nums, right, mid);
                if right == 0 {
                    break;
                }
                right -= 1;
            }
        }
    }

    fn swap(nums: &mut Vec<i32>, index_a: usize, index_b: usize) {
        let temp = nums[index_a];
        nums[index_a] = nums[index_b];
        nums[index_b] = temp;
    }
}
