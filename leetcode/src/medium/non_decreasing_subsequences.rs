crate::solution!();

use std::collections::HashSet;

fn recur_sequence(mono_stk: &mut Vec<i32>, index: usize, nums: &Vec<i32>, res: &mut Vec<Vec<i32>>) {
    if mono_stk.len() > 1 {
        res.push(mono_stk.clone());
    }

    let mut used_nums = HashSet::new();
    for i in index..nums.len() {
        if used_nums.contains(&nums[i]) || *mono_stk.last().unwrap_or(&-101) > nums[i] {
            continue;
        }

        used_nums.insert(nums[i]);
        mono_stk.push(nums[i]);
        recur_sequence(mono_stk, i + 1, nums, res);
        mono_stk.pop();
    }
}

impl Solution {
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        recur_sequence(&mut Vec::new(), 0, &nums, &mut res);
        res
    }
}
