crate::leetcode::solution!();

impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let window_size = k as usize;

        let (mut left, mut right) = (0, arr.len() - window_size);
        while left < right {
            let mid_idx = (left + right) / 2;
            match x - arr[mid_idx] > arr[mid_idx + window_size] - x {
                true => left = mid_idx + 1,
                false => right = mid_idx,
            }
        }

        arr[left..left + window_size].into()
    }
}
