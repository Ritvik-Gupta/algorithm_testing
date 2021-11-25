crate::leetcode::solution!();

const IMPOSSIBLE_ELEMENT: i32 = -1;

struct SortedArray<'a>(&'a Vec<i32>);

impl<'a> SortedArray<'a> {
    fn get(&self, idx: usize) -> i32 {
        *self.0.get(idx).unwrap_or(&IMPOSSIBLE_ELEMENT)
    }
}

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        let nums = SortedArray(&nums);
        let (mut lower_idx, mut upper_idx) = (0, nums.0.len() - 1);
        while lower_idx <= upper_idx {
            let mid_idx = lower_idx + (upper_idx - lower_idx) / 2;
            let elm_at_mid = nums.get(mid_idx);

            if elm_at_mid == nums.get(mid_idx + 1) {
                match mid_idx % 2 {
                    0 => lower_idx = mid_idx + 2,
                    _ => upper_idx = mid_idx - 1,
                }
            } else if elm_at_mid == nums.get(mid_idx - 1) {
                match mid_idx % 2 {
                    0 => upper_idx = mid_idx - 2,
                    _ => lower_idx = mid_idx + 1,
                }
            } else {
                return elm_at_mid;
            }
        }
        unreachable!();
    }
}
