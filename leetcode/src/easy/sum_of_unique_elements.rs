crate::solution!();

impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut freq_store = [0; 101];
        nums.iter().for_each(|&num| freq_store[num as usize] += 1);

        freq_store
            .iter()
            .enumerate()
            .filter(|(_, &freq)| freq == 1)
            .map(|(num, _)| num as i32)
            .sum()
    }
}
