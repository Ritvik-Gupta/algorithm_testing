crate::solution!();

impl Solution {
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut freq_table = [0; 101];
        nums.iter().for_each(|&num| freq_table[num as usize] += 1);

        (k + 1..101)
            .map(|num| freq_table[num] * freq_table[num - k])
            .sum()
    }
}
