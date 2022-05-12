crate::leetcode::solution!();

fn compute_subsets(nums_left: &[i32], combination: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    result.push(combination.clone());

    for idx in 0..nums_left.len() {
        combination.push(nums_left[idx]);
        compute_subsets(&nums_left[idx + 1..], combination, result);
        combination.pop();
    }
}

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        compute_subsets(&nums, &mut Vec::with_capacity(nums.len()), &mut result);
        result
    }
}
