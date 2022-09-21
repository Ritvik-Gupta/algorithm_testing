crate::leetcode::solution!();

fn is_even(val: &i32) -> bool {
    *val & 1 == 0
}

impl Solution {
    pub fn sum_even_after_queries(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut even_sum = nums.iter().map(|&val| val).filter(is_even).sum();

        queries
            .iter()
            .map(|query| {
                let (val, index) = (query[0], query[1] as usize);

                if is_even(&nums[index]) {
                    even_sum -= nums[index];
                }
                nums[index] += val;
                if is_even(&nums[index]) {
                    even_sum += nums[index];
                }

                even_sum
            })
            .collect()
    }
}
