crate::leetcode::solution!();

impl Solution {
    fn checked_reverse(mut num: i32) -> Option<i32> {
        let is_negative = if num < 0 {
            num *= -1;
            true
        } else {
            false
        };

        let mut reversed_num: i32 = 0;
        while num > 0 {
            reversed_num = reversed_num
                .checked_mul(10)
                .and_then(|rev| rev.checked_add(num % 10))?;
            num /= 10;
        }

        if is_negative {
            reversed_num *= -1;
        }
        Some(reversed_num)
    }

    pub fn reverse(num: i32) -> i32 {
        Solution::checked_reverse(num).unwrap_or(0)
    }
}
