crate::solution!();

fn count_bits(mut n: i32) -> i32 {
    (0..32)
        .map(|_| {
            let bit = n & 1;
            n >>= 1;
            bit
        })
        .sum()
}

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        count_bits(x ^ y)
    }
}
