crate::leetcode::solution!();

impl Solution {
    #[allow(non_snake_case)]
    pub fn hammingWeight(mut num: u32) -> i32 {
        let mut result = 0;
        while num != 0 {
            result += num & 1;
            num >>= 1;
        }
        result as i32
    }
}
