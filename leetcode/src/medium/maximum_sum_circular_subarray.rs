crate::solution!();

fn kadane_max_sum(nums: &Vec<i32>) -> i32 {
    let mut current_sum = nums[0];
    let mut max_sum = nums[0];

    for i in 1..nums.len() {
        current_sum = nums[i] + i32::max(current_sum, 0);
        max_sum = i32::max(max_sum, current_sum);
    }
    max_sum
}

impl Solution {
    pub fn max_subarray_sum_circular(mut nums: Vec<i32>) -> i32 {
        let non_circular_sum = kadane_max_sum(&nums);
        let total_sum = nums
            .iter_mut()
            .map(|num| {
                *num *= -1;
                -*num
            })
            .sum::<i32>();

        let circular_sum = total_sum + kadane_max_sum(&nums);
        match circular_sum {
            0 => non_circular_sum,
            _ => i32::max(circular_sum, non_circular_sum),
        }
    }
}
