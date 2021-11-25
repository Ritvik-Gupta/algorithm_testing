crate::leetcode::solution!();

impl Solution {
    fn most_significant_bit_num(mut num: u32) -> u32 {
        num |= num >> 1;
        num |= num >> 2;
        num |= num >> 4;
        num |= num >> 8;
        num |= num >> 16;

        num += 1;
        return num >> 1;
    }

    pub fn path_in_zig_zag_tree(label: i32) -> Vec<i32> {
        let mut label = label as u32;
        let mut result = Vec::new();
        let mut most_sig_bit = Solution::most_significant_bit_num(label);
        while label > 0 {
            result.push(label as i32);

            let all_one_bits = (most_sig_bit << 1) - 1;
            label = (!label & all_one_bits) | most_sig_bit;
            label >>= 1;
            most_sig_bit >>= 1;
        }
        result.into_iter().rev().collect()
    }
}
