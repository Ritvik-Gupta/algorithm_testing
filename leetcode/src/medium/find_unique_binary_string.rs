crate::solution!();

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let mut result = String::with_capacity(nums[0].len());

        let mut iterating_idx = 0;
        for binary_ptr in nums.iter().map(|binary| binary.as_ptr()) {
            let token = char::from(unsafe { *binary_ptr.wrapping_offset(iterating_idx) });
            result.push(if token == '0' { '1' } else { '0' });
            iterating_idx += 1;
        }

        result
    }
}
