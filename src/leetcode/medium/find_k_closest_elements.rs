pub struct Solution;

impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let window_size = k as usize;

        let (mut left_idx, mut right_idx) = (0, arr.len() - window_size);
        while left_idx < right_idx {
            let mid_idx = (left_idx + right_idx) / 2;
            match x - arr[mid_idx] > arr[mid_idx + window_size] - x {
                true => left_idx = mid_idx + 1,
                false => right_idx = mid_idx,
            }
        }

        arr[left_idx..left_idx + window_size].into()
    }
}
