crate::solution!();

impl Solution {
    pub fn count_odds(mut low: i32, high: i32) -> i32 {
        if low & 1 == 0 {
            low += 1;
        }

        if low > high {
            0
        } else {
            (high - low) / 2 + 1
        }
    }
}
