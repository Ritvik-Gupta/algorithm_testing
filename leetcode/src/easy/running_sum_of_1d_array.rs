crate::solution!();

impl Solution {
    pub fn running_sum(mut nums: Vec<i32>) -> Vec<i32> {
        let mut sum = 0;
        nums.iter_mut().for_each(|num| {
            sum += *num;
            *num = sum;
        });
        nums
    }
}
