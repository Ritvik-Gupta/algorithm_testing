crate::solution!();

use std::cmp::max;

fn cumulative_max<'a>(iter: impl Iterator<Item = &'a i32>) -> i32 {
    iter.scan(0, |cumulative, &elm| {
        *cumulative += elm;
        Some(*cumulative)
    })
    .max()
    .unwrap()
}

fn helper(nums: &Vec<i32>, lower_idx: usize, upper_idx: usize) -> i32 {
    if lower_idx + 1 == upper_idx {
        return nums[lower_idx];
    }

    let mid_idx = (lower_idx + upper_idx) / 2;
    let (left_to_mid, right_to_mid) = (
        cumulative_max(nums[lower_idx..mid_idx].iter().rev()),
        cumulative_max(nums[mid_idx..upper_idx].iter()),
    );
    return max(
        max(
            helper(nums, lower_idx, mid_idx),
            helper(nums, mid_idx, upper_idx),
        ),
        left_to_mid + right_to_mid,
    );
}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        return helper(&nums, 0, nums.len());
    }
}
