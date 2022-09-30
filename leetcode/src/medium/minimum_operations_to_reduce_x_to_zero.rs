crate::solution!();

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let target = nums.iter().sum::<i32>() - x;

        let (mut curr_sum, mut max_len) = (0, 0);

        let mut start_idx = 0;
        let mut found_subarray = false;
        for end_idx in 0..nums.len() {
            curr_sum += nums[end_idx];
            while start_idx <= end_idx && curr_sum > target {
                curr_sum -= nums[start_idx];
                start_idx += 1;
            }

            if curr_sum == target {
                found_subarray = true;
                max_len = usize::max(max_len, end_idx - start_idx + 1);
            }
        }

        if !found_subarray {
            return -1;
        }

        (nums.len() - max_len) as i32
    }
}
