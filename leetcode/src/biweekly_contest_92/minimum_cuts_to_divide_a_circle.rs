crate::solution!();

impl Solution {
    pub fn number_of_cuts(n: i32) -> i32 {
        if n == 1 {
            return 0;
        }
        n / if n & 1 == 0 { 2 } else { 1 }
    }
}
