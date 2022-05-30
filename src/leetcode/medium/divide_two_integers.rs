crate::leetcode::solution!();

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == i32::MIN && divisor == -1 {
            return i32::MAX;
        }

        let (dvd, dvs) = (dividend.abs() as f64, divisor.abs() as f64);

        let quotient = f64::exp2(dvd.log2() - dvs.log2()).floor() as i32;

        if dividend.is_positive() == divisor.is_positive() {
            quotient
        } else {
            -quotient
        }
    }
}
