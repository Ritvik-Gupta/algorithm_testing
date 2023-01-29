crate::solution!();

impl Solution {
    pub fn distinct_integers(n: i32) -> i32 {
        i32::max(1, n - 1)
    }
}
