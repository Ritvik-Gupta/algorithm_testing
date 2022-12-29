crate::solution!();

impl Solution {
    pub fn sum_base(n: i32, k: i32) -> i32 {
        match n {
            0 => 0,
            _ => n % k + Solution::sum_base(n / k, k),
        }
    }
}
