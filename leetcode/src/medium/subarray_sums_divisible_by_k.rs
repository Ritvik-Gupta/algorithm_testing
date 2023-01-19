crate::solution!();

impl Solution {
    pub fn subarrays_div_by_k(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut cumulative = 0;
        nums.iter_mut().for_each(|num| {
            cumulative = ((cumulative + *num) % k + k) % k;
            *num = cumulative;
        });

        let mut subarr_count = 0;
        let mut freq_table = vec![0; k as usize];
        freq_table[0] = 1;

        nums.iter().for_each(|&num| {
            let freq = &mut freq_table[num as usize];
            subarr_count += *freq;
            *freq += 1;
        });

        subarr_count
    }
}
