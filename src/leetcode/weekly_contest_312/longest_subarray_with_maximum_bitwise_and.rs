crate::leetcode::solution!();

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let max_elm = *nums.iter().max().unwrap();
        let (mut continous_max_elms, mut sliding_window) = (0, 0);

        for num in nums {
            sliding_window = if num == max_elm {
                sliding_window + 1
            } else {
                0
            };

            continous_max_elms = i32::max(continous_max_elms, sliding_window);
        }

        continous_max_elms
    }
}
