crate::solution!();

impl Solution {
    pub fn trap(heights: Vec<i32>) -> i32 {
        let (mut ordered_stack, mut water_trapped, mut idx) = (Vec::new(), 0, 0);

        while idx < heights.len() {
            match ordered_stack.last() {
                Some(&pivot) if heights[idx] > heights[pivot] => {
                    ordered_stack.pop();
                    if let Some(&prev_pivot) = ordered_stack.last() {
                        let min_height = std::cmp::min(heights[prev_pivot], heights[idx]);
                        water_trapped +=
                            (min_height - heights[pivot]) * (idx - prev_pivot - 1) as i32;
                    }
                }
                _ => {
                    ordered_stack.push(idx);
                    idx += 1;
                }
            }
        }
        water_trapped
    }
}
