crate::leetcode::solution!();

use std::collections::VecDeque;

impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        if nums.len() < 3 {
            return false;
        }

        let mut cumulative_min = i32::MAX;
        let mut min_before_num: Vec<_> = Vec::with_capacity(nums.len());
        nums.iter().for_each(|&elm| {
            cumulative_min = i32::min(cumulative_min, elm);
            min_before_num.push(cumulative_min);
        });

        let mut possible_stack = VecDeque::with_capacity(nums.len() / 5);

        for j in (0..nums.len()).rev() {
            if nums[j] <= min_before_num[j] {
                continue;
            }

            while *possible_stack.back().unwrap_or(&i32::MAX) <= min_before_num[j] {
                possible_stack.pop_back();
            }

            if *possible_stack.back().unwrap_or(&i32::MAX) < nums[j] {
                return true;
            }
            possible_stack.push_back(nums[j]);
        }
        false
    }
}
