crate::solution!();

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        use std::convert::TryFrom;
        let num = match u32::try_from(n) {
            Ok(0) => return false,
            Ok(num) => num,
            Err(_) => return false,
        };

        num & (num - 1) == 0
    }
}
