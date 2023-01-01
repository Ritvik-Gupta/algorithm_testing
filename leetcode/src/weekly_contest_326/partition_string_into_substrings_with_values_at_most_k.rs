crate::solution!();

impl Solution {
    pub fn minimum_partition(s: String, k: i32) -> i32 {
        let k = k as i64;
        let mut num_partition_substrs = 0;
        let mut num_buffer = 0;

        for digit in s.chars().map(|digit| (digit as u8 - b'0') as i64) {
            if num_buffer > k {
                return -1;
            }
            num_buffer = num_buffer * 10 + digit;

            if num_buffer > k {
                num_partition_substrs += 1;
                num_buffer = digit
            }
        }
        num_partition_substrs + 1
    }
}
