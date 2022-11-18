crate::solution!();

impl Solution {
    pub fn is_ugly(mut num: i32) -> bool {
        if num <= 0 {
            return false;
        }

        for prime_factor in [2, 3, 5] {
            while num % prime_factor == 0 {
                num /= prime_factor;
            }
        }
        num == 1
    }
}
