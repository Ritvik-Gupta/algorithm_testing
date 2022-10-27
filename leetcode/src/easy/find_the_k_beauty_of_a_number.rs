crate::solution!();

impl Solution {
    pub fn divisor_substrings(mut num: i32, k: i32) -> i32 {
        let kth_power = 10i32.pow(k as u32 - 1);

        let dividend = num;
        let mut divisor = num % (kth_power * 10);
        num /= kth_power;

        let mut total_divisor_substrs = 0;
        while num > 0 {
            num /= 10;
            if divisor != 0 && dividend % divisor == 0 {
                total_divisor_substrs += 1;
            }
            divisor = (divisor / 10) + (num % 10) * kth_power;
        }
        total_divisor_substrs
    }
}
