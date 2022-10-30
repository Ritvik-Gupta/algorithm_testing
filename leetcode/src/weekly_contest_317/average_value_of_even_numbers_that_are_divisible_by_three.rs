crate::solution!();

impl Solution {
    pub fn average_value(nums: Vec<i32>) -> i32 {
        let (sum, num_elms) = nums
            .into_iter()
            .filter(|&num| num % 6 == 0)
            .fold((0, 0), |(sum, num_elms), num| (sum + num, num_elms + 1));

        sum.checked_div(num_elms).unwrap_or(0)
    }
}
