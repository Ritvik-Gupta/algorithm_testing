crate::leetcode::solution!();

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == i32::MIN && divisor == -1 {
            return i32::MAX;
        }

        let (mut dvd, dvs) = (dividend.abs(), divisor.abs());
        let mut quotient = 0;

        for bit in (0..32).rev() {
            if (dvd >> bit) - dvs >= 0 {
                quotient += 1 << bit;
                dvd -= dvs << bit;
            }
        }

        if dividend.is_positive() == divisor.is_positive() {
            quotient
        } else {
            -quotient
        }
    }
}
