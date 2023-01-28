crate::solution!();

impl Solution {
    pub fn alternate_digit_sum(mut n: i32) -> i32 {
        let (mut sum, mut sign_flag, mut digit_count) = (0, 1, 0);
        while n > 0 {
            sum += sign_flag * n % 10;
            n /= 10;
            sign_flag *= -1;
            digit_count += 1;
        }
        if digit_count % 2 == 0 {
            sum *= -1;
        }
        sum
    }
}
