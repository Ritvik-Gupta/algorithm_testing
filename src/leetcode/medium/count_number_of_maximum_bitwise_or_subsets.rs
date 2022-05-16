crate::leetcode::solution!();

impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let or_of_nums = nums.iter().fold(0, |acc, &num| acc | num);

        let mut subset_count = 1;
        let mut bit_offset = 1;
        while bit_offset <= or_of_nums {
            let mut count_with_bit = 0;
            for &num in nums.iter() {
                if num & bit_offset != 0 {
                    count_with_bit += 1;
                }
            }

            if count_with_bit != 0 {
                subset_count *= count_with_bit;
            }
            bit_offset <<= 1;
        }
        subset_count
    }
}
