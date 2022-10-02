crate::solution!();

impl Solution {
    pub fn common_factors(a: i32, b: i32) -> i32 {
        let mut factors = 0;
        for num in 1..=i32::min(a, b) {
            if a % num == 0 && b % num == 0 {
                factors += 1;
            }
        }
        factors
    }
}
