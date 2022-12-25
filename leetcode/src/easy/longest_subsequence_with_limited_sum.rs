crate::solution!();

impl Solution {
    pub fn answer_queries(mut nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        nums.sort();
        let n = nums.len();

        for i in 1..n {
            nums[i] += nums[i - 1];
        }

        queries
            .iter()
            .map(|query| nums.binary_search(query).map_or_else(|i| i, |i| i + 1) as i32)
            .collect()
    }
}
