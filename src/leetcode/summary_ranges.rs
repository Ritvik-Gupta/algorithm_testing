pub struct Solution;

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let nums_size = nums.len();
        let mut result = Vec::new();
        if nums_size == 0 {
            return result;
        }

        let mut range_start = 0;
        for pos in 1..=nums_size {
            if pos == nums_size || nums[pos] != nums[pos - 1] + 1 {
                result.push(match range_start == pos - 1 {
                    true => nums[range_start].to_string(),
                    false => format!("{}->{}", nums[range_start], nums[pos - 1]),
                });
                range_start = pos;
            }
        }

        result
    }
}
