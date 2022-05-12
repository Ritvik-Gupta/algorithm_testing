crate::leetcode::solution!();

impl Solution {
    pub fn reverse_bits(num: u32) -> u32 {
        let mut reversed_num = 0;
        let mut bit_offset = 1;
        for _ in 0..32 {
            reversed_num <<= 1;
            if (num & bit_offset) != 0 {
                reversed_num |= 1;
            }
            bit_offset <<= 1;
        }
        reversed_num
    }
}
