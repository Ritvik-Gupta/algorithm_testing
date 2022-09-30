crate::solution!();

fn recursive_combination_sum(
    candidates: &Vec<i32>,
    target: i32,
    result: &mut Vec<Vec<i32>>,
    combination: &mut Vec<i32>,
    start_candidate_idx: usize,
) {
    if target == 0 {
        result.push(combination.clone());
        return;
    }

    for idx in start_candidate_idx..candidates.len() {
        let candidate = candidates[idx];
        if target < candidate {
            break;
        }
        if idx > start_candidate_idx && candidates[idx - 1] == candidate {
            continue;
        }

        combination.push(candidate);
        recursive_combination_sum(candidates, target - candidate, result, combination, idx + 1);
        combination.pop();
    }
}

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();
        let mut result = Vec::new();
        recursive_combination_sum(&candidates, target, &mut result, &mut Vec::new(), 0);
        result
    }
}
