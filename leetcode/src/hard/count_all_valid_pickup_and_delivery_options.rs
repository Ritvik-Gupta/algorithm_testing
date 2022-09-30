crate::solution!();

const MODULE_FOR_RESULT: u64 = 1000000007;
const ADDER_INCREMENT: u64 = 4;

impl Solution {
    pub fn count_orders(n: i32) -> i32 {
        let (mut adder, mut multiplier) = (5u64, 1u64);
        (1..n).fold(1u64, |acc, _| {
            multiplier += adder;
            adder += ADDER_INCREMENT;
            (acc * multiplier) % MODULE_FOR_RESULT
        }) as i32
    }
}
