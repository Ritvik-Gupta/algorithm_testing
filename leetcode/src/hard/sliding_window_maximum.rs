crate::solution!();

use std::collections::VecDeque;

struct SlidingWindow<'a> {
    nums: &'a Vec<i32>,
    window_size: usize,
    window: VecDeque<usize>,
}

impl<'a> SlidingWindow<'a> {
    fn new(nums: &'a Vec<i32>, window_size: usize) -> Self {
        Self {
            nums,
            window_size,
            window: VecDeque::with_capacity(window_size + 1),
        }
    }

    fn has_extended_window(&self, idx: usize) -> bool {
        self.window
            .front()
            .map_or(false, |&front_idx| front_idx + self.window_size <= idx)
    }

    fn has_unwanted_indices(&self, elm: i32) -> bool {
        self.window
            .back()
            .map_or(false, |&back_idx| self.nums[back_idx] < elm)
    }
}

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut max_sliding = SlidingWindow::new(&nums, k as usize);
        let mut result = Vec::with_capacity(nums.len() - max_sliding.window_size + 1);

        (0..nums.len()).for_each(|idx| {
            if max_sliding.has_extended_window(idx) {
                max_sliding.window.pop_front();
            }

            while max_sliding.has_unwanted_indices(nums[idx]) {
                max_sliding.window.pop_back();
            }

            max_sliding.window.push_back(idx);
            if idx + 1 >= max_sliding.window_size {
                result.push(nums[max_sliding.window[0]]);
            }
        });
        result
    }
}
