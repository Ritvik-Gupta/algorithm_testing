crate::leetcode::solution!();

impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        (n.chars().max().unwrap() as u8 - b'0') as i32
    }
}
