crate::leetcode::solution!();

impl Solution {
    fn closest_power_of_2(mut num: u32) -> u32 {
        num |= num >> 1;
        num |= num >> 2;
        num |= num >> 4;
        num |= num >> 8;
        num |= num >> 16;

        num += 1;
        return num >> 1;
    }

    pub fn find_complement(num: i32) -> i32 {
        let num = num as u32;
        let all_ones = (Solution::closest_power_of_2(num) << 1) - 1;

        (!num & all_ones) as i32
    }
}
