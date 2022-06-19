crate::leetcode::solution!();

impl Solution {
    pub fn minimum_numbers(num: i32, k: i32) -> i32 {
        if num == 0 {
            return 0;
        }

        let unit_digit = num % 10;
        let mut x = k;
        for i in 1..=10 {
            if unit_digit == x && k * i <= num {
                return i;
            }
            x += k;
            x %= 10;
        }
        -1
    }
}
