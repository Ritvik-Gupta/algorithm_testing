crate::solution!();

impl Solution {
    pub fn total_hamming_distance(mut nums: Vec<i32>) -> i32 {
        let mut base_table = [0; 2];
        (0..u32::BITS)
            .map(|_| {
                base_table[0] = 0;
                base_table[1] = 0;
                nums.iter_mut().for_each(|num| {
                    base_table[(*num & 1) as usize] += 1;
                    *num >>= 1;
                });
                base_table[0] * base_table[1]
            })
            .sum()
    }
}
