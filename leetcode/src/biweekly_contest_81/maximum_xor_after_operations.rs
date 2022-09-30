crate::solution!();

impl Solution {
    pub fn maximum_xor(nums: Vec<i32>) -> i32 {
        let mut result = 0;

        nums.iter().for_each(|&num| {
            let mut bit_offset = 1;
            while num >= bit_offset {
                if num & bit_offset != 0 {
                    result |= bit_offset;
                }
                bit_offset <<= 1;
            }
        });

        result
    }
}
