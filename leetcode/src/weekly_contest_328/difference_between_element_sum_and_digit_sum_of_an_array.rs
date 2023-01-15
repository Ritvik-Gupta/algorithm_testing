crate::solution!();

fn digit_sum(mut num: i32) -> i32 {
    let mut digit_sum = 0;
    while num > 0 {
        digit_sum += num % 10;
        num /= 10;
    }
    digit_sum
}

impl Solution {
    pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
        i32::abs(nums.iter().sum::<i32>() - nums.into_iter().map(digit_sum).sum::<i32>())
    }
}
