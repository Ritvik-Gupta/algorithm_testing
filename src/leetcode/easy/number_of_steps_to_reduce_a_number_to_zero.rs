crate::leetcode::solution!();

impl Solution {
    pub fn number_of_steps(mut num: i32) -> i32 {
        if num == 0 {
            return 0;
        }

        let mut total_steps = 1;
        while num > 1 {
            total_steps += (num & 1) + 1;
            num >>= 1;
        }
        total_steps
    }
}
