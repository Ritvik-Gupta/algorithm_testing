crate::solution!();

fn backtrack_for_unused_combinations(
    nums: &Vec<i32>,
    result: &mut Vec<Vec<i32>>,
    permutation: &mut Vec<i32>,
    used_nums: &mut Vec<bool>,
) {
    if permutation.len() == nums.len() {
        result.push(permutation.clone());
        return;
    }

    for i in 0..nums.len() {
        if used_nums[i] || (i > 0 && nums[i] == nums[i - 1] && !used_nums[i - 1]) {
            continue;
        }

        permutation.push(nums[i]);
        used_nums[i] = true;
        backtrack_for_unused_combinations(nums, result, permutation, used_nums);
        used_nums[i] = false;
        permutation.pop();
    }
}

impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut result = Vec::with_capacity(nums.len());
        backtrack_for_unused_combinations(
            &nums,
            &mut result,
            &mut Vec::with_capacity(nums.len()),
            &mut vec![false; nums.len()],
        );
        result
    }
}
