crate::solution!();

use std::cmp::{max, min};

impl Solution {
    pub fn min_max_game(mut nums: Vec<i32>) -> i32 {
        let mut total_nums = nums.len();

        while total_nums > 1 {
            let mut is_min_opr = true;
            for i in (0..total_nums).step_by(2) {
                nums[i / 2] = if is_min_opr {
                    min(nums[i], nums[i + 1])
                } else {
                    max(nums[i], nums[i + 1])
                };
                is_min_opr = !is_min_opr;
            }

            total_nums >>= 1;
        }

        nums[0]
    }
}
